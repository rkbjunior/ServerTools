//Hello world example for rocket from https://rocket.rs/v0.4/guide/getting-started/#hello-world
//Exmaples used from https://notryanb.github.io/rust-blog-series-1.html
//examples used from https://docs.rs/wmi/0.4.4/wmi/

#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
extern crate rocket_contrib;
extern crate tera;
extern crate winapi;
extern crate wmi;

use rocket_contrib::templates::Template;
use tera::Context;
//use std::collections::HashMap;
//use wmi::Variant;
use wmi::{COMLibrary, WMIConnection};
use serde::Deserialize;
use wmi::WMIDateTime;

#[derive(Deserialize, Debug)]
#[serde(rename = "Win32_OperatingSystem")]
#[serde(rename_all = "PascalCase")]
struct OperatingSystem {
	caption: String,
	debug: bool,
	last_boot_up_time: WMIDateTime,
}

#[get("/")]
fn index() -> Template {

	let com_con = COMLibrary::new().unwrap();
	let wmi_con = WMIConnection::new(com_con.into()).unwrap();

	let results: Vec<OperatingSystem> = wmi_con.query().unwrap();

	for os in &results {
		println!("{:#?}", os);
	}

	let mut context = Context::new();

	context.insert("my_message", &results[0].caption);
	Template::render("layout", &context)
	
}


fn main() {
    rocket::ignite().mount("/", routes![index]).attach(Template::fairing()).launch();

}