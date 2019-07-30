use wmi::{WMIDateTime};
use serde::Deserialize;
use serde::Serialize;

/// Struct that stores operating system information from a wmi call.
#[derive(Deserialize, Debug)]
#[serde(rename = "Win32_OperatingSystem")]
#[serde(rename_all = "PascalCase")]
pub struct OperatingSystem {
	pub caption: String,				//os_name
	pub buildnumber: String,			//build
	pub last_boot_up_time: WMIDateTime,	//last_boot
	pub csname: String,					//comp_name
	pub freephysicalmemory: String,		//free_mem
	pub installdate: WMIDateTime,		//installdate
	pub osarchitecture: String,			//architecture
	pub total_visible_memory_size: String,	//total_mem
}

/// Struct that stores cpu information from a wmi call.
#[derive(Deserialize, Debug)]
#[serde(rename = "Win32_PerfFormattedData_Counters_ProcessorInformation")]
#[serde(rename_all = "PascalCase")]
pub struct ProcessUtilization {
	pub percent_processor_time: String
}

/// Struct that stores operating system information from a wmi call.
#[derive(Deserialize, Debug)]
#[serde(rename = "Win32_Service")]
#[serde(rename_all = "PascalCase")]
pub struct Win32Service {
	pub AcceptPause: bool,
	pub AcceptStop: bool,
	pub Caption: String,
	pub CheckPoint: u32,
	pub CreationClassName: String,
	pub DelayedAutoStart: bool,
	pub Description: String,
	pub DesktopInteract: bool,
	pub DisplayName: String,
	pub ErrorControl: String,
	pub ExitCode: u32,
	pub InstallDate: WMIDateTime,
	pub Name: String,
	pub PathName: String,
	pub ProcessId: u32,
	pub ServiceSpecificExitCode: u32,
	pub ServiceType: String,
	pub Started: bool,
	pub StartMode: String,
	pub StartName: String,
	pub State: String,
	pub Status: String,
	pub SystemCreationClassname: String,
	pub SystemName: String,
	pub TagId: u32,
	pub WaitHint: u32,
}

/// Struct that consolidates multiple wmi calls and stores current pc statistics
/// * `freemem` - A value the represents the computers free memory.
/// * `totalmem` - A value that represents the computers total memory.
/// * `usedmem` - A value that represents the amount of memory in use.
/// * `cpuu` - A value that represents the current cpu utilization.
#[derive(Serialize, Deserialize)]
pub struct Stats {
	pub osname: String,
	pub osbuild: String,
	pub arch: String,
	pub install: String,
	pub lastboot: String,
	pub compname: String,
	pub freemem : f64,
	pub totalmem: f64,
	pub usedmem: f64,
	pub cpuu: u64
}