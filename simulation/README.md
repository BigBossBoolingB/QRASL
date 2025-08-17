# QRASL/Chronos System Simulation

This directory contains a framework for a discrete-event simulation of the entire QRASL/Chronos ecosystem. The goal is to model the interactions between users, Heterogeneous Cognitive Nodes (HCNs), and the blockchain itself to validate the system's architecture, economic model, and performance characteristics under various conditions.

## Purpose

- **Economic Validation:** Test the tokenomics, including staking rewards, fee mechanisms, and burn rates, to ensure the economic stability and incentive compatibility of the system.
- **Performance Modeling:** Simulate transaction throughput, block propagation, and the time-to-solution for Chronos problems under different network loads.
- **Architectural Verification:** Validate the conceptual flow of information, from a user submitting a problem via the `ChronosInterface` to an HCN processing it and registering a proof.

## Framework

The simulation is built using Python and the `SimPy` library for discrete-event simulation.

- `main.py`: The main entry point for running the simulation. It sets up the environment, agents, and starts the simulation process.
- `config.py`: Contains all the configurable parameters for the simulation, such as the number of agents, simulation duration, and network properties.
- `environment.py`: Defines the simulation environment, including the shared state (mock blockchain, mempools) and the `SimPy` environment itself.
- `agents.py`: Defines the behavior of the different actors (agents) in the system, such as `UserAgent` and `HCNAgent`.

## How to Run (Conceptual)

1.  Install dependencies:
    ```bash
    pip install simpy
    ```
2.  Run the simulation:
    ```bash
    python main.py
    ```
3.  Analyze the output logs to assess the system's performance.
