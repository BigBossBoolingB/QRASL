from .block import Block

class Blockchain:
    """
    Manages the chain of blocks.
    """
    def __init__(self):
        """
        Initializes the blockchain with a genesis block.
        """
        self.chain = [self.create_genesis_block()]
        # A simple placeholder for difficulty, for potential future use with Proof of Work
        # self.difficulty = 2

    def create_genesis_block(self):
        """
        Creates the very first block in the chain (Block 0).
        """
        return Block(0, "Genesis Block", "0")

    def get_latest_block(self):
        """
        Returns the most recent block in the chain.
        """
        return self.chain[-1]

    def add_block(self, transactions):
        """
        Adds a new block to the chain after validating it (validation is basic for now).
        """
        latest_block = self.get_latest_block()
        new_block = Block(
            index=latest_block.index + 1,
            transactions=transactions,
            previous_hash=latest_block.hash
        )
        self.chain.append(new_block)
        return new_block

    def is_chain_valid(self):
        """
        Determines if the blockchain is valid.
        1. Checks if the stored hash of each block is correct.
        2. Checks if the previous_hash of each block links to the hash of the previous block.
        """
        for i in range(1, len(self.chain)):
            current_block = self.chain[i]
            previous_block = self.chain[i-1]

            if current_block.hash != current_block.calculate_hash():
                print(f"Stored hash for block {current_block.index} is incorrect.")
                return False

            if current_block.previous_hash != previous_block.hash:
                print(f"Chain is broken at block {current_block.index}.")
                return False

        # Also check genesis block
        if self.chain[0].hash != self.chain[0].calculate_hash():
            print("Genesis block hash is incorrect.")
            return False

        return True
