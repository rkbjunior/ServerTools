//Hello world example for rocket from	https://rocket.rs/v0.4/guide/getting-started/#hello-world
//Exmaples used from					https://notryanb.github.io/rust-blog-series-1.html
//examples used from					https://docs.rs/wmi/0.4.4/wmi/

// Enables procedural macros so that the route macro below can be expanded into an expression.
#![feature(proc_macro_hygiene, decl_macro)]

//Enable custom attibutes
//#![feature(custom_attribute)]

#[macro_use] extern crate rocket;

mod routes;
mod wmiqueries;
use routes::{scripts, get };

use rocket_contrib::templates::Template;
//use rocket::routes;


fn main() {
    rocket::ignite().mount("/", routes![scripts::file,get::index,get::stats, get::add]).attach(Template::fairing()).launch();
}