use crate::simulation::models::region::NodeStatus;

#[derive(Debug, Clone)]
pub struct HealthCalculationParams {
    pub cpu_usage: f32,
    pub memory_usage: f32,
    pub latency_ms: u32,
    pub error_rate: f32,
    pub disk_usage: f32,
}

pub struct HealthEngine;

impl HealthEngine {
    pub fn calculate(params: &HealthCalculationParams) -> f32 {
        let mut health = 100.0;

        // CPU penalty: 90%+ usage reduces health
        if params.cpu_usage > 90.0 {
            health -= (params.cpu_usage - 90.0) * 0.5;
        } else if params.cpu_usage > 75.0 {
            health -= (params.cpu_usage - 75.0) * 0.2;
        }

        // Memory penalty: 85%+ usage reduces health
        if params.memory_usage > 85.0 {
            health -= (params.memory_usage - 85.0) * 0.5;
        } else if params.memory_usage > 70.0 {
            health -= (params.memory_usage - 70.0) * 0.2;
        }

        // Latency penalty: every 50ms adds 5 points penalty
        let latency_penalty = (params.latency_ms as f32 / 50.0) * 5.0;
        health -= latency_penalty.min(20.0);

        // Error rate penalty: 0.5% error rate = 10 points
        health -= params.error_rate * 2000.0;

        // Disk penalty: 90%+ usage reduces health
        if params.disk_usage > 90.0 {
            health -= (params.disk_usage - 90.0) * 0.3;
        }

        // Clamp health between 0 and 100
        health.max(0.0).min(100.0)
    }
}

pub struct StatusEngine;

impl StatusEngine {
    pub fn derive(health: f32) -> NodeStatus {
        if health >= 85.0 {
            NodeStatus::Healthy
        } else if health >= 70.0 {
            NodeStatus::Warning
        } else if health >= 40.0 {
            NodeStatus::Degraded
        } else {
            NodeStatus::Failed
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn health_calculation_high_cpu() {
        let params = HealthCalculationParams {
            cpu_usage: 95.0,
            memory_usage: 50.0,
            latency_ms: 10,
            error_rate: 0.0,
            disk_usage: 50.0,
        };
        let health = HealthEngine::calculate(&params);
        assert!(health < 100.0);
    }

    #[test]
    fn status_derives_from_health() {
        assert_eq!(StatusEngine::derive(90.0), NodeStatus::Healthy);
        assert_eq!(StatusEngine::derive(75.0), NodeStatus::Warning);
        assert_eq!(StatusEngine::derive(50.0), NodeStatus::Degraded);
        assert_eq!(StatusEngine::derive(20.0), NodeStatus::Failed);
    }
}
