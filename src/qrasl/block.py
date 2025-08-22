import hashlib
import json
from time import time

class Block:
    """
    A single block in the blockchain.
    """
    def __init__(self, index, transactions, previous_hash):
        self.index = index
        self.timestamp = time()
        self.transactions = transactions # Could be a list of transactions
        self.previous_hash = previous_hash
        # self.nonce = 0 # For Proof of Work, can be added later
        self.hash = self.calculate_hash()

    def calculate_hash(self):
        """
        Calculates the SHA-256 hash of the block.
        We are hashing a JSON representation of the block's dictionary.
        """
        # We must make sure that the Dictionary is Ordered, or we'll have inconsistent hashes
        block_dict = {
            'index': self.index,
            'timestamp': self.timestamp,
            'transactions': self.transactions,
            'previous_hash': self.previous_hash,
        }
        block_string = json.dumps(block_dict, sort_keys=True).encode()
        return hashlib.sha256(block_string).hexdigest()
