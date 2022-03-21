use std::net::TcpListener;
use std::io::prelude::*;
use std::fs;
use std::collections::HashMap;

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

pub struct Request {
    headers: HashMap<String, String>
}

impl<'a> App<'a> {
    pub fn get(&mut self, route: &str, f: fn() -> &'a str){
        self.get_list.push(Route {
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
                let request_string = String::from_utf8_lossy(&buffer[..]).into_owned();
                println!("{}", format_request(request_string));
                // let words: Vec<&str> = request_string.split(" ").collect();

                // let response = match words[0] {
                //     "GET" => get_request(words[1], &app, request_string),
                //     _ => panic!("{}", "AAAAAHHH!!!")
                // };


                // result.write(response.as_bytes()).unwrap();
                // result.flush().unwrap();
            }
            Err(e) => panic!("{}", e)
        };
    }
}

fn format_request(request: String) -> (String){
    let lines: Vec<&str> = request.split("\r\n").collect();
    let mut location = String::from("");

    for line in lines {
        location = String::from(line);
        break;
    }

    (location)
}

fn get_request(request: &str, app: &App, request_string: String) -> String {
    // println!("{}", request_string);
    let mut response: String = String::from("");
    for route in &app.get_list {
        if route.route == request {
            response = format_response((route.f)());
            break;
        }
    }

    response
}

fn format_response(file: &str) -> String {
    let status = "HTTP/1.1 200 OK\r\n\r\n";
    let file = match fs::read_to_string(&file) {
        Ok(result) => result,
        Err(_e) => fs::read_to_string("src/404.html").unwrap()
    };
    return format!("{}{}", status, file);
}