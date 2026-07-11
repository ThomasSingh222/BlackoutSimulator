use crate::simulation::services::linux_collector::SystemSnapshot;

#[derive(Debug, Clone)]
pub struct HostObservation {
    pub hostname: String,
    pub cpu_usage: f32,
    pub memory_used_bytes: u64,
    pub memory_total_bytes: u64,
    pub disk_used_bytes: u64,
    pub disk_total_bytes: u64,
    pub load_average_one: f64,
    pub load_average_five: f64,
    pub load_average_fifteen: f64,
    pub uptime_seconds: u64,
}

impl HostObservation {
    pub fn from_snapshot(snapshot: &SystemSnapshot) -> Self {
        Self {
            hostname: snapshot.hostname.clone(),
            cpu_usage: snapshot.cpu_usage_percent,
            memory_used_bytes: snapshot.memory_used_bytes,
            memory_total_bytes: snapshot.memory_total_bytes,
            disk_used_bytes: snapshot.disk_used_bytes,
            disk_total_bytes: snapshot.disk_total_bytes,
            load_average_one: snapshot.load_average_one,
            load_average_five: snapshot.load_average_five,
            load_average_fifteen: snapshot.load_average_fifteen,
            uptime_seconds: snapshot.uptime_seconds,
        }
    }

    pub fn memory_usage_percent(&self) -> f32 {
        if self.memory_total_bytes == 0 {
            0.0
        } else {
            (self.memory_used_bytes as f32 / self.memory_total_bytes as f32) * 100.0
        }
    }

    pub fn disk_usage_percent(&self) -> f32 {
        if self.disk_total_bytes == 0 {
            0.0
        } else {
            (self.disk_used_bytes as f32 / self.disk_total_bytes as f32) * 100.0
        }
    }
}

#[derive(Debug, Clone)]
pub struct ContainerObservation {
    pub id: String,
    pub name: String,
    pub image: String,
    pub status: String,
    pub cpu_usage: f32,
    pub memory_usage: u64,
    pub ports: Vec<String>,
    pub networks: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct InfrastructureSnapshot {
    pub timestamp: u64,
    pub host_observation: Option<HostObservation>,
    pub container_observations: Vec<ContainerObservation>,
}

impl InfrastructureSnapshot {
    pub fn new(timestamp: u64) -> Self {
        Self {
            timestamp,
            host_observation: None,
            container_observations: Vec::new(),
        }
    }
}
