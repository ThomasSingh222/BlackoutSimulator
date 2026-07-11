use crate::simulation::core::health_engine::{HealthCalculationParams, HealthEngine, StatusEngine};
use crate::simulation::core::metrics_engine::MetricsEngine;
use crate::simulation::engine::event_engine::EventEngine;
use crate::simulation::engine::tick_engine::TickEngine;
use crate::simulation::models::event::{EventType, SimulationEvent};
use crate::simulation::models::node::{Node, NodeStatus};
use crate::simulation::models::observation::{HostObservation, InfrastructureSnapshot};
use crate::simulation::models::world::WorldState;
use crate::simulation::services::linux_collector::LinuxCollector;
use std::collections::{HashSet, VecDeque};

pub struct Runtime {
    pub world: WorldState,
    pub tick_engine: TickEngine,
    pub event_engine: EventEngine,
    pub max_ticks: u32,
    pub linux_collector: LinuxCollector,
}

impl Runtime {
    pub fn new(max_ticks: u32) -> Self {
        Self {
            world: WorldState::new(),
            tick_engine: TickEngine::new(),
            event_engine: EventEngine::new(),
            max_ticks,
            linux_collector: LinuxCollector::new(),
        }
    }

    pub fn initialize(&mut self) {
        self.world.current_tick = 0;
        self.world.nodes = vec![
            Node::new(1, "CDN"),
            Node::new(2, "DNS"),
            Node::new(3, "API"),
            Node::new(4, "DB"),
        ];
        self.world.dependents.clear();
        self.world.dependents.insert(2, vec![1, 3]);
        self.world.dependents.insert(4, vec![3]);

        self.event_engine = EventEngine::new();
        self.event_engine.enqueue(SimulationEvent {
            id: 1,
            tick_to_execute: 2,
            event_type: EventType::LatencySpike,
            target_node_id: 2,
            severity: 50,
        });
        self.event_engine.enqueue(SimulationEvent {
            id: 2,
            tick_to_execute: 3,
            event_type: EventType::CpuSpike,
            target_node_id: 4,
            severity: 30,
        });
        self.event_engine.enqueue(SimulationEvent {
            id: 3,
            tick_to_execute: 5,
            event_type: EventType::MemorySpike,
            target_node_id: 3,
            severity: 20,
        });

        self.world.pending_events = self.event_engine.queue.clone();
        println!("Simulation Initialized");
        println!("World Loaded");
        println!("Current Tick: {}", self.world.current_tick);
        println!("Events Loaded: {}", self.world.pending_events.len());
    }

    pub fn run(&mut self) {
        println!("Simulation Started");
        self.initialize();

        let initial_snapshot = self.linux_collector.snapshot();
        println!(
            "Linux Collector: cpu={:.1}% mem={}MB host={} uptime={}s",
            initial_snapshot.cpu_usage_percent,
            initial_snapshot.memory_used_bytes / 1024 / 1024,
            initial_snapshot.hostname,
            initial_snapshot.uptime_seconds
        );

        while self.tick_engine.current_tick() < self.max_ticks {
            let next_tick = self.tick_engine.advance();
            self.world.current_tick = next_tick;

            println!("------------------------------------------");
            println!("Tick {}", self.world.current_tick);

            // Observation layer: scan infrastructure
            self.scan_infrastructure();

            // Process events
            println!("Processing Events...");
            if let Some(event) = self.event_engine.peek().cloned() {
                if event.tick_to_execute <= self.world.current_tick {
                    println!("{} Event Found", match event.event_type {
                        EventType::LatencySpike => "Latency Spike",
                        EventType::CpuSpike => "CPU Spike",
                        EventType::MemorySpike => "Memory Spike",
                        EventType::Unknown => "Unknown Event",
                    });
                    self.event_engine.dequeue();
                    self.world.pending_events.pop_front();
                    self.process_event(event);
                    println!("World Updated");
                } else {
                    println!("{} Event Found", match event.event_type {
                        EventType::LatencySpike => "Latency Spike",
                        EventType::CpuSpike => "CPU Spike",
                        EventType::MemorySpike => "Memory Spike",
                        EventType::Unknown => "Unknown Event",
                    });
                    println!("Event Deferred (Execution Tick: {})", event.tick_to_execute);
                    println!("World Updated");
                }
            } else {
                println!("No Events Executed");
            }

            if self.world.current_tick >= self.max_ticks {
                break;
            }
        }

        println!("------------------------------------------");
        println!("Simulation Complete");
    }

