import unittest
import sys
import os

# Add the src directory to the Python path
sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), '..')))

from src.qrasl.blockchain import Blockchain
from src.qrasl.intent import Intent, Solver, Solution

class TestIntentDrivenWorkflow(unittest.TestCase):
    def setUp(self):
        """Set up a new blockchain and a solver for each test."""
        self.difficulty = 1  # Low difficulty for fast tests
        self.blockchain = Blockchain(difficulty=self.difficulty)
        self.solver = Solver()

    def test_add_intent_to_pool(self):
        """Tests that intents are correctly added to the intent pool."""
        self.assertEqual(len(self.blockchain.intent_pool), 0)
        intent = Intent(user="Alice", intent_data={'type': 'transfer', 'to': 'Bob', 'amount': 10})
        self.blockchain.add_intent(intent)
        self.assertEqual(len(self.blockchain.intent_pool), 1)
        self.assertEqual(self.blockchain.intent_pool[0].hash, intent.hash)

    def test_create_block_processes_intents(self):
        """Tests that creating a block processes intents from the pool and creates solutions."""
        intent1 = Intent(user="Alice", intent_data={'type': 'transfer', 'to': 'Bob', 'amount': 10})
        intent2 = Intent(user="Charlie", intent_data={'type': 'transfer', 'to': 'David', 'amount': 5})
        self.blockchain.add_intent(intent1)
        self.blockchain.add_intent(intent2)

        new_block = self.blockchain.create_new_block(self.solver)

        # The new block should not be None
        self.assertIsNotNone(new_block)
        # It should contain two solutions, one for each intent
        self.assertEqual(len(new_block.solutions), 2)
        # The intent pool should now be empty
        self.assertEqual(len(self.blockchain.intent_pool), 0)

        # Check that the solutions in the block correspond to the processed intents
        processed_intent_hashes = {s.intent_hash for s in new_block.solutions}
        self.assertEqual({intent1.hash, intent2.hash}, processed_intent_hashes)

    def test_no_block_created_for_empty_pool(self):
        """Tests that a block is not created if the intent pool is empty."""
        new_block = self.blockchain.create_new_block(self.solver)
        self.assertIsNone(new_block, "No block should be created when there are no intents.")
        # The blockchain should only contain the genesis block
        self.assertEqual(len(self.blockchain.blocks), 1)

    def test_end_to_end_chain_validity(self):
        """Tests the validity of a chain after several blocks are created via intents."""
        # Block 1
        self.blockchain.add_intent(Intent(user="User1", intent_data={'type': 'transfer', 'to': 'User2', 'amount': 1}))
        self.blockchain.create_new_block(self.solver)

        # Block 2
        self.blockchain.add_intent(Intent(user="User2", intent_data={'type': 'transfer', 'to': 'User1', 'amount': 2}))
        self.blockchain.create_new_block(self.solver)

        self.assertTrue(self.blockchain.is_chain_valid(), "The chain should be valid after creating blocks from intents.")

    def test_validation_fails_if_solution_tampered(self):
        """Tests that the chain is invalid if a solution within a block is tampered with."""
        self.blockchain.add_intent(Intent(user="Alice", intent_data={'type': 'transfer', 'to': 'Bob', 'amount': 10}))
        new_block = self.blockchain.create_new_block(self.solver)

        # The chain should be valid initially
        self.assertTrue(self.blockchain.is_chain_valid())

        # Tamper with the executed transaction amount inside the solution
        new_block.solutions[0].executed_tx['amount'] = 9999

        # The chain should now be invalid because the block's hash will not match its content
        self.assertFalse(self.blockchain.is_chain_valid(), "The chain should be invalid after tampering with a solution.")

if __name__ == '__main__':
    unittest.main()
