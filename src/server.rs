use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs;

pub struct Route {
    pub route: String,
    pub file: String,
}

impl Route{
    pub fn new(method: String, route: String, file: String) -> Route{
        let route = Route {
            route: format!("{} {} HTTP", method.to_uppercase(), route),
            file: file
        };

        route
    }
}

pub fn listen(routes: Vec<Route>, port: String) {
    let address = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(address).unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(result) => handle_connection(result, &routes),
            Err(e) => panic!("{}", e)
        };
    }
}

fn handle_connection(mut stream: TcpStream, routes: &Vec<Route>) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let iter = routes.iter();

    for route in iter {
        if buffer.starts_with(route.route.as_bytes()) {
            send(&route, stream);
            break;
        }
    }
}
 fn send(route: &Route, mut stream: TcpStream) {
    let status = "HTTP/1.1 200 OK\r\n\r\n";
    let file = fs::read_to_string(&route.file).unwrap();

    let response = format!("{}{}", status, file);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
 }