    fn scan_infrastructure(&mut self) {
        let linux_snapshot = self.linux_collector.snapshot();
        let host_observation = HostObservation::from_snapshot(&linux_snapshot);

        let mut infra_snapshot = InfrastructureSnapshot::new(self.world.current_tick as u64);
        infra_snapshot.host_observation = Some(host_observation.clone());

        // Calculate metrics from observations
        let metrics = MetricsEngine::calculate(&infra_snapshot);

        // Update all nodes with calculated health
        for node in &mut self.world.nodes {
            let health_params = HealthCalculationParams {
                cpu_usage: host_observation.cpu_usage,
                memory_usage: host_observation.memory_usage_percent(),
                latency_ms: node.latency_ms,
                error_rate: node.error_rate,
                disk_usage: host_observation.disk_usage_percent(),
            };

            let calculated_health = HealthEngine::calculate(&health_params);
            node.health = calculated_health as i32;
            node.status = if calculated_health >= 85.0 {
                NodeStatus::Healthy
            } else if calculated_health >= 70.0 {
                NodeStatus::Degraded
            } else {
                NodeStatus::Failed
            };
        }

        println!(
            "Infrastructure Scan: cpu={:.1}% mem={:.1}% disk={:.1}%",
            host_observation.cpu_usage,
            host_observation.memory_usage_percent(),
            host_observation.disk_usage_percent()
        );
    }

    pub fn process_event(&mut self, event: SimulationEvent) {
        self.apply_event_effect(event.target_node_id, event.event_type.clone(), event.severity, 0);

        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        visited.insert(event.target_node_id);

        if let Some(dependents) = self.world.dependents.get(&event.target_node_id) {
            for dependent_id in dependents.iter().copied() {
                queue.push_back((dependent_id, 1));
            }
        }

        while let Some((node_id, depth)) = queue.pop_front() {
            if !visited.insert(node_id) {
                continue;
            }

            let adjusted_severity = event.severity.saturating_sub(depth as u32 * 10).max(5);
            self.apply_event_effect(node_id, event.event_type.clone(), adjusted_severity, depth);

            if let Some(dependents) = self.world.dependents.get(&node_id) {
                for dependent_id in dependents.iter().copied() {
                    if !visited.contains(&dependent_id) {
                        queue.push_back((dependent_id, depth + 1));
                    }
                }
            }
        }
    }

    fn apply_event_effect(&mut self, node_id: u32, event_type: EventType, severity: u32, depth: usize) {
        let Some(node) = self.world.nodes.iter_mut().find(|candidate| candidate.id == node_id) else {
            return;
        };

        match event_type {
            EventType::LatencySpike => {
                node.latency_ms = node.latency_ms.saturating_add(severity / 2);
                node.error_rate = (node.error_rate + severity as f32 / 100.0).min(1.0);
            }
            EventType::CpuSpike => {
                let increase = (severity / 3) as u8;
                node.cpu_usage = node.cpu_usage.saturating_add(increase);
            }
            EventType::MemorySpike => {
                let increase = (severity / 3) as u8;
                node.memory_usage = node.memory_usage.saturating_add(increase);
            }
            EventType::Unknown => {}
        }

        let health_penalty = if depth == 0 {
            severity as i32 / 2
        } else {
            severity as i32 / 2 + depth as i32
        };
        node.health = (node.health - health_penalty).max(0);
        Self::recalculate_status(node);
    }

    fn recalculate_status(node: &mut Node) {
        let health = node.health;
        node.status = if health <= 25 {
            NodeStatus::Failed
        } else if health <= 60 {
            NodeStatus::Degraded
        } else {
            NodeStatus::Healthy
        };
    }
}

#[cfg(test)]
mod tests {
    use super::Runtime;
    use crate::simulation::models::event::{EventType, SimulationEvent};

    #[test]
    fn applying_latency_spike_reduces_health_and_propagates_to_dependents() {
        let mut runtime = Runtime::new(3);
        runtime.initialize();

        let event = SimulationEvent {
            id: 1,
            tick_to_execute: 1,
            event_type: EventType::LatencySpike,
            target_node_id: 2,
            severity: 50,
        };

        runtime.process_event(event);

        let dns = runtime.world.nodes.iter().find(|node| node.id == 2).unwrap();
        let api = runtime.world.nodes.iter().find(|node| node.id == 3).unwrap();

        assert!(dns.health < 100);
        assert!(api.health < 100);
    }
}
