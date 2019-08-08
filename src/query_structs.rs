use wmi::WMIDateTime;
use serde::Deserialize;
use serde::Serialize;

/// Struct that stores operating system information from a wmi call.
#[derive(Deserialize, Debug,Clone)]
#[serde(rename = "Win32_OperatingSystem")]
#[serde(rename_all = "PascalCase")]
pub struct OperatingSystem {
	pub caption: String,				//os_name
	pub buildnumber: String,			//build
	pub last_boot_up_time: String,	//last_boot
	pub csname: String,					//comp_name
	pub freephysicalmemory: String,		//free_mem
	pub installdate: String,		//installdate
	pub osarchitecture: String,			//architecture
	pub total_visible_memory_size: String,	//total_mem
}

/// Struct that stores cpu information from a wmi call.
#[derive(Deserialize, Debug, Clone)]
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
#[derive(Serialize, Deserialize, Default)]
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

impl Stats {
	pub fn new() -> Stats {
		Stats {
			osname: "".to_string(),
			osbuild: "".to_string(),
			arch: "".to_string(),
			install: "".to_string(),
			lastboot: "".to_string(),
			compname: "".to_string(),
			freemem : 0.0,
			totalmem: 0.0,
			usedmem: 0.0,
			cpuu: 0,
		}
	}

	pub fn set_cpu(&mut self, value: u64) {
		self.cpuu = value;
	}
	pub fn set_osname(&mut self, value: String) {
		self.osname = value;
	}
	pub fn set_osbuild(&mut self, value: String) {
		self.osbuild = value;
	}
	pub fn set_arch(&mut self, value: String) {
		self.arch = value;
	}
	pub fn set_install(&mut self, value: String) {
		self.install = value;
	}
	pub fn set_lastboot(&mut self, value: String) {
		self.lastboot = value;
	}
	pub fn set_compname(&mut self, value: String) {
		self.compname = value;
	}
	pub fn set_freemem(&mut self, value: f64) {
		self.freemem = value;
	}
	pub fn set_totalmem(&mut self, value: f64) {
		self.totalmem = value;
	}
	pub fn set_usedmem(&mut self, value: f64) {
		self.usedmem = value;
	}
}