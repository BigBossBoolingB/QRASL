import hashlib
import json

class Intent:
    """Represents a user's declared intention."""
    def __init__(self, user, intent_data):
        self.user = user
        self.data = intent_data
        self.hash = self._calculate_hash()

    def _calculate_hash(self):
        intent_string = json.dumps(self.__dict__, sort_keys=True).encode()
        return hashlib.sha256(intent_string).hexdigest()

    def to_dict(self):
        return {"user": self.user, "data": self.data, "hash": self.hash}

class Solution:
    """Represents the result of a solver processing an intent for local execution."""
    def __init__(self, intent_hash, executed_tx):
        self.intent_hash = intent_hash
        self.executed_tx = executed_tx

    def to_dict(self):
        return {"intent_hash": self.intent_hash, "executed_tx": self.executed_tx}

class CrossShardMessage:
    """Represents a message to be sent from one shard to another via the Beacon Chain."""
    def __init__(self, source_shard_id, destination_shard_id, payload):
        self.source_shard_id = source_shard_id
        self.destination_shard_id = destination_shard_id
        self.payload = payload  # e.g., {'action': 'credit', 'user': 'Zoe', 'amount': 50}

    def to_dict(self):
        """Returns a dictionary representation of the message."""
        return {
            "source_shard_id": self.source_shard_id,
            "destination_shard_id": self.destination_shard_id,
            "payload": self.payload,
        }

class Solver:
    """
    A conceptual Solver that processes intents, producing local solutions and
    outgoing cross-shard messages.
    """
    def __init__(self, shard_id):
        self.shard_id = shard_id  # The solver needs to know which shard it's on

    def solve_intents(self, intents):
        """
        Processes intents and returns a tuple of:
        (list of local Solutions, list of outgoing CrossShardMessages)
        """
        local_solutions = []
        outgoing_messages = []
        for intent in intents:
            intent_data = intent.data

            if intent_data.get('type') == 'transfer':
                # This is a simple transfer within the same shard.
                tx = {'from': intent.user, 'to': intent_data.get('to'), 'amount': intent_data.get('amount')}
                local_solutions.append(Solution(intent_hash=intent.hash, executed_tx=tx))

            elif intent_data.get('type') == 'cross_shard_transfer':
                # This involves creating a local transaction (debit) and an outgoing message (credit).
                # 1. Create the local transaction to debit the user's account on this shard.
                debit_tx = {'from': intent.user, 'to': 'cross_shard_burn_address', 'amount': intent_data.get('amount')}
                local_solutions.append(Solution(intent_hash=intent.hash, executed_tx=debit_tx))

                # 2. Create the message to be sent to the other shard.
                credit_payload = {
                    'action': 'credit_from_message',
                    'user': intent_data.get('to_user'),
                    'amount': intent_data.get('amount')
                }
                msg = CrossShardMessage(
                    source_shard_id=self.shard_id,
                    destination_shard_id=intent_data.get('to_shard'),
                    payload=credit_payload
                )
                outgoing_messages.append(msg)

        return local_solutions, outgoing_messages
