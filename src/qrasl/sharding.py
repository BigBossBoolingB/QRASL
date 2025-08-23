from .blockchain import Blockchain
from .intent import Solver

class Shard:
    """
    Represents a single shard in the network. Each shard maintains its own
    independent blockchain (DAG) and processes its own intents.
    """
    def __init__(self, shard_id, difficulty=2):
        self.shard_id = shard_id
        # Each shard has its own instance of our DAG-based blockchain
        self.chain = Blockchain(difficulty=difficulty)
        # In a real system, solvers might be specialized per shard
        self.solver = Solver()
        print(f"Shard {self.shard_id}: Initialized.")

    def add_intent(self, intent):
        """Adds an intent to this specific shard's intent pool."""
        return self.chain.add_intent(intent)

    def create_block(self):
        """Triggers the creation of a new block on this shard's chain."""
        print(f"--- Shard {self.shard_id}: Attempting to create a new block ---")
        return self.chain.create_new_block(self.solver)

    def get_latest_state(self):
        """
        Returns a representation of the shard's latest state, which for a DAG
        is the set of its unconfirmed "tip" blocks.
        """
        return {tip.hash for tip in self.chain.get_tips()}


class BeaconChain:
    """
    The central coordinator of the sharded network. It manages shards and
    creates network-wide checkpoints.
    """
    def __init__(self):
        self.shards = {}  # {shard_id: Shard}
        self.checkpoints = []  # A log of network-wide states (checkpoints)
        print("BeaconChain: Initialized.")

    def register_shard(self, shard):
        """Adds a new shard to be tracked by the Beacon Chain."""
        if shard.shard_id in self.shards:
            raise ValueError(f"Shard with ID {shard.shard_id} is already registered.")
        print(f"BeaconChain: Registering Shard {shard.shard_id}...")
        self.shards[shard.shard_id] = shard

    def create_checkpoint(self):
        """
        Creates a checkpoint by recording the state of all registered shards.
        This is a simplified model of achieving network-wide consensus and finality.
        """
        print("\n--- BeaconChain: Creating network-wide checkpoint ---")
        checkpoint_data = {}
        for shard_id, shard in self.shards.items():
            shard_state = shard.get_latest_state()
            checkpoint_data[shard_id] = shard_state
            # Using a set comprehension for a more compact print output
            shard_state_short = {h[:8] for h in shard_state}
            print(f"  - Recording state for Shard {shard_id}: {shard_state_short}")

        self.checkpoints.append(checkpoint_data)
        print("--- Checkpoint created. ---")
        return checkpoint_data
