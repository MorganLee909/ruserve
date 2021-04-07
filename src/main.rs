mod server;

use server::*;

fn main() {
    let mut routes: Vec<Route> = Vec::new();

    routes.push(Route::new(
        "get".to_string(),
        "/".to_string(),
        "index.html".to_string()
    ));

    routes.push(Route::new(
        "get".to_string(),
        "/style".to_string(),
        "index.css".to_string()
    ));

    listen(routes, "7878".to_string());
}