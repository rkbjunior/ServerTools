use serde::Deserialize;
use serde::Serialize;

/// Struct that stores operating system information from a wmi call.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename = "Win32_OperatingSystem")]
#[serde(rename_all = "PascalCase")]
pub struct OperatingSystem {
    pub caption: String,                   //os_name
    pub buildnumber: String,               //build
    pub last_boot_up_time: String,         //last_boot
    pub csname: String,                    //comp_name
    pub freephysicalmemory: String,        //free_mem
    pub installdate: String,               //installdate
    pub osarchitecture: String,            //architecture
    pub total_visible_memory_size: String, //total_mem
}

/// Struct that stores cpu information from a wmi call.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename = "Win32_PerfFormattedData_Counters_ProcessorInformation")]
#[serde(rename_all = "PascalCase")]
pub struct ProcessUtilization {
    pub percent_processor_time: String,
}

/// Struct that stores service information from a wmi call.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename = "Win32_Service")]
#[serde(rename_all = "PascalCase")]
pub struct Win32Service {
    pub name: String,
    pub display_name: String,
    pub start_mode: String,
    pub state: String,
    pub process_id: u32,
    pub description: Option<String>,
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
    pub freemem: f64,
    pub totalmem: f64,
    pub usedmem: f64,
    pub cpuu: u64,
}

/// Implements Stats
impl Stats {
    /// Default constructor for stats.
    pub fn new() -> Stats {
        Stats {
            osname: "".to_string(),
            osbuild: "".to_string(),
            arch: "".to_string(),
            install: "".to_string(),
            lastboot: "".to_string(),
            compname: "".to_string(),
            freemem: 0.0,
            totalmem: 0.0,
            usedmem: 0.0,
            cpuu: 0,
        }
    }

    /// Set function for the cpuu field.
    pub fn set_cpu(&mut self, value: u64) {
        self.cpuu = value;
    }
    /// Set function for the osname field.
    pub fn set_osname(&mut self, value: String) {
        self.osname = value;
    }
    /// Set function for the osbuild field.
    pub fn set_osbuild(&mut self, value: String) {
        self.osbuild = value;
    }

    /// Set function for the arch field.
    pub fn set_arch(&mut self, value: String) {
        self.arch = value;
    }
    /// Set function for the install field.
    pub fn set_install(&mut self, value: String) {
        self.install = value;
    }
    /// Set function for the lastboot field.
    pub fn set_lastboot(&mut self, value: String) {
        self.lastboot = value;
    }
    /// Set function for the compname field.
    pub fn set_compname(&mut self, value: String) {
        self.compname = value;
    }
    /// Set function for the freemem field.
    pub fn set_freemem(&mut self, value: f64) {
        self.freemem = value;
    }
    /// Set function for the totalmem field.
    pub fn set_totalmem(&mut self, value: f64) {
        self.totalmem = value;
    }
    /// Set function for the usedmem field.
    pub fn set_usedmem(&mut self, value: f64) {
        self.usedmem = value;
    }
}
