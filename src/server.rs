use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs;
use std::any::type_name;

fn _type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

pub struct Route<'a> {
    pub method: &'a str,
    pub url: &'a str,
    pub file: &'a str
}

struct RouteCollection<'a> {
    get: Vec<Route<'a>>,
    post: Vec<Route<'a>>,
    put: Vec<Route<'a>>,
    delete: Vec<Route<'a>>
}

pub fn listen(routes: Vec<Route>, port: String) {
    let address = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(address).unwrap();

    let mut route_collection = RouteCollection {
        get: vec![],
        post: vec![],
        put: vec![],
        delete: vec![]
    };

    for route in routes {
        let lower = &route.method.to_lowercase()[..];
        match lower {
            "get" => route_collection.get.push(route),
            "post"=> route_collection.post.push(route),
            "put" => route_collection.put.push(route),
            "delete" => route_collection.delete.push(route),
            _ => panic!("{} is not a method", route.method)
        };
    };

    for stream in listener.incoming() {
        match stream {
            Ok(result) => handle_connection(result, &route_collection),
            Err(e) => panic!("{}", e)
        };
    }
}

fn handle_connection(mut stream: TcpStream, route_collection: &RouteCollection) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    println!("{}", String::from_utf8_lossy(&buffer[..]));

    let request_string = String::from_utf8_lossy(&buffer[..]);
    let mut iter = request_string.split_whitespace();
    let method = &iter.next().unwrap().to_lowercase()[..];

    if method == "get" {
        let incoming = iter.next().unwrap();
        for route in route_collection.get.iter() {
            if route.url == incoming {
                send_response(&route, stream);
                break;
            }
        }
    } else if method == "post" {
        
    }
    
    // match &method[..] {
    //     "get" => {
            
    //         for route in route_collection.get.iter() {
    //             println!("{}", _type_of(route.url));
    //             // println!("{}", route.url);
    //             // if route.url == iter.next().unwrap() {

    //             // }
    //         }
    //     },
    //     "post" => println!("post"),
    //     "put" => println!("put"),
    //     "delete" => println!("delete"),
    //     _ => panic!("What the fucking kind of route is that?!")
    // };

    // let iter = routes.iter();

    // for route in iter {
    //     if buffer.starts_with(route.route.as_bytes()) {
    //         send(&route, stream);
    //         break;
    //     }
    // }
}

fn send_response(route: &Route, mut stream: TcpStream) {
    let status = "HTTP/1.1 200 OK\r\n\r\n";
    let file = fs::read_to_string(&route.file).unwrap();

    let response = format!("{}{}", status, file);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
 }