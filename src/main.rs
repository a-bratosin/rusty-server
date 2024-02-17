use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread::{self, Thread},
    time::Duration,
};
use std::process::exit;
use rusty_server::ThreadPool;

fn main() {
    
    let listener = TcpListener::bind("127.0.0.1:7878");
    match listener{
        Err(_) => {
            println!("Could not connect to the port!");
            exit(1);
        },
        Ok(_)=>{println!("Connected to port!")},
    }

    let pool = ThreadPool::new(4);

    for stream in listener.unwrap().incoming(){
        let stream = stream.unwrap();

     //   println!("Connection established! :D");
        pool.execute(|| {
            handle_connection(stream);
        });
    }
    println!("Hello, world!");
}

fn handle_connection(mut stream: TcpStream){
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    


    let (status_line, filename) = match &request_line[..]{
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "media/hello.html"),
        "GET /secret HTTP/1.1" => ("HTTP/1.1 200 OK", "media/secret.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "media/hello.html")
        },
        "GET /silly.jpeg HTTP/1.1" => ("HTTP/1.1 200 OK", "media/silly.jpeg"),
        _ => ("HTTP/1.1 404 NOT FOUND", "media/404.html"),
    };

    let contents = fs::read(filename).unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n");

    stream.write_all(response.as_bytes()).unwrap();    
    stream.write_all(&contents).unwrap();
}
