
use std::{net::{SocketAddr, TcpListener, TcpStream}, thread, sync::mpsc, str::FromStr, io::{Read, Write, stdin}};

use crate::package::{Pack, Cmd};


pub fn server(){
    println!("Hello, world!");
    let addr: SocketAddr = SocketAddr::from_str("127.0.0.1:4000").unwrap();
    let listener = TcpListener::bind(addr).unwrap();
    let (send,recv) = mpsc::channel();
    thread::spawn(move || {
        for stream in listener.incoming(){
            let send = send.clone();
            let stream = stream.unwrap();
            println!("Connection established!");
            thread::spawn(||{
                handle_connection(stream,send);
            });
        };
    });
    for info in recv {
        println!("{}:{}","someone",info);
    }
}

fn handle_connection (mut stream:TcpStream,send:mpsc::Sender<String>){
    loop{
        match read_pack(&mut stream).into(){
            Cmd::Nop => continue,
            Cmd::Msg(info) => {
                send.send(info).unwrap();
                let response = "HTTP/1.1 200 OK\r\n\r\n";
                stream.write(response.as_bytes()).unwrap();
                stream.flush().unwrap();
            },
            Cmd::Quit => break,
        }
    }
}
fn read_pack(stream:&mut TcpStream)->Pack{
    let mut buffer = [0 as u8;1024];
    stream.read(&mut buffer).unwrap();
    println!("@@@ buffer len: {}",buffer.len());
    buffer.as_slice().into()
}


pub fn client(){
    let mut tcp=TcpStream::connect("127.0.0.1:4000").unwrap();
    println!("Connect Success");
    loop {
        let mut line = String::new();
        stdin().read_line(&mut line).unwrap();
        let v:Vec<u8> =Cmd::Msg(line).into();
        tcp.write(v.as_slice()).unwrap();
    }
}
