use lib::*;
use lib::models::*;
use diesel;
use diesel::prelude::*;
use rocket::response::NamedFile;
use rocket_contrib::json::Json;
use rocket_contrib::templates::Template;
use std::path::{Path, PathBuf};
use tera::Context;

#[get("/")]
pub fn index(connection: DbConn) -> Template {
	use schema::remote_servers::dsl::*;

	//get the name of the first registered server use none if none.
	let host = None;
	println!("Calling / stats2");

	let res = lib::get_stats2(host.clone());
	let ser = lib::get_services(host.clone()).unwrap();

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

	let servers_list = remote_servers.load::<Server>(&*connection).expect("Error Loading Servers");

	context.insert("servers", &servers_list);
	context.insert("win32service", &ser);
	Template::render("layout", &context)
}

#[get("/servers/<name>")]
pub fn server(connection: DbConn, name: String) -> Template {
	use schema::remote_servers::dsl::*;

	//get the name of the first registered server use none if none.
	let host = Some(name);
	println!("Calling /<name> stats2");
	let res = lib::get_stats2(host.clone());
	let ser = lib::get_services(host.clone()).unwrap();
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

	let servers_list = remote_servers.load::<Server>(&*connection).expect("Error Loading Servers");
	context.insert("servers", &servers_list);
	context.insert("win32service", &ser);
	Template::render("layout", &context)
}

#[get("/add")]
pub fn add(connection: DbConn) -> Template {
	use schema::remote_servers::dsl::*;

	let mut context = Context::new();
	let servers_list = remote_servers.load::<Server>(&*connection).expect("Error Loading Servers");

	context.insert("servers", &servers_list);
	context.insert("class","hidenotice");
	context.insert("message","Nothing to see here, move along...");

	Template::render("add",context)
}

/// Route for ajax call to dynamically update the html page with new data at a specific interval.
/// Uses wmi connection to obtain most recent data and returns the data as a JSON string using serde Serialize.
//#[get("/stats", format = "application/json")]
//pub fn stats() -> Json<lib::query_structs::Stats> {
	//let host = None;

	//let res: lib::query_structs::Stats = lib::get_stats2(host);
	//Json(res)
//}

#[get("/stats/<name>", format = "application/json")]
pub fn stats_by_name(name: String) -> Json<lib::query_structs::Stats> {
	let host = Some(name);
		println!("Calling /stats/<name> stats2");
	let res: lib::query_structs::Stats = lib::get_stats2(host);
	Json(res)
}

#[get("/static/<sub_folder>/<file..>")]
pub fn file(file: PathBuf, sub_folder: String) -> Option<NamedFile> {

	let path = format!("static/{}/", sub_folder);
	NamedFile::open(Path::new(&path).join(file)).ok()
}