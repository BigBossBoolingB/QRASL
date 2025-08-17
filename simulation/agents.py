"""
Definition of Simulation Agents (Actors)
"""
import random
import itertools

class UserAgent:
    """Represents a user of the Chronos system."""
    def __init__(self, env, user_id, blockchain, submission_rate):
        self.env = env
        self.id = user_id
        self.blockchain = blockchain
        self.submission_rate = submission_rate
        self.action = env.process(self.run())

    def run(self):
        """The user's main behavior loop."""
        while True:
            # Decide whether to submit a problem in this time step
            if random.random() < self.submission_rate:
                problem = {
                    "id": f"problem-{self.id}-{self.env.now:.0f}",
                    "submitter": self.id
                }
                self.blockchain.add_problem_to_mempool(problem)

            # Wait for the next time step
            yield self.env.timeout(1)

class HCNAgent:
    """Represents a Heterogeneous Cognitive Node."""
    def __init__(self, env, hcn_id, blockchain):
        self.env = env
        self.id = hcn_id
        self.blockchain = blockchain
        self.action = env.process(self.run())

    def run(self):
        """The HCN's main behavior loop."""
        while True:
            # Simulate the three core functions in parallel
            yield self.env.process(self.validator_duty())
            yield self.env.process(self.solver_duty())
            yield self.env.process(self.prover_duty())

            # Wait for a short period before the next cycle
            yield self.env.timeout(0.5)

    def validator_duty(self):
        """Simulates the Validator role: creating blocks."""
        if self.blockchain.problem_mempool or self.blockchain.solution_mempool:
            print(f"{self.env.now:.2f}: HCN {self.id} (Validator) is creating a new block.")
            # Mock block creation time
            yield self.env.timeout(1)
            self.blockchain.chain_height += 1
            print(f"{self.env.now:.2f}: HCN {self.id} created block {self.blockchain.chain_height}.")
            # Clear mempools as part of block creation
            self.blockchain.problem_mempool.clear()
            self.blockchain.solution_mempool.clear()

    def solver_duty(self):
        """Simulates the Solver role: processing intents (problems)."""
        if self.blockchain.problem_mempool:
            problem = self.blockchain.problem_mempool[0] # Take the first problem
            print(f"{self.env.now:.2f}: HCN {self.id} (Solver) is solving problem {problem['id']}.")
            # Mock solving time
            yield self.env.timeout(random.uniform(1, 5))
            solution = {
                "problem_id": problem['id'],
                "solver": self.id
            }
            self.blockchain.add_solution_to_mempool(solution)

    def prover_duty(self):
        """Simulates the ZK-Miner role: generating proofs."""
        # This can be triggered by other events in a more complex simulation
        if random.random() < 0.1: # Sporadically generate a proof for a rollup
            print(f"{self.env.now:.2f}: HCN {self.id} (Prover) is generating a ZK proof.")
            # Mock proving time
            yield self.env.timeout(random.uniform(5, 10))
            proof = { "prover": self.id }
            self.blockchain.add_proof_to_mempool(proof)
