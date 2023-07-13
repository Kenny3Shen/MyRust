use std::net::TcpListener;
use std::io::{Read, Write};
fn main() {
    let listener = TcpListener::bind("localhost:8080").unwrap();
    println!("Running on port 8080...");
    //let result = listener.accept().unwrap(); //only listen once

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        println!("Connection established!");
        let mut buffer = [0; 1024];

        stream.read(&mut buffer).unwrap();
        stream.write(&mut buffer).unwrap();
    }
    
}
