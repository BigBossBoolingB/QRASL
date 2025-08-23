import hashlib
import json

class Intent:
    """
    Represents a user's declared intention, before it is processed by a solver.
    """
    def __init__(self, user, intent_data):
        self.user = user
        self.data = intent_data  # e.g., {'type': 'transfer', 'amount': 10, 'to': 'Bob'}
        self.hash = self._calculate_hash()

    def _calculate_hash(self):
        """Calculates a unique hash for the intent."""
        # The dictionary must be sorted to ensure the hash is deterministic.
        intent_string = json.dumps(self.__dict__, sort_keys=True).encode()
        return hashlib.sha256(intent_string).hexdigest()

    def to_dict(self):
        """Returns a dictionary representation of the Intent."""
        return {
            "user": self.user,
            "data": self.data,
            "hash": self.hash
        }

class Solution:
    """
    Represents the result of a solver processing an intent.
    This is what gets included in a block.
    """
    def __init__(self, intent_hash, executed_tx):
        self.intent_hash = intent_hash
        self.executed_tx = executed_tx  # The actual transaction created by the solver

    def to_dict(self):
        """Returns a dictionary representation of the Solution."""
        return {
            "intent_hash": self.intent_hash,
            "executed_tx": self.executed_tx
        }

class Solver:
    """
    A conceptual Solver entity that processes intents and produces solutions.
    """
    def __init__(self):
        pass  # In a real system, a solver might have its own state or configuration.

    def solve_intents(self, intents):
        """
        A simple solver that converts a list of intents directly into solutions.

        In a real, complex system, this would involve sophisticated logic,
        such as batching transfers, finding optimal paths for swaps, etc.
        """
        solutions = []
        for intent in intents:
            # This is a basic 1-to-1 conversion for demonstration purposes.
            if intent.data.get('type') == 'transfer':
                tx = {
                    'from': intent.user,
                    'to': intent.data.get('to'),
                    'amount': intent.data.get('amount')
                }
                solutions.append(Solution(intent_hash=intent.hash, executed_tx=tx))
        return solutions
