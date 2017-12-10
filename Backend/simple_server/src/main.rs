#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[get("/")]
fn index() -> &'static str {
	"Hello, world!"
}

#[get("/test")]
fn test() -> &'static str {
	"This is the test page"
}

fn main() {
    rocket::ignite().mount("/", routes![index, test]).launch();
}
