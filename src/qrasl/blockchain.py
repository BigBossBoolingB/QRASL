from .block import Block
import time

class Blockchain:
    """
    Manages the chain of blocks and the 'Proof of Solution' mechanism.
    """
    def __init__(self, difficulty=4):
        """
        Initializes the blockchain.
        The 'difficulty' defines the 'problem' for the solvers.
        """
        # The "problem" is to find a hash with this many leading zeros
        self.difficulty = difficulty
        self.chain = [self.create_genesis_block()]

    def create_genesis_block(self):
        """
        Creates the very first block. The genesis block is special
        and does not require solving.
        """
        genesis_block = Block(0, "Genesis Block", "0")
        genesis_block.solution = 0  # Pre-defined solution for the genesis block
        genesis_block.hash = genesis_block.calculate_hash()
        return genesis_block

    def get_latest_block(self):
        """
        Returns the most recent block in the chain.
        """
        return self.chain[-1]

    def solve_problem_for_block(self, block):
        """
        Finds a 'solution' that satisfies the network's problem.
        This is the conceptual "mining" or "useful work" process.
        """
        print(f"Solving problem for Block {block.index}...")
        start_time = time.time()

        target_prefix = '0' * self.difficulty
        block.solution = 0

        # Keep trying solutions until the hash has the required prefix
        while not block.calculate_hash().startswith(target_prefix):
            block.solution += 1

        end_time = time.time()
        solve_time = end_time - start_time
        print(f"Problem solved in {solve_time:.4f}s. Solution found: {block.solution}")

        return block

    def add_block(self, transactions):
        """
        Creates a new block, solves its problem, and adds it to the chain.
        """
        latest_block = self.get_latest_block()
        new_block = Block(
            index=latest_block.index + 1,
            transactions=transactions,
            previous_hash=latest_block.hash
        )

        # The core of the "Proof of Solution" is finding the solution
        solved_block = self.solve_problem_for_block(new_block)

        # Once solved, the hash is finalized
        solved_block.hash = solved_block.calculate_hash()

        self.chain.append(solved_block)
        return solved_block

    def is_solution_valid(self, block):
        """
        Checks if a block's hash satisfies the problem's requirements.
        """
        target_prefix = '0' * self.difficulty
        return block.hash.startswith(target_prefix)

    def is_chain_valid(self):
        """
        Determines if the entire blockchain is valid.
        """
        for i in range(1, len(self.chain)):
            current_block = self.chain[i]
            previous_block = self.chain[i-1]

            # 1. Check if the block's hash is still correct based on its data
            if current_block.hash != current_block.calculate_hash():
                print(f"Data integrity check failed for Block {current_block.index}.")
                return False

            # 2. Check if the chain link is broken
            if current_block.previous_hash != previous_block.hash:
                print(f"Chain link broken at Block {current_block.index}.")
                return False

            # 3. Check if the block's solution is valid for the problem
            if not self.is_solution_valid(current_block):
                print(f"Solution is not valid for Block {current_block.index}.")
                return False

        # 4. Check genesis block integrity and solution
        if self.chain[0].hash != self.chain[0].calculate_hash():
            print("Genesis block data integrity failed.")
            return False
        # Genesis block does not need to meet difficulty in this model, but could be enforced

        return True
