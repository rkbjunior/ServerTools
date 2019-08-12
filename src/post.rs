use lib::models::*;
use lib::*;

use diesel;
use diesel::prelude::*;

use rocket::request::Form;
use rocket_contrib::templates::Template;

use tera::Context;

/// Struct that hold form data from the register server page.
#[derive(FromForm, Clone)]
pub struct ServerFormData {
    pub servername: String,
    pub ipaddress: String,
}

/// Post route that registers a new server. Matches /add
///
/// # Arguments
///
/// * `connection` - A DBCon database connection
/// * serverformdata` - Form data from the http post.
///
/// # Returns
///
/// Returns a rendered template with all changes made.
#[post("/add", data = "<serverformdata>")]
pub fn add(connection: DbConn, serverformdata: Form<ServerFormData>) -> Template {
    use schema::remote_servers::dsl::*;

    let mut context = Context::new();

    context.insert("class", "hidenotice");
    context.insert("message", "Nothing to see here, move along...");

    let nserver = NewServer {
        servername: serverformdata.servername.clone(),
        ip_address: serverformdata.ipaddress.clone(),
    };

    if validate_add_server(&nserver) {
        let insert = diesel::insert_into(remote_servers)
            .values(&nserver)
            .execute(&*connection);

        match insert {
            Ok(i) => {
                context.insert("class", "notice success");
                context.insert("message", &format!("{} {}", i, "row(s) were inserted."));
            }
            Err(e) => {
                context.insert("class", "notice error");
                context.insert("message", &e.to_string());
            }
        };
    } else {
        context.insert("class", "notice info");
        context.insert("message", "Invalid server name.");
    }

    let servers_list = remote_servers
        .load::<Server>(&*connection)
        .expect("Error Loading Servers");

    context.insert("servers", &servers_list);

    Template::render("add", context)
}

/// Post route that removes a registered server. Matches /remove/<server_name>
///
/// # Arguments
///
/// * `connection` - A DBCon database connection
/// * `server_name` - The name of the server to remove.
///
/// # Returns
///
/// Returns a string indicating success or how many rows were deleted.
#[post("/remove/<server_name>")]
pub fn remove(connection: DbConn, server_name: String) -> String {
    use schema::remote_servers::dsl::*;

    let delete =
        diesel::delete(remote_servers.filter(servername.eq(server_name))).execute(&*connection);

    match delete {
        Ok(d) => return format!("{} {}", d, "row(s) were deleted."),
        Err(e) => e.to_string(),
    }
}

/// Function checks and makes sure the name of the server being added is valid.
///
/// # Arguments
///
/// * `entry` - A NewServer Struct that was added by the form.
///
/// # Returns
///
/// Returns a boolean that indicates the name is valid (true) or not valid (false)
///
/// # Remarks
/// Protects against blank names so that a blank server name wont be added to the database.
pub fn validate_add_server(entry: &NewServer) -> bool {
    if entry.servername == "" {
        return false;
    }

    true
}
