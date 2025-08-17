"""
Simulation Environment and Shared State
"""
import simpy

class Blockchain:
    """A mocked representation of the QRASL blockchain state."""
    def __init__(self, env):
        self.env = env
        self.chain_height = 0
        self.problem_mempool = []
        self.solution_mempool = []
        self.proof_mempool = []

    def add_problem_to_mempool(self, problem):
        print(f"{self.env.now:.2f}: New problem {problem['id']} added to mempool.")
        self.problem_mempool.append(problem)

    def add_solution_to_mempool(self, solution):
        print(f"{self.env.now:.2f}: New solution for problem {solution['problem_id']} added to mempool.")
        self.solution_mempool.append(solution)

    def add_proof_to_mempool(self, proof):
        print(f"{self.env.now:.2f}: New proof for rollup added to mempool.")
        self.proof_mempool.append(proof)

class Network:
    """A mocked representation of the P2P network, handling latency."""
    def __init__(self, env, latency_mean, latency_std):
        self.env = env
        self.latency_mean = latency_mean
        self.latency_std = latency_std

    def get_latency(self):
        # In a real simulation, this would be more complex.
        return max(0, self.env.rand.normal(self.latency_mean, self.latency_std))

def create_simulation_environment():
    """Initializes and returns the simulation environment and shared state."""
    env = simpy.Environment()
    blockchain = Blockchain(env)
    return env, blockchain
