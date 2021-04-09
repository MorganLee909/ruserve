mod server;

use server::*;

fn main() {
    let mut routes: Vec<Route> = Vec::new();

    routes.push(Route {
        method: "get",
        url: "/",
        file: "index.html"
    });

    routes.push(Route {
        method: "get",
        url: "/style",
        file: "index.css"
    });

    listen(routes, "7878".to_string());
}