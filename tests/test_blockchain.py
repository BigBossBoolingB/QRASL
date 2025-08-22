import unittest
import sys
import os

# Add the src directory to the Python path
sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), '..')))

from src.qrasl.block import Block
from src.qrasl.blockchain import Blockchain

class TestBlock(unittest.TestCase):
    def test_block_creation_and_hash(self):
        """Tests the creation of a block and the consistency of its hash."""
        block = Block(index=1, transactions={"data": "test"}, previous_hash="abc")
        self.assertEqual(block.index, 1)
        self.assertEqual(block.transactions, {"data": "test"})
        self.assertEqual(block.previous_hash, "abc")

        # Hash should be calculated correctly and consistently
        expected_hash = block.calculate_hash()
        self.assertEqual(block.hash, expected_hash)

        # A different block with same data should have a different hash due to timestamp
        # but if we control timestamp, it should be the same.
        # This is harder to test without mocking time, so we'll skip for this basic test.

        # Changing data should result in a different hash
        block.transactions = {"data": "changed"}
        self.assertNotEqual(block.calculate_hash(), expected_hash)

class TestBlockchain(unittest.TestCase):
    def setUp(self):
        """This method is called before each test."""
        self.blockchain = Blockchain()

    def test_genesis_block_creation(self):
        """Tests that the genesis block is created correctly."""
        self.assertEqual(len(self.blockchain.chain), 1)
        genesis_block = self.blockchain.get_latest_block()
        self.assertEqual(genesis_block.index, 0)
        self.assertEqual(genesis_block.transactions, "Genesis Block")
        self.assertEqual(genesis_block.previous_hash, "0")

    def test_add_new_block(self):
        """Tests adding a new block to the chain."""
        transactions = {"sender": "Alice", "receiver": "Bob", "amount": 10}
        self.blockchain.add_block(transactions)

        self.assertEqual(len(self.blockchain.chain), 2)
        new_block = self.blockchain.get_latest_block()
        genesis_block = self.blockchain.chain[0]

        self.assertEqual(new_block.index, 1)
        self.assertEqual(new_block.transactions, transactions)
        self.assertEqual(new_block.previous_hash, genesis_block.hash)

    def test_chain_validity(self):
        """Tests the integrity of a valid blockchain."""
        self.blockchain.add_block("tx1")
        self.blockchain.add_block("tx2")
        self.assertTrue(self.blockchain.is_chain_valid())

    def test_chain_invalid_due_to_data_tampering(self):
        """Tests that the chain becomes invalid if data in a block is tampered with."""
        self.blockchain.add_block({"data": "original"})

        # Tamper with the block's data directly
        self.blockchain.chain[1].transactions = {"data": "tampered"}

        self.assertFalse(self.blockchain.is_chain_valid())

    def test_chain_invalid_due_to_broken_hash_link(self):
        """Tests that the chain becomes invalid if the hash link is broken."""
        self.blockchain.add_block({"data": "some data"})

        # Manually break the chain's hash link
        self.blockchain.chain[1].previous_hash = "incorrect_hash_value"

        self.assertFalse(self.blockchain.is_chain_valid())

if __name__ == '__main__':
    unittest.main()
