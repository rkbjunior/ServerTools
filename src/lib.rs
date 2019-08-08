#[macro_use] extern crate diesel;
use diesel::prelude::*;

use dotenv::dotenv;

use r2d2::{Pool, PooledConnection};
use r2d2_diesel::ConnectionManager;

use rocket::{Outcome, Request, State};
use rocket::http::Status;
use rocket::request::{self, FromRequest};

use std::env;
use std::ops::Deref;

use wmi::{COMLibrary, WMIConnection};

pub mod models;
pub mod query_structs;
pub mod schema;

const GIGACONVERSION: f64 = 1_048_576.0;
const MEGACONVERSION: f64 = 1024.0;

/// Converts a float that represents memory in bytes to MegaBytes or Gigabytes depending on the passed in paramaters.
pub fn convert_memory_units(mut number: f64, scale: String) -> f64 {
	if scale == "MB" {
		number /= MEGACONVERSION;
	}

	if scale == "GB" {
		number /= GIGACONVERSION;
	}

	number
}

/// Trims a float to a specified number of decimal places.
pub fn round_decimals(mut number: f64, decimal_places: u64) -> f64 {
	let base: f64 = 10.0;
	let decimal_places = base.powf(decimal_places as f64);

	number = (number * decimal_places).round() / decimal_places;

	number
}

pub fn get_wmi_connection() -> Result<WMIConnection,String> {

	//WMI crate fails to get com connection sometimes, not sure why, but i loop
	//here until I get back a good connection until I can figure this out.
	let comresult = COMLibrary::new();

	//comresult should be okay, but lets check it anyway
	if comresult.is_ok() {
		let com_con = comresult.unwrap();
		let wmi_con = WMIConnection::new(com_con.into());

		//check that th wmi connection was okay
		if wmi_con.is_ok() {
			return Ok(wmi_con.unwrap());
		} 

		return Err("WMIConnection attempt failed.".to_string());
	}

	Err("Comm library connection failed.".to_string())
}

pub fn get_cpu() -> Option<query_structs::ProcessUtilization> {
	let wmi_con = get_wmi_connection();

	if wmi_con.is_ok() {
		let query = wmi_con.unwrap().query();

		if query.is_ok() {
			let info: Vec<query_structs::ProcessUtilization> = query.unwrap();
			return Some(info[0].clone());

		}
		return None;
	}

	None
}

pub fn get_os() -> Option<query_structs::OperatingSystem> {
	let wmi_con = get_wmi_connection();

	if wmi_con.is_ok() {
		let query = wmi_con.unwrap().query();

		if query.is_ok() {
			let os: Vec<query_structs::OperatingSystem> = query.unwrap();
			return Some(os[0].clone());

		}
		return None;
	}
	None
}

pub fn get_stats2() -> query_structs::Stats {
	let cpu = get_cpu();

	//let osinfo;
	let osinfo = get_os();

	let mut stats = query_structs::Stats::new();

	match osinfo {
		Some(o) => {
			stats.set_osname(o.caption.clone());
			stats.set_osbuild(o.buildnumber.clone());
			stats.set_arch(o.osarchitecture.clone());
			stats.set_install(o.installdate);
			stats.set_lastboot(o.last_boot_up_time);
			stats.set_compname(o.csname.clone());

			let freemem = round_decimals (
				convert_memory_units (
					o.freephysicalmemory
					.clone()
					.parse::<f64>()
					.unwrap()
					,"GB".to_string()
				)
				, 2
			);

			let totalmem = round_decimals (
				convert_memory_units (
					o.total_visible_memory_size
					.clone()
					.parse::<f64>()
					.unwrap()
					,"GB".to_string()
				)
				, 2
			);

			let usedmem = round_decimals( totalmem - freemem, 2);

			stats.set_freemem(freemem);
			stats.set_totalmem(totalmem);
			stats.set_usedmem(usedmem);
		},
		None => println!("There was a problem loading wmi data.")
	}

	match cpu {
		Some(c) => stats.set_cpu(c.percent_processor_time.parse::<u64>().unwrap()),
		None => println!("There was a problem loading wmi data.")
	}

	stats
}

pub fn create_db_pool() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    Pool::new(manager).expect("Failed to create pool.")
}

pub struct DbConn(PooledConnection<ConnectionManager<PgConnection>>);

impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<DbConn, ()> {
        let pool = request.guard::<State<Pool<ConnectionManager<PgConnection>>>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

impl Deref for DbConn {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}