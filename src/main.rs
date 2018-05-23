#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
mod router;
use router::api_router::api_router;

#[get("/")]
fn index() -> &'static str {
    "Hello, Rocket!"
}

fn main() {
    rocket::ignite().mount("/",routes![index,api_router::api_get,api_router::api_post]).launch();
}