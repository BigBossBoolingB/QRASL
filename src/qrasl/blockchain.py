from .block import Block
from .intent import Intent, Solver
import time

class Blockchain:
    """
    Manages the DAG of blocks and the pool of user intents.
    The block creation logic is now updated to handle cross-shard messages.
    """
    def __init__(self, difficulty=4):
        self.blocks = {}
        self.intent_pool = []
        self.difficulty = difficulty
        self.create_genesis_block()

    def add_intent(self, intent):
        """Adds an intent to the pool."""
        self.intent_pool.append(intent)
        print(f"Intent {intent.hash[:10]}... added to the pool.")
        return True

    def create_genesis_block(self):
        """Creates the first block, with no solutions or processed messages."""
        genesis_block = Block(
            index=0,
            solutions=[],
            parent_hashes=[],
            processed_messages=[] # New field
        )
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

    def create_new_block(self, solver, incoming_messages, max_intents=10):
        """
        Creates a new block by processing intents and incoming messages.
        Returns the new block and any generated outgoing messages.
        """
        # 1. Process intents from the pool
        intents_to_process = self.intent_pool[:max_intents]
        local_solutions, outgoing_messages = solver.solve_intents(intents_to_process)
        print(f"Processing {len(intents_to_process)} intents -> {len(local_solutions)} local solutions, {len(outgoing_messages)} outgoing messages.")

        # Also process incoming messages. For this simulation, we'll just record them.
        # A real implementation would have logic here to credit users, etc.
        processed_messages = incoming_messages
        print(f"Processing {len(processed_messages)} incoming messages.")

        # If there's nothing to do, don't create a block.
        if not local_solutions and not processed_messages:
            print("No work to do. Block not created.")
            self.intent_pool = self.intent_pool[len(intents_to_process):] # Still clear the (unsolvable) intents
            return None, []

        # 2. Create the block with solutions and processed messages
        parent_blocks = self.get_tips()
        parent_hashes = [p.hash for p in parent_blocks]
        new_index = max(p.index for p in parent_blocks) + 1 if parent_blocks else 0
        new_block = Block(
            index=new_index,
            solutions=local_solutions,
            parent_hashes=parent_hashes,
            processed_messages=processed_messages
        )

        # 3. Solve the block's own problem and finalize it
        solved_block = self.solve_problem_for_block(new_block)
        solved_block.hash = solved_block.calculate_hash()

        # 4. Add to DAG and clean up intent pool
        self.blocks[solved_block.hash] = solved_block
        self.intent_pool = self.intent_pool[len(intents_to_process):]

        print(f"New block {solved_block.index} created with hash {solved_block.hash[:10]}...")
        return solved_block, outgoing_messages

    def is_chain_valid(self):
        """Validates the entire DAG blockchain."""
        # This method doesn't need to change, as the hash calculation in Block
        # already covers the new `processed_messages` field.
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
