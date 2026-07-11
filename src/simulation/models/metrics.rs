#[derive(Debug, Clone, Default)]
pub struct Metrics {
    pub availability: f32,
    pub latency: u32,
    pub error_rate: f32,
}
