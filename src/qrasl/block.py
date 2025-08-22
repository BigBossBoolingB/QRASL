import hashlib
import json
from time import time

class Block:
    """
    A single block in the blockchain.
    Its hash is determined after a 'solution' is found.
    """
    def __init__(self, index, transactions, previous_hash):
        self.index = index
        self.timestamp = time()
        self.transactions = transactions
        self.previous_hash = previous_hash
        self.solution = None  # The solution to the computational problem
        self.hash = None      # The hash is calculated after the solution is found

    def calculate_hash(self):
        """
        Calculates the SHA-256 hash of the block.
        The hash is dependent on all block data, including the solution.
        """
        # We must make sure that the Dictionary is Ordered, or we'll have inconsistent hashes
        block_dict = {
            'index': self.index,
            'timestamp': self.timestamp,
            'transactions': self.transactions,
            'previous_hash': self.previous_hash,
            'solution': self.solution,
        }
        block_string = json.dumps(block_dict, sort_keys=True).encode()
        return hashlib.sha256(block_string).hexdigest()
