#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello World"
}

#[get("/ohea")]
fn test() -> &'static str {
    "ohea"
}

fn main() {
    rocket::ignite().mount("/", routes![index, test]).launch();
}
