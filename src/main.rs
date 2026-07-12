mod simulation; 
use std::net::SocketAddr;
use warp::Filter;

use simulation::core::simulation::Simulation;


#[tokio::main]
async fn main() {
    let routes = warp::any().map(|| "BC Simulation Chal peya OA!!");

    let mut simulation = Simulation::new(4);
    tokio::task::spawn_blocking(move || simulation.run());

    let addr: SocketAddr = "127.0.0.1:3030".parse().unwrap();
    warp::serve(routes).run(addr).await;  
    
}