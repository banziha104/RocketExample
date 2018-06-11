#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

mod router;
mod model;

use router::api_router::api_router;

#[get("/")]
fn index() -> &'static str {
    "Hello, Rocket!"
}


fn main() {

    rocket::ignite().mount("/",
    routes![
        index,
        api_router::api_get,

        ]).launch();
}