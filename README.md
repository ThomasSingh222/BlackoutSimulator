# Digital Blackout Simulator

## Project Vision

The Digital Blackout Simulator is a deterministic runtime for modeling distributed-system failures and recovery scenarios over simulated time. The platform is designed to help engineers understand how latency, CPU pressure, and memory pressure affect node health across a virtual infrastructure without relying on real-time execution.

## Architecture Diagram

```text
Simulation
├── Core Runtime
│   ├── TickEngine
│   ├── EventEngine
│   └── WorldState
├── Models
│   ├── Node
│   ├── Metrics
│   └── SimulationEvent
└── Services
    ├── HealthService
    └── LoggingService
```

## Folder Structure

```text
src/
  main.rs
  simulation/
    core/
      mod.rs
      simulation.rs
      runtime.rs
      scheduler.rs
    engine/
      mod.rs
      tick_engine.rs
      event_engine.rs
    models/
      mod.rs
      node.rs
      metrics.rs
      event.rs
      world.rs
    services/
      mod.rs
      health_service.rs
      logging_service.rs
    utils/
```

## Current Sprint Status

- Core runtime scaffold is in place.
- Tick engine and event queue are implemented.
- The simulation loop is wired to advance ticks and process queued events.
- Failure propagation is intentionally deferred to a later sprint.
