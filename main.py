from src.qrasl.sharding import BeaconChain, Shard
from src.qrasl.intent import Intent

def main():
    """
    A demonstration of the sharded architecture of the QRASL network.
    """
    print("--- Sharded QRASL Network Demonstration ---")

    # 1. Initialize the core components: the Beacon Chain and multiple Shards.
    # As per the README, shards can have different specializations (e.g., difficulty).
    print("\n--- Step 1: Initializing the Network Infrastructure ---")
    beacon_chain = BeaconChain()
    shard_0 = Shard(shard_id=0, difficulty=1)  # A general-purpose shard
    shard_1 = Shard(shard_id=1, difficulty=2)  # A more secure/slower DeFi shard

    # Shards must be registered with the Beacon Chain to be tracked.
    beacon_chain.register_shard(shard_0)
    beacon_chain.register_shard(shard_1)

    # 2. Users submit intents to specific shards based on their needs.
    print("\n--- Step 2: Users submit intents to different shards ---")
    # A simple transfer on the general-purpose shard
    shard_0.add_intent(Intent(user="Alice", intent_data={'type': 'transfer', 'amount': 10, 'to': 'Bob'}))

    # Higher value transactions on the DeFi shard
    shard_1.add_intent(Intent(user="Charlie", intent_data={'type': 'transfer', 'amount': 100, 'to': 'DeFi-Pool-A'}))
    shard_1.add_intent(Intent(user="David", intent_data={'type': 'transfer', 'amount': 200, 'to': 'DeFi-Pool-B'}))

    # 3. Block producers for each shard run independently to process intents.
    print("\n--- Step 3: Shards create blocks in parallel ---")
    shard_0.create_block()
    shard_1.create_block()

    # 4. The Beacon Chain creates a network-wide checkpoint to "notarize" the state of all shards.
    beacon_chain.create_checkpoint()

    # 5. More activity occurs in the next "epoch".
    print("\n--- Step 4: More activity occurs on the network ---")
    shard_0.add_intent(Intent(user="Bob", intent_data={'type': 'transfer', 'amount': 5, 'to': 'Alice'}))
    # Shard 1 has no new intents this round.

    # Shard 0 creates a new block. Shard 1 does not, as it has no intents.
    shard_0.create_block()
    shard_1.create_block() # This should indicate no intents to process

    # 6. The Beacon Chain creates a final checkpoint.
    final_checkpoint = beacon_chain.create_checkpoint()

    print("\n--- Final Network State Summary ---")
    print(f"Total checkpoints created by BeaconChain: {len(beacon_chain.checkpoints)}")
    print("Final recorded state in last checkpoint:")
    for shard_id, state in final_checkpoint.items():
        # Convert set to list for stable output, then shorten hashes
        state_hashes_short = [h[:10] for h in sorted(list(state))]
        print(f"  - Shard {shard_id} Final Tip Hashes: {state_hashes_short}")

if __name__ == "__main__":
    main()
