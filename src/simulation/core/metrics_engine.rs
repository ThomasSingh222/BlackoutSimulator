use crate::simulation::models::observation::InfrastructureSnapshot;

#[derive(Debug, Clone)]
pub struct InfrastructureMetrics {
    pub timestamp: u64,
    pub average_cpu: f32,
    pub average_memory: f32,
    pub average_disk: f32,
    pub healthy_containers: u32,
    pub degraded_containers: u32,
    pub failed_containers: u32,
    pub total_containers: u32,
    pub average_latency_ms: u32,
}

pub struct MetricsEngine;

impl MetricsEngine {
    pub fn calculate(snapshot: &InfrastructureSnapshot) -> InfrastructureMetrics {
        let mut metrics = InfrastructureMetrics {
            timestamp: snapshot.timestamp,
            average_cpu: 0.0,
            average_memory: 0.0,
            average_disk: 0.0,
            healthy_containers: 0,
            degraded_containers: 0,
            failed_containers: 0,
            total_containers: snapshot.container_observations.len() as u32,
            average_latency_ms: 0,
        };

        if let Some(host) = &snapshot.host_observation {
            metrics.average_cpu = host.cpu_usage;
            metrics.average_memory = host.memory_usage_percent();
            metrics.average_disk = host.disk_usage_percent();
        }

        if metrics.total_containers > 0 {
            let total_container_cpu: f32 = snapshot
                .container_observations
                .iter()
                .map(|c| c.cpu_usage)
                .sum();

            metrics.average_cpu = total_container_cpu / metrics.total_containers as f32;
        }

        metrics
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn metrics_calculation_empty_snapshot() {
        let snapshot = InfrastructureSnapshot::new(100);
        let metrics = MetricsEngine::calculate(&snapshot);

        assert_eq!(metrics.timestamp, 100);
        assert_eq!(metrics.total_containers, 0);
    }
}
