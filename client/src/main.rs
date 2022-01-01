use std::net::TcpStream;
use std::io::{Write, Read};
use std::str::from_utf8;

fn main() {
    let mut stream = TcpStream::connect("localhost:3000").unwrap();
    stream.write("hello world".as_bytes()).unwrap();

    let mut buffer = [0; 11];
    stream.read(&mut buffer).unwrap();

    println!("response:{:?}", from_utf8(&buffer).unwrap());
}
