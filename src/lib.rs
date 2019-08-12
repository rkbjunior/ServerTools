#[macro_use]
extern crate diesel;
use diesel::prelude::*;
use dotenv::dotenv;
use r2d2::{Pool, PooledConnection};
use r2d2_diesel::ConnectionManager;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request, State};
use std::env;
use std::ops::Deref;
use wmi::{COMLibrary, WMIConnection};

pub mod models;
pub mod query_structs;
pub mod schema;

const GIGACONVERSION: f64 = 1_048_576.0;
const MEGACONVERSION: f64 = 1024.0;

/// Converts a float that represents memory in bytes to MegaBytes or Gigabytes depending on the passed in paramaters.
///
/// # Example
/// ```
/// let freemem = convert_memory_units(1048576.0);
/// ```
///
/// # Arguments
///
/// * `number` - A floating point number
/// * `scale` - A string that represents the scale. Either "MB" for Megabytes or "GB" for Gigabytes.
///
/// # Returns
///
/// A floating point number that represents the memory in the chosen scale.
///
/// # Remarks
///
/// This function uses consts `GIGACONVERSION` and `MEGACONVERSION` to calculate the float in bytes into a float in MB or GB.
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
///
/// # Example
/// ```
/// let freemem = round_decimals(1048576.564);
/// ```
///
/// # Arguments
///
/// * `number` - A floating point number
/// * `decimal_places` - An integer that number of places to trim to after the decimal point.
///
/// # Returns
///
/// A floating point trimmed to the specified decimal places.
///
/// # Remarks
///
/// This function trims the end of a floating point number to the specified number of decimals.
/// first we raise a base 10 to the power of decimal places to trim to. Then we multiply our number
/// by our new base and round the result Lastly we divide the result by the result of the first step.
pub fn round_decimals(mut number: f64, decimal_places: u64) -> f64 {
    let base: f64 = 10.0;
    let decimal_places = base.powf(decimal_places as f64);

    number = (number * decimal_places).round() / decimal_places;

    number
}

/// Gets a WMI connection from the `wmi` crate.
///
/// # Example
/// Localhost:
/// ```
/// let wmi_con = get_wmi_connection(None);
/// ```
///
/// Remote host:
/// ```
/// let wmi_con = get_wmi_connection(Some("HOST".to_string()));
/// ```
///
/// # Arguments
///
/// * `host` - The host name of the remote server to get a connection to. A value of None will get the localhost.
///
/// # Returns
///
/// A result type contiaining the `WMIConnection` or an `Err`
///
/// # Remarks
///
/// Functions opens a new COMLibrary and then checks the result, if its okay, it opens a WMIConnection using the COMLibrary
/// If the WMIConnection is ok, it is returned.
pub fn get_wmi_connection(host: Option<String>) -> Result<WMIConnection, String> {
    //WMI crate fails to get com connection sometimes, not sure why, but i loop
    //here until I get back a good connection until I can figure this out.
    let comresult = COMLibrary::new();

    //comresult should be okay, but lets check it anyway
    if comresult.is_ok() {
        let com_con = comresult.unwrap();
        let wmi_con = WMIConnection::new(com_con.into(), host);

        //check that th wmi connection was okay
        if wmi_con.is_ok() {
            return Ok(wmi_con.unwrap());
        }

        return Err("WMIConnection attempt failed.".to_string());
    }

    Err("Comm library connection failed.".to_string())
}

