use diesel;
use diesel::prelude::*;
use lib::models::*;
use lib::*;
use rocket::response::NamedFile;
use rocket_contrib::json::Json;
use rocket_contrib::templates::Template;
use std::path::{Path, PathBuf};
use tera::Context;

/// GET route for the root of the web application. Matches /
///
/// # Example
/// HTTP GET /
///
/// # Arguments
///
/// * `connection` - A DBCon struct, a wrapper for a pooled connection and connection from r2d2 crate.
///
/// # Returns
/// * `Template` - Returns a rendered Template based on a `tera::Context`
///
/// # Remarks
///
/// This route will first query the locahosts WMI provider for system information including memory useage,
/// cpu utilization, and a list of all the services on the computer. It then applies all the data from the
/// query into a `tera::Context` and renders it to the screen.
#[get("/")]
pub fn index(connection: DbConn) -> Template {
    use schema::remote_servers::dsl::*;

    //Base path is always the localhost so host is None
    let host = None;

    //Query wmi for machine info and services.
    let res = lib::get_stats(host.clone());
    let ser = lib::get_services(host.clone());
	let mut context = Context::new();
	let mut template_name = "layout".to_string();

	match ser {
		Some(s) => context.insert("win32service", &s),
		None => template_name = "hosterror".to_string(),
	}

    //Load the registered servers from the database.
    let servers_list = remote_servers
        .load::<Server>(&*connection)
        .expect("Error Loading Servers");



    //Add the data to our template
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
    context.insert("servers", &servers_list);

    //Render the template
    Template::render(template_name, &context)
}

/// GET route for a specific registered remote server. Matches /servers/<server_name>
///
/// # Example
/// HTTP GET /servers/<server_name>
/// Where <server_name> is the name of your remote server.
///
/// # Arguments
///
/// * `connection` - A `DBCon` struct, a wrapper for a pooled connection and connection from `r2d2` crate.
/// * `name` - The hostname of the remote server.
///
/// # Returns
/// * `Template` - Returns a rendered Template based on a `tera::Context`
///
/// # Remarks
///
/// This route will first query the locahosts WMI provider for system information including memory useage,
/// cpu utilization, and a list of all the services on the computer. It then applies all the data from the
/// query into a `tera::Context` and renders it to the screen.
#[get("/servers/<name>")]
pub fn server(connection: DbConn, name: String) -> Template {
    use schema::remote_servers::dsl::*;

    //get the name of the computer we need to get information on.
    let host = Some(name);

    //Query wmi for machine info and services.
    let res = lib::get_stats(host.clone());
    let ser = lib::get_services(host.clone());
	let mut context = Context::new();
	let mut template_name = "layout".to_string();

	match ser {
		Some(s) => context.insert("win32service", &s),
		None => template_name = "hosterror".to_string(),
	}

	//println!("{:?}",res);
	//println!("{:?}",ser);

    //Load the registered servers from the database.
    let servers_list = remote_servers
        .load::<Server>(&*connection)
        .expect("Error Loading Servers");



    //Add the data to our template
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
    context.insert("servers", &servers_list);
    

    //Render the template
    Template::render(template_name, &context)
}

/// GET route for the register server page. Matches /add.
///
/// # Example
/// HTTP GET /add
///
/// # Arguments
///
/// * `connection` - A `DBCon` struct, a wrapper for a pooled connection and connection from `r2d2` crate.
///
/// # Returns
/// * `Template` - Returns a rendered template based on a `tera::Context`
///
/// # Remarks
///
/// This route queries the list of all the registered servers and populates them in the browser. Provides
/// The functionality to add and remove new registered servers.
#[get("/add")]
pub fn add(connection: DbConn) -> Template {
    use schema::remote_servers::dsl::*;

    //Load the registered servers from the database.
    let servers_list = remote_servers
        .load::<Server>(&*connection)
        .expect("Error Loading Servers");

    let mut context = Context::new();

    //Add the data to our template
    context.insert("servers", &servers_list);
    context.insert("class", "hidenotice");
    context.insert("message", "Nothing to see here, move along...");

    //Render the template
    Template::render("add", context)
}

/// GET route for dynamic ajax calls that update server cpu and memory usuage. Matches /stats/<server_name>
///
/// # Example
/// HTTP GET /stats/<server_name>
/// ContentType must be `application/json`
/// Where <server_name> is the name of your remote server.
///
/// # Arguments
///
/// * `server_name` - A string with the hostname of the remote server you want to query.
///
/// # Returns
///
/// * `Json<lib::query_structs::Stats>` - A json payload with the updated stats.
///
/// # Remarks
///
/// This route is a JSON route. Its specifically used for ajax calls so that the cpu, and memory can be updated without refreshing the page.
#[get("/stats/<server_name>", format = "application/json")]
pub fn stats(server_name: String) -> Json<lib::query_structs::Stats> {
    let host = Some(server_name);
    let res: lib::query_structs::Stats = lib::get_stats(host);
    Json(res)
}

/// GET route for all the static files used on the site. Matches /static/<sub_folder>/<file>
///
/// # Example
/// HTTP GET /satic/<sub_folder>/<file>
/// Where <sub_folder> can be any subdirectory under static and <file> is the name of the file to match.
///
/// # Arguments
///
/// * `file` - A `rocket` `PathBuf` struct.
/// * `sub_folder` - A string holding the name of the sub folder to look in.
///
/// # Returns
///
/// * `Option<NamedFile>` - Returns None if the file wasn't found, and the contents of the file if it was found.'
///
/// # Remarks
///
/// This route is a JSON route. Its specifically used for ajax calls so that the cpu, and memory can be updated without refreshing the page.
#[get("/static/<sub_folder>/<file..>")]
pub fn file(file: PathBuf, sub_folder: String) -> Option<NamedFile> {
    let path = format!("static/{}/", sub_folder);
    NamedFile::open(Path::new(&path).join(file)).ok()
}
