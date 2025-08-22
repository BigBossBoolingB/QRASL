import hashlib
import json
from time import time

class Block:
    """
    A single block in a DAG-based blockchain.
    It can have multiple parent blocks, making it part of a graph.
    """
    def __init__(self, index, transactions, parent_hashes):
        self.index = index
        self.timestamp = time()
        self.transactions = transactions
        # Parent hashes are sorted to ensure deterministic hash calculation
        self.parent_hashes = sorted(list(parent_hashes))
        self.solution = None  # The solution to the computational problem
        self.hash = None      # The hash is calculated after the solution is found

    def calculate_hash(self):
        """
        Calculates the SHA-256 hash of the block.
        The hash is dependent on all block data, including the list of parent hashes.
        """
        block_dict = {
            'index': self.index,
            'timestamp': self.timestamp,
            'transactions': self.transactions,
            'parent_hashes': self.parent_hashes,
            'solution': self.solution,
        }
        # Using sort_keys=True ensures that the JSON string is always the same for the same data
        block_string = json.dumps(block_dict, sort_keys=True).encode()
        return hashlib.sha256(block_string).hexdigest()
