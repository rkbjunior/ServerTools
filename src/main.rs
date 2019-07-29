//Hello world example for rocket from https://rocket.rs/v0.4/guide/getting-started/#hello-world
//Exmaples used from https://notryanb.github.io/rust-blog-series-1.html
//examples used from https://docs.rs/wmi/0.4.4/wmi/

#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
extern crate rocket_contrib;
extern crate tera;
extern crate winapi;
extern crate wmi;

mod routes;
use crate::routes::{scripts, get };
use rocket_contrib::templates::Template;

fn main() {
    rocket::ignite().mount("/", routes![scripts::file,get::index,get::stats, get::add]).attach(Template::fairing()).launch();
}