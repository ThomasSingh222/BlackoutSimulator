#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum NodeStatus {
    #[default]
    Healthy,
    Degraded,
    Failed,
}

#[derive(Debug, Clone, Default)]
pub struct Node {
    pub id: u32,
    pub name: String,
    pub status: NodeStatus,
    pub health: i32,
    pub latency_ms: u32,
    pub error_rate: f32,
    pub requests_per_second: u32,
    pub cpu_usage: u8,
    pub memory_usage: u8,
}

impl Node {
    pub fn new(id: u32, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            status: NodeStatus::Healthy,
            health: 100,
            latency_ms: 20,
            error_rate: 0.0,
            requests_per_second: 100,
            cpu_usage: 10,
            memory_usage: 20,
        }
    }
}
