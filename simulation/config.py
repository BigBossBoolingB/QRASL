"""
Simulation Configuration
"""

# Simulation duration in abstract time units
SIMULATION_DURATION = 1000

# Number of agents
NUM_USER_AGENTS = 10
NUM_HCN_AGENTS = 5

# Network properties (in time units)
AVG_BLOCK_TIME = 12
NETWORK_LATENCY_MEAN = 0.5
NETWORK_LATENCY_STD = 0.1

# HCN Agent properties
HCN_VALIDATOR_SPEED = 1.0  # How many blocks can be validated per time unit
HCN_SOLVER_SPEED = 0.5     # How many intents can be solved per time unit
HCN_PROVER_SPEED = 0.1     # How many ZK proofs can be generated per time unit

# User Agent properties
USER_PROBLEM_SUBMISSION_RATE = 0.2  # Probability of submitting a problem each time unit
