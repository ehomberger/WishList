#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;

use std::io;
use std::path::{Path, PathBuf};

use rocket::request::{Form, FromFormValue};
use rocket::response::Redirect;
use rocket::response::NamedFile;

#[derive(FromForm)]
struct ListItem {
	name: String,
	price: f32,
}

#[get("/")]
fn index() -> io::Result<NamedFile> {
	NamedFile::open("static/index.html")
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
	NamedFile::open(Path::new("static/").join(file)).ok()
}

#[post("/list/add", data = "<item_form>")]
fn add_item<'a>(item_form: Form<'a, ListItem>) -> Result<Redirect, String> {
	let item = item_form.get();

	println!("[item name, item price] --- [{}, {}]", item.name, item.price);
	Ok(Redirect::to("/"))
}

fn rocket() -> rocket::Rocket {
	rocket::ignite().mount("/", routes![index, files, add_item])
}

fn main() {
    rocket().launch();
}
