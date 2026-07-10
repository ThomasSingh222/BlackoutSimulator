#[derive(Debug, Clone, Default)]
pub struct TickEngine {
    current: u32,
}

impl TickEngine {
    pub fn new() -> Self {
        Self { current: 0 }
    }

    pub fn current_tick(&self) -> u32 {
        self.current
    }

    pub fn advance(&mut self) -> u32 {
        self.current += 1;
        self.current
    }

    pub fn reset(&mut self) {
        self.current = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::TickEngine;

    #[test]
    fn starts_at_zero() {
        let engine = TickEngine::new();
        assert_eq!(engine.current_tick(), 0);
    }

    #[test]
    fn increments_correctly() {
        let mut engine = TickEngine::new();
        assert_eq!(engine.advance(), 1);
        assert_eq!(engine.current_tick(), 1);
    }

    #[test]
    fn reset_brings_tick_back_to_zero() {
        let mut engine = TickEngine::new();
        engine.advance();
        engine.reset();
        assert_eq!(engine.current_tick(), 0);
    }
}
