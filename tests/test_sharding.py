import unittest
import sys
import os

# Add the src directory to the Python path
sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), '..')))

from src.qrasl.sharding import BeaconChain, Shard
from src.qrasl.intent import Intent, CrossShardMessage

class TestCrossShardCommunication(unittest.TestCase):
    def setUp(self):
        """Set up a fresh network for each test."""
        self.beacon_chain = BeaconChain()
        self.shard_0 = Shard(shard_id=0, difficulty=1)
        self.shard_1 = Shard(shard_id=1, difficulty=1)
        self.beacon_chain.register_shard(self.shard_0)
        self.beacon_chain.register_shard(self.shard_1)

    def test_message_generation_from_intent(self):
        """
        Tests that a 'cross_shard_transfer' intent correctly generates
        both a local solution (debit) and an outgoing message (credit).
        """
        intent = Intent(
            user="Alice",
            intent_data={'type': 'cross_shard_transfer', 'to_shard': 1, 'to_user': 'Zoe', 'amount': 50}
        )
        self.shard_0.add_intent(intent)

        # The shard's create_block method should return the outgoing messages
        _, outgoing_messages = self.shard_0.create_block()

        # Verify the outgoing message
        self.assertEqual(len(outgoing_messages), 1)
        msg = outgoing_messages[0]
        self.assertIsInstance(msg, CrossShardMessage)
        self.assertEqual(msg.destination_shard_id, 1)
        self.assertEqual(msg.payload['user'], 'Zoe')

        # Verify the local block on Shard 0 contains the corresponding debit
        local_block = self.shard_0.chain.get_tips()[0]
        self.assertEqual(len(local_block.solutions), 1)
        self.assertEqual(local_block.solutions[0].executed_tx['from'], 'Alice')
        self.assertEqual(local_block.solutions[0].executed_tx['to'], 'cross_shard_burn_address')

    def test_message_routing_via_beacon_chain(self):
        """Tests that the Beacon Chain correctly routes messages to the destination shard."""
        msg = CrossShardMessage(source_shard_id=0, destination_shard_id=1, payload={'data': 'test'})
        self.beacon_chain.publish_messages([msg])

        # Before checkpointing, the message should be in the buffer, not the shard's queue
        self.assertEqual(len(self.beacon_chain.message_buffer), 1)
        self.assertEqual(len(self.shard_1.incoming_messages), 0)

        # After checkpointing, the buffer should be empty and the message delivered
        self.beacon_chain.create_checkpoint()
        self.assertEqual(len(self.beacon_chain.message_buffer), 0)
        self.assertEqual(len(self.shard_1.incoming_messages), 1)
        self.assertEqual(self.shard_1.incoming_messages[0].payload['data'], 'test')

    def test_message_processing_in_new_block(self):
        """Tests that a destination shard correctly processes a delivered message into a new block."""
        msg = CrossShardMessage(source_shard_id=0, destination_shard_id=1, payload={'data': 'test'})
        self.shard_1.deliver_message(msg)

        # Create a block on the destination shard
        new_block, _ = self.shard_1.create_block()

        self.assertIsNotNone(new_block)
        # The new block should contain the processed message
        self.assertEqual(len(new_block.processed_messages), 1)
        self.assertEqual(new_block.processed_messages[0].payload['data'], 'test')
        # The shard's incoming queue should now be empty
        self.assertEqual(len(self.shard_1.incoming_messages), 0)

    def test_full_cross_shard_lifecycle(self):
        """
        Performs an end-to-end test of a cross-shard transaction, from intent
        submission on Shard 0 to its inclusion in a block on Shard 1.
        """
        # 1. User submits a cross-shard intent to Shard 0
        intent = Intent("Alice", {'type': 'cross_shard_transfer', 'to_shard': 1, 'to_user': 'Zoe', 'amount': 50})
        self.shard_0.add_intent(intent)

        # 2. Shard 0 creates a block, generating an outgoing message
        _, outgoing = self.shard_0.create_block()

        # 3. The message is published to the Beacon Chain
        self.beacon_chain.publish_messages(outgoing)

        # 4. The Beacon Chain checkpoints, routing the message to Shard 1
        self.beacon_chain.create_checkpoint()

        # 5. Shard 1 creates a block, processing the message from its queue
        final_block_on_shard_1, _ = self.shard_1.create_block()

        # Final Assertions
        self.assertEqual(len(self.shard_0.chain.blocks), 2, "Shard 0 should have genesis + 1 block.")
        self.assertEqual(len(self.shard_1.chain.blocks), 2, "Shard 1 should have genesis + 1 block.")

        # Verify the contents of the final block on Shard 1
        self.assertEqual(len(final_block_on_shard_1.processed_messages), 1)
        self.assertEqual(final_block_on_shard_1.processed_messages[0].payload['user'], 'Zoe')
        self.assertEqual(final_block_on_shard_1.processed_messages[0].payload['amount'], 50)

if __name__ == '__main__':
    unittest.main()
