use crate::simulation::models::event::SimulationEvent;
use crate::simulation::models::metrics::Metrics;
use crate::simulation::models::node::Node;
use std::collections::{HashMap, VecDeque};

// The simulation engine owns the world because the world is the shared source of truth
// for tick time, active nodes, queued work, and aggregated metrics. Keeping ownership
// here ensures a deterministic view of the entire distributed system instead of spreading
// state across independent node containers.
#[derive(Debug, Clone, Default)]
pub struct WorldState {
    pub current_tick: u32,
    pub nodes: Vec<Node>,
    pub pending_events: VecDeque<SimulationEvent>,
    pub dependents: HashMap<u32, Vec<u32>>,
    pub metrics: Metrics,
}

impl WorldState {
    pub fn new() -> Self {
        Self {
            current_tick: 0,
            nodes: Vec::new(),
            pending_events: VecDeque::new(),
            dependents: HashMap::new(),
            metrics: Metrics::default(),
        }
    }
}
