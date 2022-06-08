
mod package;
mod webserver;

use std::{env, io::{self, Read}};

use webserver::server;

use crate::webserver::client;


fn main(){
    println!("Hello world!");
    let args:Vec<String> = env::args().collect();
    match args[1].as_str() {
        "server"=> server(),
        "client"=>client(),
        "slice"=>array_slice(),
        _=>todo!("Invalid operation"),
    }
    println!("{:?}",args);
    //server();
}

fn array_slice(){
    let mut x=[0;10];
    io::stdin().read_exact(&mut x[0..5]).unwrap();
    println!("{:?}",x);
    io::stdin().read_exact(&mut x[5..10]).unwrap();
    println!("{:?}",x);
}
