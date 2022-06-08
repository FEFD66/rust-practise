
use std::{net::{SocketAddr, TcpListener, TcpStream}, thread, sync::mpsc, str::FromStr, io::{Read, Write, stdin}, time::Duration};

use crate::package::{Pack, Cmd, PackReader};


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

fn handle_connection (stream:TcpStream,send:mpsc::Sender<String>){
    let mut reader=PackReader::new(stream);
    let mut str = [0;1024];
    //reader.stream.set_read_timeout(Some(Duration::from_secs(1))).unwrap();

    reader.stream.read(&mut str).unwrap();
    eprintln!("@@@{}",String::from_utf8_lossy(&str));
    loop{
        match reader.read_pack().into(){
            Cmd::Nop => continue,
            Cmd::Msg(info) => {
                send.send(info).unwrap();
            },
            Cmd::Quit => break,
        }
    }
}


pub fn client(){
    let tcp=TcpStream::connect("127.0.0.1:4000").unwrap();
    let mut pr=PackReader::new(tcp);
    println!("Connect Success");
    pr.stream.write(String::from("Yes, It is!").as_bytes()).unwrap();
    loop {
        let mut line = String::new();
        stdin().read_line(&mut line).unwrap();
        pr.write_pack(Cmd::Msg(line).into());
    }
}
