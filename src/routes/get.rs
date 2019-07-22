use rocket_contrib::templates::Template;
use tera::Context;
use wmi::{COMLibrary, WMIConnection};
use serde::Deserialize;
use wmi::WMIDateTime;

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

#[derive(Deserialize, Debug)]
#[serde(rename = "Win32_PerfFormattedData_Counters_ProcessorInformation")]
#[serde(rename_all = "PascalCase")]
struct ProcessUtilization {
	percent_processor_time: String
}

const GIGACONVERSION: f64 = 1048576.0;

fn convert_bytes_string_to_gigabytes_string(byte_string: String) -> String {
	//possibly improve this to detect size and return appropriate units
	//such as MB, GB or KB

	let mut gigabytes = byte_string.parse::<f64>().unwrap();

	gigabytes /= GIGACONVERSION;

	format!("{:.*}",2,gigabytes.to_string())
}

#[get("/")]
pub fn index() -> Template {

	let com_con = COMLibrary::new().unwrap();
	let wmi_con = WMIConnection::new(com_con.into()).unwrap();

	let results: Vec<OperatingSystem> = wmi_con.query().unwrap();
	let cpu: Vec<ProcessUtilization> = wmi_con.query().unwrap();

	let mut context = Context::new();

	let freemem = convert_bytes_string_to_gigabytes_string( results[0].freephysicalmemory.clone());
	let totalmem = convert_bytes_string_to_gigabytes_string( results[0].total_visible_memory_size.clone());

	context.insert("os_name", &results[0].caption);
	context.insert("build", &results[0].buildnumber);
	context.insert("local_date", &results[0].localdatetime);
	context.insert("last_boot", &results[0].last_boot_up_time);
	context.insert("comp_name", &results[0].csname);
	context.insert("desc", &results[0].description);
	context.insert("free_mem", &freemem);
	context.insert("installdate", &results[0].installdate);
	context.insert("num_of_processes", &results[0].numberofprocesses);
	context.insert("num_of_users", &results[0].numberofusers);
	context.insert("sku", &results[0].operatingsystemsku);
	context.insert("architecture", &results[0].osarchitecture);
	context.insert("total_mem", &totalmem);
	context.insert("cpu_utilization", &cpu[0].percent_processor_time);

	Template::render("layout", &context)
	
}