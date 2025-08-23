from .blockchain import Blockchain
from .intent import Solver
from .governance import GovernanceSolver, Proposal

class Shard:
    """
    Represents a single shard, now with specialization. It can be a 'general'
    shard or a 'governance' shard with special logic.
    """
    def __init__(self, shard_id, difficulty=2, shard_type='general'):
        self.shard_id = shard_id
        self.type = shard_type
        self.chain = Blockchain(difficulty=difficulty)
        self.incoming_messages = []

        if self.type == 'governance':
            self.solver = GovernanceSolver()
            self.proposals = {}  # The current state of all proposals
        else:
            self.solver = Solver(shard_id=self.shard_id)

        print(f"Shard {self.shard_id} (type: {self.type}): Initialized.")

    def add_intent(self, intent):
        """Adds an intent to this shard's intent pool."""
        return self.chain.add_intent(intent)

    def deliver_message(self, message):
        """Adds an incoming cross-shard message to this shard's queue for processing."""
        print(f"Shard {self.shard_id}: Received incoming message for user {message.payload.get('user')}.")
        self.incoming_messages.append(message)

    def create_block(self):
        """
        Triggers the creation of a new block or a governance state update.
        Returns a tuple: (new_block, outgoing_messages, governance_actions)
        """
        print(f"--- Shard {self.shard_id}: Attempting to create a new block/state update ---")

        if self.type == 'governance':
            intents = self.chain.intent_pool
            # The Governance Solver processes intents and the current proposal state
            tips = self.chain.get_tips()
            new_block_index = max(t.index for t in tips) + 1 if tips else 0
            updated_proposals, execution_actions = self.solver.process_governance_intents(
                intents, self.proposals, new_block_index
            )
            self.proposals = updated_proposals
            self.chain.intent_pool = [] # Clear the processed intents

            # A governance shard doesn't create a "block" in the same way.
            # It just updates its state and outputs actions for the Beacon Chain.
            return None, [], execution_actions
        else:
            # Standard shard logic
            messages_to_process = self.incoming_messages
            self.incoming_messages = []
            new_block, outgoing_messages = self.chain.create_new_block(
                solver=self.solver,
                incoming_messages=messages_to_process
            )
            return new_block, outgoing_messages, [] # No governance actions

    def get_latest_state(self):
        """Returns the set of tip hashes for this shard's DAG."""
        return {tip.hash for tip in self.chain.get_tips()}


class BeaconChain:
    """The central coordinator, responsible for routing and executing governance actions."""
    def __init__(self):
        self.shards = {}
        self.checkpoints = []
        self.message_buffer = []
        print("BeaconChain: Initialized.")

    def register_shard(self, shard):
        """Adds a new shard to be tracked by the Beacon Chain."""
        if shard.shard_id in self.shards:
            raise ValueError(f"Shard with ID {shard.shard_id} is already registered.")
        print(f"BeaconChain: Registering Shard {shard.shard_id} (type: {shard.type})...")
        self.shards[shard.shard_id] = shard

    def process_shard_outputs(self, outgoing_messages, governance_actions):
        """
        Processes the outputs from a shard's block creation, publishing messages
        and executing governance actions.
        """
        if outgoing_messages:
            self.message_buffer.extend(outgoing_messages)
            print(f"BeaconChain: Published {len(outgoing_messages)} new messages to the buffer.")

        if governance_actions:
            print(f"BeaconChain: Received {len(governance_actions)} governance actions to execute.")
            for action in governance_actions:
                self.execute_governance_action(action)

    def execute_governance_action(self, action):
        """Executes a governance action, like changing a network parameter."""
        print(f"BeaconChain: Executing action for proposal '{action['proposal_id']}'...")
        if action['type'] == 'execute_proposal':
            target_param = action['target']
            new_value = action['value']

            if target_param == 'difficulty':
                print(f"  - ACTION: Changing network difficulty for all shards to {new_value}.")
                for shard in self.shards.values():
                    shard.chain.difficulty = new_value
            else:
                print(f"  - WARNING: Unknown governance target '{target_param}'. Action ignored.")

    def create_checkpoint(self):
        """Creates a checkpoint and routes all messages currently in the buffer."""
        print("\n--- BeaconChain: Starting checkpoint process ---")

        # Route messages
        messages_to_route = self.message_buffer
        self.message_buffer = []
        for msg in messages_to_route:
            destination_shard = self.shards.get(msg.destination_shard_id)
            if destination_shard:
                destination_shard.deliver_message(msg)

        # Record shard states
        checkpoint_data = {}
        for shard_id, shard in self.shards.items():
            checkpoint_data[shard_id] = shard.get_latest_state()
        self.checkpoints.append(checkpoint_data)
        print("--- Checkpoint created ---")
        return checkpoint_data
