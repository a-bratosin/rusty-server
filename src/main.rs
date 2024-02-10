use std::net::TcpListener;
use std::process::exit;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:80");
    match listener{
        Err(_) => {
            println!("Could not connect to the port!");
            exit(1);
        },
        Ok(_)=>{println!("Connected to port!")},
    }


    for stream in listener.unwrap().incoming(){
        let stream = stream.unwrap();

        println!("Connection established! :D");
    }
    println!("Hello, world!");
}
