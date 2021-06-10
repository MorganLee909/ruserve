mod server;
mod controller;

use server::*;
use controller::*;

fn main() {
    let mut app = create_app();

    app.get("/", home);
    app.get("/style", style_page);
    app.post("/", post);

    listen(app, "7878");
}