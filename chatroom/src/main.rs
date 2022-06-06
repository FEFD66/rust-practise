use std::{net::{SocketAddr, TcpListener, TcpStream}, str::FromStr, sync::mpsc, io::{Read, Write}};

fn main() {
    println!("Hello, world!");
    let addr: SocketAddr = SocketAddr::from_str("127.0.0.1:4000").unwrap();
    let listener = TcpListener::bind(addr).unwrap();
    //let (send,recv) = mpsc::channel();

    for stream in listener.incoming(){
        let stream = stream.unwrap();
        println!("Connection established!");
        handle_connection(stream);
    }
}

fn handle_connection (mut stream:TcpStream){
    let mut buffer = [0;1024];
    stream.read(&mut buffer).unwrap();

    let response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
