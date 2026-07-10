use std::collections::VecDeque;

use crate::simulation::models::event::SimulationEvent;

// A VecDeque is the right primitive here because it preserves FIFO ordering while
// offering efficient push_back and pop_front operations for event scheduling.
#[derive(Debug, Clone, Default)]
pub struct EventEngine {
    pub queue: VecDeque<SimulationEvent>,
}

impl EventEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn enqueue(&mut self, event: SimulationEvent) {
        self.queue.push_back(event);
    }

    pub fn dequeue(&mut self) -> Option<SimulationEvent> {
        self.queue.pop_front()
    }

    pub fn peek(&self) -> Option<&SimulationEvent> {
        self.queue.front()
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::EventEngine;
    use crate::simulation::models::event::{EventType, SimulationEvent};

    #[test]
    fn enqueue_dequeue_preserves_fifo_order() {
        let mut queue = EventEngine::new();
        queue.enqueue(SimulationEvent {
            id: 1,
            tick_to_execute: 2,
            event_type: EventType::LatencySpike,
            target_node_id: 1,
            severity: 1,
        });
        queue.enqueue(SimulationEvent {
            id: 2,
            tick_to_execute: 3,
            event_type: EventType::CpuSpike,
            target_node_id: 2,
            severity: 2,
        });

        let first = queue.dequeue().unwrap();
        let second = queue.dequeue().unwrap();

        assert_eq!(first.id, 1);
        assert_eq!(second.id, 2);
    }

    #[test]
    fn empty_queue_reports_true_when_no_events_exist() {
        let queue = EventEngine::new();
        assert!(queue.is_empty());
    }
}
