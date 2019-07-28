extern crate rocket;

use rocket_contrib::templates::Template;
use tera::Context;
use wmi::{COMLibrary, WMIConnection};
use serde::Deserialize;
use serde::Serialize;
use wmi::WMIDateTime;
use rocket_contrib::json::Json;

/// Struct that stores operating system information from a wmi call.
#[derive(Deserialize, Debug)]
#[serde(rename = "Win32_OperatingSystem")]
#[serde(rename_all = "PascalCase")]
struct OperatingSystem {
	caption: String,				//os_name
	buildnumber: String,			//build
	debug: bool,
	last_boot_up_time: WMIDateTime,	//last_boot
	csname: String,					//comp_name
	description: String,			//desc
	freephysicalmemory: String,		//free_mem
	installdate: WMIDateTime,		//installdate
	localdatetime: WMIDateTime,		//local_date
	numberofprocesses: u32,			//num_of_processes
	numberofusers: u32,				//num_of_users
	operatingsystemsku: u32,		//sku
	osarchitecture: String,			//architecture
	total_visible_memory_size: String,	//total_mem

}

/// Struct that stores cpu information from a wmi call.
#[derive(Deserialize, Debug)]
#[serde(rename = "Win32_PerfFormattedData_Counters_ProcessorInformation")]
#[serde(rename_all = "PascalCase")]
struct ProcessUtilization {
	percent_processor_time: String
}


/// Struct that stores current pc statistics
/// * `freemem` - A value the represents the computers free memory.
/// * `totalmem` - A value that represents the computers total memory.
/// * `usedmem` - A value that represents the amount of memory in use.
/// * `cpuu` - A value that represents the current cpu utilization.
#[derive(Serialize, Deserialize)]
pub struct Stats {
	freemem : f64,
	totalmem: f64,
	usedmem: f64,
	cpuu: u64
}

const GIGACONVERSION: f64 = 1048576.0;
const MEGACONVERSION: f64 = 1024.0;

/// WMI returns memory as a String represting the number of bytes that are available/free. This function
/// convertst the string of bytes into a floating point number.
///
/// # Arguments
///
/// * `byte_string` - A String in bytes representing the amount of memory.
/// * `decimal_places` - A floating point number the sets the decimal places to round to.
/// * `scale` - A string the tells the function what to convert the bytes into, either MB or GB.
///
/// #Examples
///
/// ```
/// let x = "234404".to_string();
/// let result = convert_bytes_string_to_float(x, 2.0, "GB".to_string());
/// assert_eq!(result, 0.23);
/// ```
fn convert_bytes_string_to_gigabytes_float(byte_string: String, decimal_places: f64, scale: String ) -> f64 {
	let base: f64 = 10.0;
	let mut number = byte_string.parse::<f64>().unwrap();
	let decimal_places = base.powf(decimal_places);

	if scale == "MB".to_string() {
		number /= MEGACONVERSION;
	}

	if scale == "GB".to_string() {
		number /= GIGACONVERSION;
	}

	number = (number * decimal_places).round() / decimal_places;

	number
}

/// Route the returns a tera template, populating it with program variables gathered from wmi calls.
#[get("/")]
pub fn index() -> Template {

	let com_con = COMLibrary::new().unwrap();
	let wmi_con = WMIConnection::new(com_con.into()).unwrap();
	let mut osinfo: Vec<OperatingSystem> = Vec::new();
	let mut cpuinfo: Vec<ProcessUtilization> = Vec::new();

	//let results: Vec<OperatingSystem> = wmi_con.query();
	let osresults = wmi_con.query();
	let cpuresults = wmi_con.query();

	if osresults.is_ok() {
		osinfo = osresults.unwrap()
	} else {
		println!("WMI OS Query Failed.");
	}

	if cpuresults.is_ok() {
		cpuinfo = cpuresults.unwrap();
	} else {
		println!("WMI CPU Query Failed.");
	}
	

	let mut context = Context::new();

	let freemem = convert_bytes_string_to_gigabytes_float( osinfo[0].freephysicalmemory.clone(), 2.0, "GB".to_string());
	let totalmem = convert_bytes_string_to_gigabytes_float( osinfo[0].total_visible_memory_size.clone(), 2.0, "GB".to_string());
	let usedmem = totalmem - freemem;

	context.insert("os_name", &osinfo[0].caption);
	context.insert("build", &osinfo[0].buildnumber);
	context.insert("local_date", &osinfo[0].localdatetime);
	context.insert("last_boot", &osinfo[0].last_boot_up_time);
	context.insert("comp_name", &osinfo[0].csname);
	context.insert("desc", &osinfo[0].description);
	context.insert("free_mem", &freemem);
	context.insert("used_mem", &usedmem);
	context.insert("installdate", &osinfo[0].installdate);
	context.insert("num_of_processes", &osinfo[0].numberofprocesses);
	context.insert("num_of_users", &osinfo[0].numberofusers);
	context.insert("sku", &osinfo[0].operatingsystemsku);
	context.insert("architecture", &osinfo[0].osarchitecture);
	context.insert("total_mem", &totalmem);
	context.insert("cpu_utilization", &cpuinfo[0].percent_processor_time);

	Template::render("layout", &context)
	
}

/// Route for ajax call to dynamically update the html page with new data at a specific interval.
/// Uses wmi connection to obtain most recent data and returns the data as a JSON string using serde Serialize.
#[get("/stats", format = "application/json")]
pub fn stats() -> Json<Stats> {
	let com_con = COMLibrary::new().unwrap();
	let wmi_con = WMIConnection::new(com_con.into()).unwrap();

	let results: Vec<OperatingSystem> = wmi_con.query().unwrap();
	let cpu: Vec<ProcessUtilization> = wmi_con.query().unwrap();

	let freemem = convert_bytes_string_to_gigabytes_float( results[0].freephysicalmemory.clone(), 2.0, "GB".to_string());
	let totalmem = convert_bytes_string_to_gigabytes_float( results[0].total_visible_memory_size.clone(), 2.0, "GB".to_string());
	let usedmem = totalmem - freemem;
	let cpuu = cpu[0].percent_processor_time.to_string().parse::<u64>().unwrap();

	let json_string = Stats {
		freemem: freemem,
		totalmem: totalmem,
		usedmem: usedmem,
		cpuu: cpuu
	};

	Json(json_string)
}
