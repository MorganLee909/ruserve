mod server;
mod controller;

use server::*;
use controller::*;

fn main() {
    let mut app = create_app();

    app.get("/", home);
    app.get("/style", style_page);

    listen(app, "8000");
}