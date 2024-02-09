use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1").unwrap();

    for stream in listener.incoming(){
        let stream = stream.unwrap();

        println!("Connection established! :D");
    }
    println!("Hello, world!");
}