/// Runs a WMI query for Win32_PerfFormattedData_Counters_ProcessorInformation
///
/// # Example
/// Localhost:
/// ```
/// let cpu = get_cpu(None);
/// ```
///
/// Remote host:
/// ```
/// let cpu = get_cpu(Some("HOST".to_string()));
/// ```
///
/// # Arguments
///
/// * `host` - The host name of the remote server to get a connection to. A value of None will get the localhost.
///
/// # Returns
///
/// An Option type containing a `ProcessUtilization` struct or None which basically means the query failed for some reason.
///
/// # Remarks
///
/// Functions gets a wmi connection and then runs a query for the specified type.
pub fn get_cpu(host: Option<String>) -> Option<query_structs::ProcessUtilization> {
    let wmi_con = get_wmi_connection(host);

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

/// Runs a WMI query for Win32_OperatingSystem
///
/// # Example
/// Localhost:
/// ```
/// let os = get_os(None);
/// ```
///
/// Remote host:
/// ```
/// let os = get_os(Some("HOST".to_string()));
/// ```
///
/// # Arguments
///
/// * `host` - The host name of the remote server to get a connection to. A value of None will get the localhost.
///
/// # Returns
///
/// An Option type containing a `OperatingSystem` struct or None which basically means the query failed for some reason.
///
/// # Remarks
///
/// Functions gets a wmi connection and then runs a query for the specified type.
pub fn get_os(host: Option<String>) -> Option<query_structs::OperatingSystem> {
    let wmi_con = get_wmi_connection(host);

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

/// Runs a WMI query for Win32_Service
///
/// # Example
/// Localhost:
/// ```
/// let service = get_services(None);
/// ```
///
/// Remote host:
/// ```
/// let service = get_services(Some("HOST".to_string()));
/// ```
///
/// # Arguments
///
/// * `host` - The host name of the remote server to get a connection to. A value of None will get the localhost.
///
/// # Returns
///
/// An Option type containing a vector of `Win32Service` structs or None which basically means the query failed for some reason.
///
/// # Remarks
///
/// Functions gets a wmi connection and then runs a query for the specified type.
pub fn get_services(host: Option<String>) -> Option<Vec<query_structs::Win32Service>> {
    let wmi_con = get_wmi_connection(host);

    if wmi_con.is_ok() {
        let query = wmi_con.unwrap().query();

        if query.is_ok() {
            println! {"query ok"};
            let services: Vec<query_structs::Win32Service> = query.unwrap();

            return Some(services);
        }
        println! {"{:?}",query};
        return None;
    }
    None
}

/// Combines two WMI queries into one data structure. Combines `get_os` and `get_cpu`
///
/// # Example
/// Localhost:
/// ```
/// let res = get_stats(None);
/// ```
///
/// Remote host:
/// ```
/// let res = get_stats(Some("host".to_string()));
/// ```
///
/// # Arguments
///
/// * `host` - The host name of the remote server to get a connection to. A value of None will get the localhost.
///
/// # Returns
///
/// An `Stats` struct that combines os info, memory, and cpu information into one data structure.
///
/// # Remarks
///
/// Functions calls two different wmi queries and combines them into one data structure. Does some conversion of the mem and cpu values so they are in the correct format.
pub fn get_stats(host_name: Option<String>) -> query_structs::Stats {
    let cpu = get_cpu(host_name.clone());

    //let osinfo;
    let osinfo = get_os(host_name.clone());

    let mut stats = query_structs::Stats::new();

    match osinfo {
        Some(o) => {
            stats.set_osname(o.caption.clone());
            stats.set_osbuild(o.buildnumber.clone());
            stats.set_arch(o.osarchitecture.clone());
            stats.set_install(o.installdate);
            stats.set_lastboot(o.last_boot_up_time);
            stats.set_compname(o.csname.clone());

            let freemem = round_decimals(
                convert_memory_units(
                    o.freephysicalmemory.clone().parse::<f64>().unwrap(),
                    "GB".to_string(),
                ),
                2,
            );

            let totalmem = round_decimals(
                convert_memory_units(
                    o.total_visible_memory_size.clone().parse::<f64>().unwrap(),
                    "GB".to_string(),
                ),
                2,
            );

            let usedmem = round_decimals(totalmem - freemem, 2);

            stats.set_freemem(freemem);
            stats.set_totalmem(totalmem);
            stats.set_usedmem(usedmem);
        }
        None => println!("There was a problem loading wmi data."),
    }

    match cpu {
        Some(c) => stats.set_cpu(c.percent_processor_time.parse::<u64>().unwrap()),
        None => println!("There was a problem loading wmi data."),
    }

    stats
}

/// Creates a connection pool for database connectivity.
///
/// # Example
///
/// ```
/// let pool = create_db_pool();
/// ```
///
/// # Returns
///
/// An pool of connections that the application can use to access the database.
pub fn create_db_pool() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    Pool::new(manager).expect("Failed to create pool.")
}

/// Struct for managing a database connection.
pub struct DbConn(PooledConnection<ConnectionManager<PgConnection>>);

/// Implments the `FromRequest` trait for DBCon
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

/// Implments the `Deref` trait for DBCon
impl Deref for DbConn {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
