mod simulation;
use std::convert::Infallible;

use sysinfo::{System, SystemExt, CpuExt};
use warp::Filter;
use simulation::core::simulation::Simulation;

// fn main() {
//     let mut simulation = Simulation::new(4);
//     simulation.run();
// }

#[tokio::main]
async fn main() {
    println!("Blackout Engine starting...");

    let root = warp::path::end().and_then(root_handler);
    let health = warp::path("health").and_then(health_handler);
    let system = warp::path("system").and_then(system_handler);

    let routes = root.or(health).or(system);

    println!("Server running on port 8081");

    //run applicaiton
    tokio::spawn(async {
        let mut simulation = Simulation::new(4);
        simulation.run();
    });

    //for server 
    warp::serve(routes)
        .run(([0, 0, 0, 0], 8081))
        .await;
}

async fn root_handler() -> Result<impl warp::Reply, Infallible> {
    Ok("Blackout Engine is running ")
}

async fn health_handler() -> Result<impl warp::Reply, Infallible> {
    Ok("OK")
}

async fn system_handler() -> Result<impl warp::Reply, Infallible> {
    let mut sys = System::new_all();
    sys.refresh_all();

    let cpu_usage = sys.global_cpu_info().cpu_usage();
    let total_memory = sys.total_memory();
    let used_memory = sys.used_memory();

    let response = format!(
        "CPU: {:.2}% | Memory: {} / {} KB",
        cpu_usage, used_memory, total_memory
    );

    Ok(response)
}