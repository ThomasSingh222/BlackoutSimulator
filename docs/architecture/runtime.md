# Runtime Architecture Overview

## Q1. What is a Simulation?

A Simulation represents one execution of a virtual distributed system over simulated time. It is a deterministic experiment in which the runtime advances a model of the system step by step, preserving internal state between ticks so that the same inputs produce the same outputs. In this project, a simulation is the top-level unit of execution that owns the world, the scheduler, and the event loop.

## Q2. What is a Tick?

A Tick is one unit of simulated progress in the runtime. It is not necessarily one real-world second; it is a logical time slice chosen by the simulation designer. The duration of a tick can be changed later by adjusting the runtime configuration, because the engine should reason in abstract time rather than wall-clock time. A single tick does not have to execute the same exact pipeline every time in all future designs, but in this implementation the tick loop advances time, evaluates pending events that are due, and updates the world snapshot.

## Q3. What is World State?

World State is the authoritative snapshot of the simulation at a given moment.

World State
↓
Current Tick
↓
Nodes
↓
Metrics
↓
Pending Events

This means the world stores the current simulation time, the collection of node states, aggregate metrics, and the queue of events scheduled for future execution. The world should be the single source of truth, which helps keep the engine deterministic and makes debugging, replay, and inspection easier.

## Q4. Engine Lifecycle

Created
↓
Initialized
↓
Running
↓
Paused
↓
Stopped

The engine begins in the Created state, transitions to Initialized after dependencies are prepared, enters Running while the simulation loop is active, can move to Paused for inspection or controlled interruption, and ends in Stopped when the run completes or is terminated.

## Design Notes

The Simulation Engine should own the World instead of owning Nodes directly because the world contains the cross-cutting state shared by all subsystems: current tick, event queue, node inventory, and metrics. If nodes were the primary owner, the engine would lose a unified view of the entire distributed system and would be forced to coordinate state across multiple independent structures. By keeping a single world object as the source of truth, the runtime can make decisions based on the complete simulation context and preserve deterministic behavior across all subsystems.
