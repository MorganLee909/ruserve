use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs;

use std::any::type_name;
fn _type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(thing) => handle_connection(thing),
            Err(e) => panic!("{}", e)
        };
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    // println!("{}", String::from_utf8_lossy(&buffer[..]));

    let home = b"GET / HTTP";
    let homecss = b"GET /style";

    let (status, file) = if buffer.starts_with(home) {
        (
            "HTTP/1.1 200 OK\r\n\r\n",
            "index.html"
        )
    } else if buffer.starts_with(homecss) {
        (
            "HTTP/1.1 200 OK\r\n\r\n",
            "index.css"
        )
    } else {
        (
            "HTTP/1.1 404 NOT FOUND\r\n\r\n",
            "404.html"
        )
    };

    let file = fs::read_to_string(file).unwrap();
    let response = format!("{}{}", status, file);
    println!("{}", response);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}