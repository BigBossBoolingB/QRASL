import hashlib
import json
from time import time
from .intent import Solution, CrossShardMessage

class Block:
    """
    A block's payload now consists of solutions from local intents and
    a list of processed incoming cross-shard messages.
    """
    def __init__(self, index, solutions, parent_hashes, processed_messages):
        self.index = index
        self.timestamp = time()
        self.solutions = solutions
        self.parent_hashes = sorted(list(parent_hashes))
        self.processed_messages = processed_messages # New field
        self.solution = None  # The solution to the PoS problem (mining)
        self.hash = None      # The hash is calculated after the solution is found

    def calculate_hash(self):
        """
        Calculates the SHA-256 hash of the block. The hash now also depends
        on the cross-shard messages that were processed in this block.
        """
        solutions_as_dicts = [s.to_dict() for s in self.solutions]
        messages_as_dicts = [m.to_dict() for m in self.processed_messages]

        block_dict = {
            'index': self.index,
            'timestamp': self.timestamp,
            'solutions': solutions_as_dicts,
            'parent_hashes': self.parent_hashes,
            'processed_messages': messages_as_dicts, # New field
            'solution': self.solution,
        }
        block_string = json.dumps(block_dict, sort_keys=True).encode()
        return hashlib.sha256(block_string).hexdigest()
