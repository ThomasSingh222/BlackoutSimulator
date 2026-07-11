#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InfrastructureType {
    Region,
    Cluster,
    Host,
    Container,
    Service,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeStatus {
    Healthy,
    Warning,
    Degraded,
    Failed,
}

#[derive(Debug, Clone)]
pub struct InfrastructureNode {
    pub id: u32,
    pub node_type: InfrastructureType,
    pub name: String,
    pub parent_id: Option<u32>,
    pub children_ids: Vec<u32>,
    pub status: NodeStatus,
    pub health: f32,
    pub cpu_usage: f32,
    pub memory_usage: f32,
    pub latency_ms: u32,
}