from .block import Block
import time

class Blockchain:
    """
    Manages the collection of blocks in a DAG structure.
    """
    def __init__(self, difficulty=4):
        """
        Initializes the blockchain DAG.
        The 'difficulty' defines the 'problem' for the solvers.
        """
        self.blocks = {}  # Using a dictionary {hash: block} to store all blocks
        self.difficulty = difficulty
        self.create_genesis_block()

    def create_genesis_block(self):
        """
        Creates the very first block in the DAG. The genesis block has no parents.
        """
        # The genesis block has an empty list of parent hashes
        genesis_block = Block(index=0, transactions="Genesis Block", parent_hashes=[])
        genesis_block.solution = 0  # Pre-defined solution
        genesis_block.hash = genesis_block.calculate_hash()
        self.blocks[genesis_block.hash] = genesis_block

    def get_tips(self):
        """
        Finds all blocks that are not parents of any other block (the "tips" of the DAG).
        These are the blocks that new blocks can be built upon.
        """
        all_parent_hashes = set()
        for block in self.blocks.values():
            for parent_hash in block.parent_hashes:
                all_parent_hashes.add(parent_hash)

        tip_hashes = set(self.blocks.keys()) - all_parent_hashes
        return [self.blocks[h] for h in tip_hashes]

    def solve_problem_for_block(self, block):
        """
        Finds a 'solution' that satisfies the network's problem (Proof of Solution).
        This method is unchanged from the previous implementation.
        """
        print(f"Solving problem for Block...")
        start_time = time.time()

        target_prefix = '0' * self.difficulty
        block.solution = 0

        while not block.calculate_hash().startswith(target_prefix):
            block.solution += 1

        end_time = time.time()
        print(f"Problem solved in {end_time - start_time:.4f}s. Solution: {block.solution}")

        return block

    def add_block(self, transactions, parent_hashes):
        """
        Creates a new block, referencing one or more parent blocks.
        """
        # Ensure all specified parents exist in our collection of blocks.
        for p_hash in parent_hashes:
            if p_hash not in self.blocks:
                raise ValueError(f"Parent block with hash {p_hash} not found.")

        parent_blocks = [self.blocks[h] for h in parent_hashes]
        # The index (or 'height') of the new block can be 1 + the max index of its parents.
        new_index = max(p.index for p in parent_blocks) + 1 if parent_blocks else 0

        new_block = Block(
            index=new_index,
            transactions=transactions,
            parent_hashes=parent_hashes
        )

        solved_block = self.solve_problem_for_block(new_block)
        solved_block.hash = solved_block.calculate_hash()

        # Add the new, solved block to our collection
        self.blocks[solved_block.hash] = solved_block
        return solved_block

    def is_chain_valid(self):
        """
        Validates the entire DAG by checking multiple properties for every block.
        1.  Data Integrity: The block's stored hash matches its calculated hash.
        2.  Proof of Solution: The block's hash meets the difficulty requirement.
        3.  Parent Links: All parent hashes point to existing blocks.
        4.  Acyclicity: A block's index must be greater than all its parents' indices.
        """
        if not self.blocks:
            return True

        target_prefix = '0' * self.difficulty

        for block_hash, block in self.blocks.items():
            # 1. Verify data integrity by recalculating the hash
            if block_hash != block.calculate_hash():
                print(f"Validation Error: Hash of block {block.index} does not match its content.")
                return False

            # 2. Verify the 'Proof of Solution'
            # The genesis block is special and doesn't need to meet the difficulty target.
            if block.index > 0 and not block.hash.startswith(target_prefix):
                print(f"Validation Error: Proof of Solution failed for block {block.index}.")
                return False

            # 3. Verify that all parent blocks exist
            for p_hash in block.parent_hashes:
                if p_hash not in self.blocks:
                    print(f"Validation Error: Parent block {p_hash} for block {block.index} not found.")
                    return False

                # 4. Verify acyclicity by checking indices
                parent_block = self.blocks[p_hash]
                if parent_block.index >= block.index:
                    print(f"Validation Error: Cycle detected at block {block.index}.")
                    return False

        # Check for the existence of a single genesis block
        genesis_blocks = [b for b in self.blocks.values() if not b.parent_hashes]
        if len(genesis_blocks) != 1:
            print(f"Validation Error: Found {len(genesis_blocks)} genesis blocks. Expected 1.")
            return False

        return True
