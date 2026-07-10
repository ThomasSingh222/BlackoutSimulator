mod simulation;

use simulation::core::simulation::Simulation;

fn main() {
    let mut simulation = Simulation::new(4);
    simulation.run();
}