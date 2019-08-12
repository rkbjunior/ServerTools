//Examples used from					https://rocket.rs/v0.4/guide/getting-started/#hello-world
//Exmaples used from					https://notryanb.github.io/rust-blog-series-1.html
//examples used from					https://docs.rs/wmi/0.4.4/wmi/

#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

use lib::create_db_pool;

use rocket_contrib::templates::Template;

pub mod get;
pub mod post;

#[cfg(test)]
mod test;

/// Functions returns a Rocket instance fully configured with our routes and connection pool for the database.
fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .manage(create_db_pool())
        .mount(
            "/",
            routes![
                get::file,
                get::index,
                get::add,
                post::add,
                post::remove,
                get::server,
                get::stats
            ],
        )
        .attach(Template::fairing())
}

/// Main function starts the rocket instance.
fn main() {
    rocket().launch();
}
