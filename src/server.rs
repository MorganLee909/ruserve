use std::net::TcpListener;
use std::io::prelude::*;
use std::fs;

use std::any::type_name;

fn _type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

struct Route<'a> {
    route: String,
    f: fn() -> &'a str
}

pub struct App<'a> {
    get_list: Vec<Route<'a>>,
    post_list: Vec<Route<'a>>,
}

impl<'a> App<'a> {
    pub fn get(&mut self, route: &str, f: fn() -> &'a str){
        self.get_list.push(Route {
            route: route.to_string(),
            f: f
        });
    }

    pub fn post(&mut self, route: &str, f: fn() -> &'a str) {
        self.post_list.push(Route {
            route: route.to_string(),
            f: f
        });
    }
}

pub fn create_app<'a>() -> App<'a> {
    App {
        get_list: vec![],
        post_list: vec![]
    }
}

pub fn listen(app: App, port: &str) {
    let address = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(address).unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut result) => {
                let mut buffer = [0; 1024];
                result.read(&mut buffer).unwrap();
                println!("{}", String::from_utf8_lossy(&buffer[..]));
                let request_string = String::from_utf8_lossy(&buffer[..]);
                let words: Vec<&str> = request_string.split(" ").collect();

                let response = match words[0] {
                    "GET" => get_request(words[1], &app),
                    "POST" => post_request(words[1], &app),
                    _ => panic!("{}", "AAAAAHHH!!!")
                };


                result.write(response.as_bytes()).unwrap();
                result.flush().unwrap();
            }
            Err(e) => panic!("{}", e)
        };
    }
}

fn get_request(request: &str ,app: &App) -> String {
    let mut response: String = String::from("");
    for route in &app.get_list {
        if route.route == request {
            response = format_response((route.f)());
            break;
        }
    }

    response
}

fn post_request(request: &str, app: &App) -> String {
    let mut response: String = String::from("");
    for route in &app.post_list {
        if route.route == request {
            response = format_response((route.f)());
            break;
        }
    }

    response
}

fn format_response(file: &str) -> String {
    let status = "HTTP/1.1 200 OK\r\n\r\n";
    let file = fs::read_to_string(&file).unwrap();
    return format!("{}{}", status, file);
}