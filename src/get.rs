use lib::*;
use lib::models::*;

use diesel;
use diesel::prelude::*;


use rocket::response::NamedFile;
use rocket_contrib::json::Json;
use rocket_contrib::templates::Template;

use std::path::{Path, PathBuf};

use tera::Context;

/// Route the returns a tera template, populating it with program variables gathered from wmi calls.
#[get("/")]
pub fn index() -> Template {

	let res = lib::get_stats2();
	let mut context = Context::new();

	context.insert("os_name", &res.osname);
	context.insert("build", &res.osbuild);
	context.insert("last_boot", &res.lastboot);
	context.insert("comp_name", &res.compname);
	context.insert("free_mem", &res.freemem);
	context.insert("used_mem", &res.usedmem);
	context.insert("installdate", &res.install);
	context.insert("architecture", &res.arch);
	context.insert("total_mem", &res.totalmem);
	context.insert("cpu_utilization", &res.cpuu);

	Template::render("layout", &context)
}

#[get("/add")]
pub fn add(connection: DbConn) -> Template {
	use schema::remote_servers::dsl::*;

	let mut context = Context::new();
	let servers_list = remote_servers.load::<Server>(&*connection).expect("Error Loading Servers");
	context.insert("servers", &servers_list);

	Template::render("add",context)
}

/// Route for ajax call to dynamically update the html page with new data at a specific interval.
/// Uses wmi connection to obtain most recent data and returns the data as a JSON string using serde Serialize.
#[get("/stats", format = "application/json")]
pub fn stats() -> Json<lib::query_structs::Stats> {
	
	let res: lib::query_structs::Stats = lib::get_stats2();

	Json(res)
}

#[get("/static/<sub_folder>/<file..>")]
pub fn file(file: PathBuf, sub_folder: String) -> Option<NamedFile> {

	let path = format!("static/{}/", sub_folder);
	NamedFile::open(Path::new(&path).join(file)).ok()

}