from src.qrasl.sharding import BeaconChain, Shard
from src.qrasl.intent import Intent

def main():
    """
    A demonstration of the 'Signal-Execute' governance model.
    """
    print("--- 'Signal-Execute' Governance Demonstration ---")

    # 1. Setup the network with a general shard and a designated Governance Shard (ID 6).
    print("\n--- Step 1: Initializing Network ---")
    beacon_chain = BeaconChain()
    shard_0 = Shard(shard_id=0, difficulty=1)
    # The Governance Shard has its own solver and state management for proposals.
    shard_6 = Shard(shard_id=6, shard_type='governance')
    beacon_chain.register_shard(shard_0)
    beacon_chain.register_shard(shard_6)
    print(f"Initial difficulty for Shard 0: {shard_0.chain.difficulty}")

    # 2. A user submits an intent to Shard 6 to propose a change.
    print("\n--- Step 2: A proposal to change network difficulty is submitted to Shard 6 ---")
    proposal_intent = Intent(
        user="GovUserA",
        intent_data={'type': 'propose', 'id': 'prop-001-difficulty', 'target': 'difficulty', 'value': 2}
    )
    shard_6.add_intent(proposal_intent)

    # The Gov shard processes this, creating the proposal in its state.
    _, _, gov_actions = shard_6.create_block()
    # The Beacon Chain processes any outputs (there should be no execution actions yet).
    beacon_chain.process_shard_outputs([], gov_actions)

    # 3. Other users signal their support for the proposal by sending intents to Shard 6.
    print("\n--- Step 3: Users signal support for 'prop-001-difficulty' ---")
    shard_6.add_intent(Intent("User1", {'type': 'signal', 'proposal_id': 'prop-001-difficulty', 'weight': 50}))
    shard_6.add_intent(Intent("User2", {'type': 'signal', 'proposal_id': 'prop-001-difficulty', 'weight': 60}))

    # 4. The Governance shard runs again. The combined signal weight (110) should
    #    exceed the default threshold (100), causing the proposal to pass.
    print("\n--- Step 4: Governance shard processes signals ---")
    # The `create_block` call on the governance shard will return an execution action.
    _, _, gov_actions = shard_6.create_block()

    # 5. The Beacon Chain receives the execution action and applies the change to all shards.
    print("\n--- Step 5: Beacon Chain executes the passed proposal's action ---")
    beacon_chain.process_shard_outputs([], gov_actions)

    # 6. Verify that the difficulty parameter on Shard 0 has been updated.
    print("\n--- Step 6: Verify parameter change on a general-purpose shard ---")
    print(f"Verifying difficulty on Shard 0. Expected: 2, Actual: {shard_0.chain.difficulty}")
    if shard_0.chain.difficulty == 2:
        print("Verification successful! The network parameter was updated via governance.")
    else:
        print("Verification FAILED.")

    # We can create a block on Shard 0 to see it use the new difficulty.
    print("\n--- Creating a block on Shard 0 with the new difficulty ---")
    shard_0.add_intent(Intent("Alice", {'type': 'transfer', 'to': 'Bob', 'amount': 10}))
    shard_0.create_block()

if __name__ == "__main__":
    main()
