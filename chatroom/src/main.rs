
mod package;
mod webserver;

use std::env;

use webserver::server;

use crate::webserver::client;


fn main(){
    println!("Hello world!");
    let args:Vec<String> = env::args().collect();
    match args[1].as_str() {
        "server"=> server(),
        "client"=>client(),
        _=>(),
    }
    println!("{:?}",args);
    //server();
}
