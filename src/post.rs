use lib::*;
use lib::models::*;

use diesel;
use diesel::prelude::*;

use rocket::request::Form;
use rocket_contrib::templates::Template;

use tera::Context;

#[derive(FromForm,Clone)]
pub struct ServerFormData {
	pub servername: String,
	pub ipaddress: String,
}

#[post("/add", data = "<serverformdata>")]
pub fn add(connection: DbConn, serverformdata: Form<ServerFormData>) -> Template {
	use schema::remote_servers::dsl::*;

	let mut context = Context::new();

	context.insert("class","hidenotice");
	context.insert("message","Nothing to see here, move along...");

	let nserver = NewServer { servername: serverformdata.servername.clone(), ip_address: serverformdata.ipaddress.clone()};

	if validate_add_server (&nserver) {
		let insert = diesel::insert_into(remote_servers).values(&nserver).execute(&*connection);

		match insert {
			Ok(i) => {
				context.insert("class","notice success");
				context.insert("message",&format!("{} {}",i, "row(s) were inserted."));
			},
			Err(e) => {
				context.insert("class","notice error");
				context.insert("message",&e.to_string());
			},
		};
	} else {
		println!("{}","Was called");
		context.insert("class","notice info");
		context.insert("message","Invalid server name.");
	}

	let servers_list = remote_servers.load::<Server>(&*connection).expect("Error Loading Servers");

	context.insert("servers", &servers_list);

	Template::render("add",context)
}

#[post("/remove/<sname>")]
pub fn remove(connection: DbConn, sname: String) -> String {
	use schema::remote_servers::dsl::*;

	let delete = diesel::delete(remote_servers.filter(servername.eq(sname))).execute(&*connection);

	match delete {
		Ok(d) => {
			return format!("{} {}",d, "row(s) were deleted.")
		},
		Err(e) => {
			return e.to_string()
		},
	};
}

pub fn validate_add_server(entry: &NewServer) -> bool
{
	if entry.servername == "" {
		return false;
	}

	true
}