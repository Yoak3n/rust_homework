
use sysinfo::{Disks,System};

pub struct DeviceInfo {
    pub name: String,
    pub cpu_usage: f64,
    pub disk_usage: f64,
    pub memory_usage: f64,
    pub device_type: String,
}
pub fn new() -> DeviceInfo {
    // 初始化设备信息
    let mut d = DeviceInfo {
        name: String::from("Device"),
        cpu_usage: 0.0,
        disk_usage: 0.0,
        memory_usage: 0.0,
        device_type: String::from("Unknown"),
    };
    d.get_system_info();
    d
}

// enum ComputerType {
//     Windows,
//     Linux,
//     MacOS,
//     Unknown
// }

// enum PhoneType {
//     Android,
//     IOS,
//     Harmony,
//     Unknown
// }
// enum DeviceType {
//     Computer,
//     Phone,
//     Nas
// }


 impl DeviceInfo{
    pub fn get_system_info(&mut self) {
        // 获取系统信息
        let mut sys = sysinfo::System::new_all();
        sys.refresh_all();
        let system_name = System::name().expect("Unknown OS");
        match system_name.as_str() {
            "Windows" => self.device_type = String::from("Computer"),
            "Linux" => self.device_type = String::from("Computer"),
            "Darwin" => self.device_type = String::from("Computer"),
            "Android" => self.device_type = String::from("Phone"),
            "iOS" => self.device_type = String::from("Phone"),
            "Harmony" => self.device_type = String::from("Phone"),
            _ => self.device_type = String::from("Unknown")
        }
        self.name = System::host_name().expect("Unknown Hostname");
        self.memory_usage = get_memory_usage(&sys);
        self.cpu_usage = get_cpu_usage(&mut sys);
        self.disk_usage = get_disk_usage();
    }

    
 }

fn get_memory_usage(sys:&System) -> f64{
    // 获取内存使用情况
    sys.used_memory() as f64 / sys.total_memory() as f64
}
fn get_cpu_usage(sys:&mut System) -> f64{
    sys.refresh_cpu();
    sys.global_cpu_info().cpu_usage() as f64 
}
fn get_disk_usage() -> f64 {
    let disks = Disks::new_with_refreshed_list();
    let mut usage_space :u64 = 0 ;
    let mut total_space :u64 = 0 ;
    for disk in &disks{
        total_space = disk.total_space() + total_space;
        // 获取磁盘使用情况
        let space = disk.total_space() - disk.available_space();
        usage_space = space + usage_space;
    }
    usage_space as f64 / total_space as f64
}
