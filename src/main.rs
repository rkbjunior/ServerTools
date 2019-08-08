//Hello world example for rocket from	https://rocket.rs/v0.4/guide/getting-started/#hello-world
//Exmaples used from					https://notryanb.github.io/rust-blog-series-1.html
//examples used from					https://docs.rs/wmi/0.4.4/wmi/

#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use lib::create_db_pool;

use rocket_contrib::templates::Template;

pub mod get;
pub mod post;

fn main() {
    rocket::ignite()
		.manage(create_db_pool())
		.mount("/", routes![get::file,get::index,get::stats, get::add, post::add])
		.attach(Template::fairing())
		.launch();
}