use std::{
    net::{TcpListener, TcpStream},
    io::{prelude::*, BufReader}
};
use std::process::exit;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878");
    match listener{
        Err(_) => {
            println!("Could not connect to the port!");
            exit(1);
        },
        Ok(_)=>{println!("Connected to port!")},
    }


    for stream in listener.unwrap().incoming(){
        let stream = stream.unwrap();

     //   println!("Connection established! :D");
        handle_connection(stream);
    }
    println!("Hello, world!");
}

fn handle_connection(mut stream: TcpStream){
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {:#?}", http_request);
}
