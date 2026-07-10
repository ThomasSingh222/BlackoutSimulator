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
    pub error_rate: f32,
}

impl InfrastructureNode {
    pub fn new(
        id: u32,
        node_type: InfrastructureType,
        name: &str,
        parent_id: Option<u32>,
    ) -> Self {
        Self {
            id,
            node_type,
            name: name.to_string(),
            parent_id,
            children_ids: Vec::new(),
            status: NodeStatus::Healthy,
            health: 100.0,
            cpu_usage: 0.0,
            memory_usage: 0.0,
            latency_ms: 0,
            error_rate: 0.0,
        }
    }

    pub fn add_child(&mut self, child_id: u32) {
        if !self.children_ids.contains(&child_id) {
            self.children_ids.push(child_id);
        }
    }
}
