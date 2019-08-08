use lib::*;
use lib::models::*;

use diesel;
use diesel::prelude::*;


//use rocket::response::NamedFile;
use rocket::request::Form;
//use rocket_contrib::json::Json;
use rocket_contrib::templates::Template;


//use std::collections::HashMap;
//use std::path::{Path, PathBuf};

use tera::Context;

#[derive(FromForm,Clone)]
pub struct ServerFormData {
	pub servername: String,
	pub ipaddress: String,
}

#[post("/add", data = "<serverformdata>")]
pub fn add(connection: DbConn, serverformdata: Form<ServerFormData>) -> Template {
	use schema::remote_servers::dsl::*;

	let nserver = NewServer { servername: serverformdata.servername.clone(), ip_address: serverformdata.ipaddress.clone()};

	diesel::insert_into(remote_servers).values(&nserver).execute(&*connection).unwrap();

	let mut context = Context::new();
	let servers_list = remote_servers.load::<Server>(&*connection).expect("Error Loading Servers");

	context.insert("servers", &servers_list);

	Template::render("add",context)
}
