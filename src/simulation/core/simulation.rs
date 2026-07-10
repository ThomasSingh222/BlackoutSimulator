use crate::simulation::core::runtime::Runtime;

pub struct Simulation {
    runtime: Runtime,
}

impl Simulation {
    pub fn new(max_ticks: u32) -> Self {
        Self {
            runtime: Runtime::new(max_ticks),
        }
    }

    pub fn run(&mut self) {
        self.runtime.run();
    }
}
