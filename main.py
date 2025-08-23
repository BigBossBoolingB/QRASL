from src.qrasl.sharding import BeaconChain, Shard
from src.qrasl.intent import Intent
import json

def print_shard_state(shard):
    """A helper function to print the detailed state of a single shard."""
    print(f"\n--- Shard {shard.shard_id} State ---")
    if not shard.chain.blocks:
        print("  No blocks yet.")
        return

    sorted_blocks = sorted(shard.chain.blocks.values(), key=lambda b: b.index)
    for block in sorted_blocks:
        print(f"  Block {block.index} (Hash: {block.hash[:10]}...):")
        if block.solutions:
            print("    Solutions (from local intents):")
            for sol in block.solutions:
                print(f"    - Intent: {sol.intent_hash[:10]}... -> TX: {json.dumps(sol.executed_tx)}")
        if block.processed_messages:
            print("    Processed Messages (from other shards):")
            for msg in block.processed_messages:
                print(f"    - From Shard {msg.source_shard_id}: {json.dumps(msg.payload)}")
        if not block.solutions and not block.processed_messages:
            print("    - (Genesis Block)")

def main():
    """
    A demonstration of the cross-shard communication via the Beacon Chain.
    """
    print("--- Cross-Shard Communication Demonstration ---")

    # 1. Setup the network with a Beacon Chain and two shards.
    print("\n--- Step 1: Initializing Network ---")
    beacon_chain = BeaconChain()
    shard_0 = Shard(shard_id=0, difficulty=1)
    shard_1 = Shard(shard_id=1, difficulty=1)
    beacon_chain.register_shard(shard_0)
    beacon_chain.register_shard(shard_1)

    # 2. A user on Shard 0 initiates a cross-shard transfer to a user on Shard 1.
    print("\n--- Step 2: User on Shard 0 initiates a cross-shard transfer ---")
    cross_shard_intent = Intent(
        user="Alice",
        intent_data={
            'type': 'cross_shard_transfer',
            'amount': 50,
            'to_shard': 1,
            'to_user': 'Zoe'
        }
    )
    shard_0.add_intent(cross_shard_intent)

    # 3. Shard 0's block producer runs. It processes the intent, creating a local
    #    "debit" transaction and an outgoing message for Shard 1.
    print("\n--- Step 3: Shard 0 creates a block ---")
    outgoing_messages_from_0 = shard_0.create_block()

    # 4. The generated messages are published to the Beacon Chain's buffer.
    print("\n--- Step 4: Messages from Shard 0 published to Beacon Chain ---")
    beacon_chain.publish_messages(outgoing_messages_from_0)

    # 5. The Beacon Chain performs its checkpointing, which includes routing
    #    all messages from its buffer to the destination shards' incoming queues.
    print("\n--- Step 5: Beacon Chain checkpoints and routes messages ---")
    beacon_chain.create_checkpoint()

    # Verify Shard 1 received the message
    print(f"Shard 1's incoming message queue size: {len(shard_1.incoming_messages)}")

    # 6. Shard 1's block producer runs. It has no new intents, but it finds
    #    the message in its queue and processes it, creating a "credit" transaction.
    print("\n--- Step 6: Shard 1 creates a block, processing the delivered message ---")
    outgoing_messages_from_1 = shard_1.create_block()
    # This should be empty, as no new cross-shard intents were processed
    beacon_chain.publish_messages(outgoing_messages_from_1)

    # 7. A final checkpoint is created to finalize the state.
    beacon_chain.create_checkpoint()

    # 8. Print the final, detailed state of both shards to see the result.
    print("\n--- Final Detailed State of All Shards ---")
    print_shard_state(shard_0)
    print_shard_state(shard_1)

if __name__ == "__main__":
    main()
