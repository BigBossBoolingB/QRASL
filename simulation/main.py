"""
Main Simulation Runner
"""
import simpy
from config import *
from environment import create_simulation_environment
from agents import UserAgent, HCNAgent

def run_simulation():
    """Sets up and runs the QRASL/Chronos simulation."""
    print("--- Starting QRASL/Chronos System Simulation ---")

    # Create the environment
    env, blockchain = create_simulation_environment()

    # Create User agents
    for i in range(NUM_USER_AGENTS):
        UserAgent(env, user_id=f"user_{i}", blockchain=blockchain, submission_rate=USER_PROBLEM_SUBMISSION_RATE)

    # Create HCN agents
    for i in range(NUM_HCN_AGENTS):
        HCNAgent(env, hcn_id=f"hcn_{i}", blockchain=blockchain)

    # Run the simulation
    print(f"\nRunning simulation for {SIMULATION_DURATION} time units...")
    env.run(until=SIMULATION_DURATION)

    print("\n--- Simulation Finished ---")
    print(f"Final Block Height: {blockchain.chain_height}")
    print(f"Problems left in mempool: {len(blockchain.problem_mempool)}")
    print(f"Solutions left in mempool: {len(blockchain.solution_mempool)}")

if __name__ == "__main__":
    run_simulation()
