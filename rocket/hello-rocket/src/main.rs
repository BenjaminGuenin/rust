#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[get("/test")]
fn index() -> &'static str {
    "Hey, world!"
}

fn main() {
    rocket::ignite().mount("/test", routes![index]).launch();
}