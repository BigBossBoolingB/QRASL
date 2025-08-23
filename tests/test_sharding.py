import unittest
import sys
import os

# Add the src directory to the Python path
sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), '..')))

from src.qrasl.sharding import BeaconChain, Shard
from src.qrasl.intent import Intent

class TestShardingSystem(unittest.TestCase):
    def setUp(self):
        """Set up a new BeaconChain and two Shards for each test."""
        self.beacon_chain = BeaconChain()
        self.shard_0 = Shard(shard_id=0, difficulty=1)
        self.shard_1 = Shard(shard_id=1, difficulty=1)

    def test_shard_initialization(self):
        """Tests that a Shard initializes with its own genesis block."""
        self.assertEqual(self.shard_0.shard_id, 0)
        self.assertIsNotNone(self.shard_0.chain, "Shard should have a chain instance.")
        self.assertEqual(len(self.shard_0.chain.blocks), 1, "Shard's chain should have a genesis block.")

    def test_beacon_chain_shard_registration(self):
        """Tests that shards can be registered and that duplicate IDs are rejected."""
        self.beacon_chain.register_shard(self.shard_0)
        self.assertIn(0, self.beacon_chain.shards)
        self.assertEqual(self.beacon_chain.shards[0], self.shard_0)

        # Test that registering a shard with a duplicate ID raises a ValueError
        with self.assertRaises(ValueError):
            self.beacon_chain.register_shard(self.shard_0)

    def test_independent_shard_activity(self):
        """Tests that activity on one shard does not affect another."""
        # Add an intent and create a block on Shard 0
        intent_a = Intent(user="Alice", intent_data={'type': 'transfer', 'to': 'Bob', 'amount': 10})
        self.shard_0.add_intent(intent_a)

        self.assertEqual(len(self.shard_0.chain.intent_pool), 1)
        self.assertEqual(len(self.shard_1.chain.intent_pool), 0, "Shard 1's pool should be unaffected.")

        block_a = self.shard_0.create_block()

        self.assertEqual(len(self.shard_0.chain.blocks), 2)
        self.assertEqual(len(self.shard_1.chain.blocks), 1, "Shard 1's chain should be unaffected.")
        self.assertIsNotNone(block_a)

    def test_checkpoint_mechanism(self):
        """Tests the Beacon Chain's ability to create accurate checkpoints."""
        self.beacon_chain.register_shard(self.shard_0)
        self.beacon_chain.register_shard(self.shard_1)

        # Create the first checkpoint when shards are in their genesis state
        checkpoint1 = self.beacon_chain.create_checkpoint()
        self.assertEqual(len(self.beacon_chain.checkpoints), 1)

        # Get the genesis hashes to verify the checkpoint
        genesis_hash_0 = list(self.shard_0.chain.blocks.keys())[0]
        genesis_hash_1 = list(self.shard_1.chain.blocks.keys())[0]

        self.assertEqual(checkpoint1[0], {genesis_hash_0})
        self.assertEqual(checkpoint1[1], {genesis_hash_1})

        # Add a block to Shard 0 and create another checkpoint
        self.shard_0.add_intent(Intent(user="UserA", intent_data={'type': 'transfer', 'to': 'UserB', 'amount': 99}))
        new_block_shard_0 = self.shard_0.create_block()

        checkpoint2 = self.beacon_chain.create_checkpoint()
        self.assertEqual(len(self.beacon_chain.checkpoints), 2)

        # The state for Shard 0 should have changed to the new block's hash
        self.assertNotEqual(checkpoint1[0], checkpoint2[0])
        self.assertEqual(checkpoint2[0], {new_block_shard_0.hash})

        # The state for Shard 1 should have remained the same
        self.assertEqual(checkpoint1[1], checkpoint2[1])

if __name__ == '__main__':
    unittest.main()
