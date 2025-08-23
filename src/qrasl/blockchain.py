from .block import Block
from .intent import Intent, Solver
import time

class Blockchain:
    """
    Manages the DAG of blocks and the pool of user intents.
    """
    def __init__(self, difficulty=4):
        self.blocks = {}
        self.intent_pool = []  # A mempool for pending user intents
        self.difficulty = difficulty
        self.create_genesis_block()

    def add_intent(self, intent):
        """
        Allows a user to submit an intent to the network's intent pool.
        """
        self.intent_pool.append(intent)
        print(f"Intent {intent.hash[:10]}... added to the pool.")
        return True

    def create_genesis_block(self):
        """
        Creates the first block in the DAG. The genesis block is special
        as it contains no solutions.
        """
        genesis_block = Block(index=0, solutions=[], parent_hashes=[])
        genesis_block.solution = 0
        genesis_block.hash = genesis_block.calculate_hash()
        self.blocks[genesis_block.hash] = genesis_block

    def get_tips(self):
        """Finds all blocks that are not parents of any other block."""
        all_parent_hashes = set(p_hash for block in self.blocks.values() for p_hash in block.parent_hashes)
        tip_hashes = set(self.blocks.keys()) - all_parent_hashes
        return [self.blocks[h] for h in tip_hashes]

    def solve_problem_for_block(self, block):
        """Finds a 'solution' for the block's computational problem (mining)."""
        print(f"Solving block problem...")
        start_time = time.time()
        target_prefix = '0' * self.difficulty
        block.solution = 0
        while not block.calculate_hash().startswith(target_prefix):
            block.solution += 1
        end_time = time.time()
        print(f"Block problem solved in {end_time - start_time:.4f}s. Solution: {block.solution}")
        return block

    def create_new_block(self, solver, max_intents=10):
        """
        This function is called by a block producer (like a Solver or Validator)
        to create a new block by processing intents from the pool.
        """
        if not self.intent_pool:
            print("No pending intents to process. Block not created.")
            return None

        # 1. Select intents to process
        intents_to_process = self.intent_pool[:max_intents]
        print(f"Processing {len(intents_to_process)} intents...")

        # 2. Use the solver to generate solutions
        solutions = solver.solve_intents(intents_to_process)
        if not solutions:
            print("Solver did not produce any solutions for the selected intents.")
            # Clear the intents that couldn't be solved to prevent getting stuck
            self.intent_pool = self.intent_pool[len(intents_to_process):]
            return None

        # 3. Select parent blocks (the current tips of the DAG)
        parent_blocks = self.get_tips()
        parent_hashes = [p.hash for p in parent_blocks]

        # 4. Create the new block with the solutions
        new_index = max(p.index for p in parent_blocks) + 1 if parent_blocks else 0
        new_block = Block(
            index=new_index,
            solutions=solutions,
            parent_hashes=parent_hashes
        )

        # 5. Solve the block's own computational problem (mining)
        solved_block = self.solve_problem_for_block(new_block)
        solved_block.hash = solved_block.calculate_hash()

        # 6. Add the finalized block to the DAG and update the intent pool
        self.blocks[solved_block.hash] = solved_block
        self.intent_pool = self.intent_pool[len(intents_to_process):]

        print(f"New block {solved_block.index} created with hash {solved_block.hash[:10]}...")
        return solved_block

    def is_chain_valid(self):
        """Validates the entire DAG blockchain."""
        if not self.blocks:
            return True
        target_prefix = '0' * self.difficulty
        for block_hash, block in self.blocks.items():
            if block_hash != block.calculate_hash():
                print(f"Validation Error: Hash of block {block.index} does not match content.")
                return False
            if block.index > 0 and not block.hash.startswith(target_prefix):
                print(f"Validation Error: Proof of Solution failed for block {block.index}.")
                return False
            for p_hash in block.parent_hashes:
                if p_hash not in self.blocks:
                    print(f"Validation Error: Parent block {p_hash} for block {block.index} not found.")
                    return False
                parent_block = self.blocks[p_hash]
                if parent_block.index >= block.index:
                    print(f"Validation Error: Cycle detected at block {block.index}.")
                    return False
        genesis_blocks = [b for b in self.blocks.values() if not b.parent_hashes]
        if len(genesis_blocks) != 1:
            print(f"Validation Error: Found {len(genesis_blocks)} genesis blocks. Expected 1.")
            return False
        return True
