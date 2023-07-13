use std::net::TcpStream;
use std::io::{Read, Write};
fn main() {
    let mut stream = TcpStream::connect("localhost:8080").unwrap();
    stream.write("Hello".as_bytes()).unwrap();

    let mut buffer = vec![0;5];
    stream.read(&mut buffer).unwrap();

    println!("Response from server:{:?}", String::from_utf8(buffer).unwrap());
}
