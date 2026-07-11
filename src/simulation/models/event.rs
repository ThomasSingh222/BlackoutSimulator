#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventType {
    LatencySpike,
    CpuSpike,
    MemorySpike,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationEvent {
    pub id: u64,
    pub tick_to_execute: u32,
    pub event_type: EventType,
    pub target_node_id: u32,
    pub severity: u32,
}
