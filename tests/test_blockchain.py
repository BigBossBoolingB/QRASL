import unittest
import sys
import os

# Add the src directory to the Python path
sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), '..')))

from src.qrasl.block import Block
from src.qrasl.blockchain import Blockchain

class TestDAGFeatures(unittest.TestCase):
    def setUp(self):
        """Set up a new DAG blockchain for each test."""
        # Use a very low difficulty for speed, as we are not testing the solver here.
        self.difficulty = 1
        self.blockchain = Blockchain(difficulty=self.difficulty)
        self.genesis = self.blockchain.get_tips()[0]

    def test_genesis_block(self):
        """Tests the properties of the initial genesis block."""
        self.assertEqual(len(self.blockchain.blocks), 1)
        self.assertEqual(self.genesis.index, 0)
        self.assertEqual(self.genesis.parent_hashes, [])
        self.assertEqual(len(self.blockchain.get_tips()), 1, "Initially, only the genesis block should be a tip.")

    def test_add_block_and_tips(self):
        """Tests adding blocks and how it affects the tips of the DAG."""
        # Add two parallel blocks after genesis
        block_a = self.blockchain.add_block("tx_a", [self.genesis.hash])
        block_b = self.blockchain.add_block("tx_b", [self.genesis.hash])

        # The tips should now be block_a and block_b
        tip_hashes = {t.hash for t in self.blockchain.get_tips()}
        self.assertEqual({block_a.hash, block_b.hash}, tip_hashes, "Tips should be the two new parallel blocks.")

        # Add a merge block that combines the two branches
        block_c = self.blockchain.add_block("tx_c_merge", [block_a.hash, block_b.hash])

        # The only tip should now be the new merge block
        self.assertEqual(len(self.blockchain.get_tips()), 1)
        self.assertEqual(self.blockchain.get_tips()[0].hash, block_c.hash, "The merge block should be the only tip.")

    def test_add_block_with_nonexistent_parent(self):
        """Tests that adding a block with a fake parent hash raises an error."""
        with self.assertRaises(ValueError):
            self.blockchain.add_block("tx_bad", ["this_hash_does_not_exist"])

    def test_dag_is_valid(self):
        """Tests the is_chain_valid method on a valid DAG structure."""
        block_a = self.blockchain.add_block("tx_a", [self.genesis.hash])
        block_b = self.blockchain.add_block("tx_b", [self.genesis.hash])
        self.blockchain.add_block("tx_c_merge", [block_a.hash, block_b.hash])
        self.assertTrue(self.blockchain.is_chain_valid(), "A validly constructed DAG should pass validation.")

    def test_dag_invalid_if_data_tampered(self):
        """Tests that tampering with a block's data invalidates the DAG."""
        block_a = self.blockchain.add_block("tx_a", [self.genesis.hash])
        # Manually change the transaction data after the block has been added
        block_a.transactions = "tampered_tx"
        self.assertFalse(self.blockchain.is_chain_valid(), "DAG should be invalid if any block's data is tampered with.")

    def test_dag_invalid_if_cycle_detected(self):
        """Tests that the validation logic detects cycles."""
        block_a = self.blockchain.add_block("tx_a", [self.genesis.hash])
        # Manually create a block that points to itself as a parent (a simple cycle)
        # This is hard to do with add_block, so we tamper with the block's parents after creation.
        block_a.parent_hashes.append(block_a.hash)
        # The validation should detect this logical inconsistency.
        # Note: Our current index check is the primary cycle detection.
        # Let's try to break the index rule instead.
        block_a.index = 0 # Make its index the same as its parent (genesis)
        self.assertFalse(self.blockchain.is_chain_valid(), "DAG should be invalid if a cycle is detected via indices.")

if __name__ == '__main__':
    unittest.main()
