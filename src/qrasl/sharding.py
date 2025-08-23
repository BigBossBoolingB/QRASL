from .blockchain import Blockchain
from .intent import Solver

class Shard:
    """
    Represents a single shard in the network. Its logic is now updated to
    handle the full cross-shard communication life cycle.
    """
    def __init__(self, shard_id, difficulty=2):
        self.shard_id = shard_id
        self.chain = Blockchain(difficulty=difficulty)
        self.solver = Solver(shard_id=self.shard_id)
        self.incoming_messages = []
        print(f"Shard {self.shard_id}: Initialized.")

    def add_intent(self, intent):
        """Adds an intent to this shard's intent pool."""
        return self.chain.add_intent(intent)

    def deliver_message(self, message):
        """Adds an incoming cross-shard message to this shard's queue for processing."""
        print(f"Shard {self.shard_id}: Received incoming message for user {message.payload.get('user')}.")
        self.incoming_messages.append(message)

    def create_block(self):
        """
        Triggers the creation of a new block on this shard's chain.
        It processes both local intents and incoming messages.
        It returns any newly generated outgoing messages.
        """
        print(f"--- Shard {self.shard_id}: Attempting to create a new block ---")

        # Pass the current queue of incoming messages to the block creation logic
        messages_to_process = self.incoming_messages
        self.incoming_messages = [] # Clear the queue

        new_block, outgoing_messages = self.chain.create_new_block(
            solver=self.solver,
            incoming_messages=messages_to_process
        )

        # Return the new block and the outgoing messages
        return new_block, outgoing_messages

    def get_latest_state(self):
        """Returns the set of tip hashes for this shard's DAG."""
        return {tip.hash for tip in self.chain.get_tips()}


class BeaconChain:
    """
    The central coordinator, which acts as a message router for cross-shard communication.
    """
    def __init__(self):
        self.shards = {}
        self.checkpoints = []
        self.message_buffer = []
        print("BeaconChain: Initialized.")

    def register_shard(self, shard):
        """Adds a new shard to be tracked by the Beacon Chain."""
        if shard.shard_id in self.shards:
            raise ValueError(f"Shard with ID {shard.shard_id} is already registered.")
        print(f"BeaconChain: Registering Shard {shard.shard_id}...")
        self.shards[shard.shard_id] = shard

    def publish_messages(self, messages):
        """Called by the network operator to publish outgoing messages from shards."""
        if messages:
            self.message_buffer.extend(messages)
            print(f"BeaconChain: Published {len(messages)} new messages to the buffer.")

    def create_checkpoint(self):
        """
        Creates a checkpoint and routes all messages currently in the buffer.
        """
        print("\n--- BeaconChain: Starting checkpoint process ---")

        # 1. Route messages from the buffer
        print("--- BeaconChain: Routing messages ---")
        messages_to_route = self.message_buffer
        self.message_buffer = []

        for msg in messages_to_route:
            destination_shard = self.shards.get(msg.destination_shard_id)
            if destination_shard:
                print(f"  - Routing message from Shard {msg.source_shard_id} to Shard {msg.destination_shard_id}.")
                destination_shard.deliver_message(msg)
            else:
                print(f"  - WARNING: Destination Shard {msg.destination_shard_id} not found. Message dropped.")
        print("--- Message routing complete ---")

        # 2. Record the state of all shards
        print("--- BeaconChain: Recording shard states ---")
        checkpoint_data = {}
        for shard_id, shard in self.shards.items():
            shard_state = shard.get_latest_state()
            checkpoint_data[shard_id] = shard_state
            shard_state_short = {h[:8] for h in shard_state}
            print(f"  - Recording state for Shard {shard_id}: {shard_state_short}")

        self.checkpoints.append(checkpoint_data)
        print("--- Checkpoint created ---")
        return checkpoint_data
