use sysinfo::{Disks, System};

pub struct SystemSnapshot {
    pub cpu_usage_percent: f32,
    pub memory_used_bytes: u64,
    pub memory_total_bytes: u64,
    pub disk_used_bytes: u64,
    pub disk_total_bytes: u64,
    pub load_average_one: f64,
    pub load_average_five: f64,
    pub load_average_fifteen: f64,
    pub hostname: String,
    pub uptime_seconds: u64,
} 
pub struct LinuxCollector;

impl LinuxCollector {
    pub fn new() -> Self {
        Self
    }

    pub fn snapshot(&self) -> SystemSnapshot {
        let mut system = System::new_all();
        system.refresh_all();

        let cpu_usage_percent = system.global_cpu_usage();
        let memory_used_bytes = system.used_memory();
        let memory_total_bytes = system.total_memory();

        let mut disks = Disks::new_with_refreshed_list();
        let disk_used_bytes = disks
            .iter()
            .map(|disk| disk.total_space() - disk.available_space())
            .sum();
        let disk_total_bytes = disks.iter().map(|disk| disk.total_space()).sum();

        let load_average = System::load_average();
        let hostname = System::host_name().unwrap_or_else(|| "Unknown".to_string());
        let uptime_seconds = System::uptime();

        SystemSnapshot {
            cpu_usage_percent,
            memory_used_bytes,
            memory_total_bytes,
            disk_used_bytes,
            disk_total_bytes,
            load_average_one: load_average.one,
            load_average_five: load_average.five,
            load_average_fifteen: load_average.fifteen,
            hostname,
            uptime_seconds,
        }
    }
}