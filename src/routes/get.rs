extern crate rocket;

use rocket_contrib::templates::Template;
use tera::Context;
use wmi::{COMLibrary, WMIConnection, WMIDateTime};
use serde::Deserialize;
use serde::Serialize;
use rocket_contrib::json::Json;
use std::collections::HashMap;

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
	osname: String,
	osbuild: String,
	arch: String,
	install: String,
	lastboot: String,
	compname: String,
	freemem : f64,
	totalmem: f64,
	usedmem: f64,
	cpuu: u64
}

const GIGACONVERSION: f64 = 1048576.0;
const MEGACONVERSION: f64 = 1024.0;

/// Converts a float that represents memory in bytes to MegaBytes or Gigabytes depending on the passed in paramaters.
fn convert_memory_units(mut number: f64, scale: String) -> f64 {
	if scale == "MB".to_string() {
		number /= MEGACONVERSION;
	}

	if scale == "GB".to_string() {
		number /= GIGACONVERSION;
	}

	number
}

/// Trims a float to a specified number of decimal places.
fn round_decimals(mut number: f64, decimal_places: u64) -> f64 {
	let base: f64 = 10.0;
	let decimal_places = base.powf(decimal_places as f64);

	number = (number * decimal_places).round() / decimal_places;

	number
}

fn get_stats() -> Option<Stats> {

	//WMI crate fails to get com connection sometimes, not sure why, but i loop
	//here until I get back a good connection until I can figure this out.
	let mut comresult = COMLibrary::new();
	let mut i = 0;
	while !comresult.is_ok() && i < 500 {
		comresult = COMLibrary::new();
		i += 1;
	}

	let _com_con;

	//comresult should be okay, but lets check it anyway
	if comresult.is_ok() {
		_com_con = comresult.unwrap();

		let wmiresult = WMIConnection::new(_com_con.into());
		let _wmi_con;

		//check that th wmi connection was okay
		if wmiresult.is_ok() {
			_wmi_con = wmiresult.unwrap();

			let mut osinfo: Vec<OperatingSystem> = Vec::new();
			let mut cpuinfo: Vec<ProcessUtilization> = Vec::new();
			let osresults = _wmi_con.query();
			let cpuresults = _wmi_con.query();

			//Check that our queries worked okay
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
	
			let freemem = round_decimals (
				convert_memory_units (
					osinfo[0].freephysicalmemory
					.clone()
					.parse::<f64>()
					.unwrap()
					,"GB".to_string()
				)
				, 2
			);

			let totalmem = round_decimals (
				convert_memory_units (
					osinfo[0].total_visible_memory_size
					.clone()
					.parse::<f64>()
					.unwrap()
					,"GB".to_string()
				)
				, 2
			);

			let usedmem = round_decimals( totalmem - freemem, 2);

			return Some(Stats {
				osname: osinfo[0].caption.clone(),
				osbuild: osinfo[0].buildnumber.clone(),
				arch: osinfo[0].osarchitecture.clone(),
				install: osinfo[0].installdate.0.to_rfc3339(),
				lastboot: osinfo[0].last_boot_up_time.0.to_rfc3339(),	
				compname: osinfo[0].csname.clone(),
				freemem: freemem,
				totalmem: totalmem,
				usedmem: usedmem,
				cpuu: cpuinfo[0].percent_processor_time.parse::<u64>().unwrap()
			});
		}
		return None;
	}
	return None;
}

/// Route the returns a tera template, populating it with program variables gathered from wmi calls.
#[get("/")]
pub fn index() -> Template {

	let res = get_stats();
	let mut context = Context::new();

	match res {
		Some(value) => {
			context.insert("os_name", &value.osname);
			context.insert("build", &value.osbuild);
			context.insert("last_boot", &value.lastboot);
			context.insert("comp_name", &value.compname);
			context.insert("free_mem", &value.freemem);
			context.insert("used_mem", &value.usedmem);
			context.insert("installdate", &value.install);
			context.insert("architecture", &value.arch);
			context.insert("total_mem", &value.totalmem);
			context.insert("cpu_utilization", &value.cpuu);
		}
		None => {
			context.insert("os_name", "ERR");
			context.insert("build", "ERR");
			context.insert("last_boot", "ERR");
			context.insert("comp_name", "ERR");
			context.insert("free_mem", "ERR");
			context.insert("used_mem", "ERR");
			context.insert("installdate", "ERR");
			context.insert("architecture", "ERR");
			context.insert("total_mem", "ERR");
			context.insert("cpu_utilization", "ERR");
		}
	}

	Template::render("layout", &context)
}

#[get("/add")]
pub fn add() -> Template {
	let context = HashMap::<String, String>::new();

	Template::render("add",context)
}

/// Route for ajax call to dynamically update the html page with new data at a specific interval.
/// Uses wmi connection to obtain most recent data and returns the data as a JSON string using serde Serialize.
#[get("/stats", format = "application/json")]
pub fn stats() -> Json<Stats> {
	
	let res = get_stats();
	let json_string;

	match res {
		Some(value) => {
			json_string = Stats {
				osname: value.osname,
				osbuild: value.osbuild,
				arch: value.arch,
				install: value.install,
				lastboot: value.lastboot,	
				compname: value.compname,
				freemem: value.freemem,
				totalmem: value.totalmem,
				usedmem: value.usedmem,
				cpuu: value.cpuu
			};
		}
		None => {
			json_string = Stats {
				osname: "Err".to_string(),
				osbuild: "Err".to_string(),
				arch: "Err".to_string(),
				install: "".to_string(),
				lastboot: "".to_string(),	
				compname: "Err".to_string(),
				freemem: 0.0,
				totalmem: 0.0,
				usedmem: 0.0,
				cpuu: 0
			};
		}
	}

	Json(json_string)
}
