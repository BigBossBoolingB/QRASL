import unittest
import sys
import os

# Add the src directory to the Python path to allow for direct imports
sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), '..')))

from src.qrasl.block import Block
from src.qrasl.blockchain import Blockchain

class TestBlockProofOfSolution(unittest.TestCase):
    def test_block_initialization(self):
        """Tests that a new block initializes correctly before being solved."""
        block = Block(index=1, transactions={"data": "test"}, previous_hash="abc")
        self.assertEqual(block.index, 1)
        self.assertIsNone(block.solution, "Solution should be None on init")
        self.assertIsNone(block.hash, "Hash should be None on init")

    def test_hash_depends_on_solution(self):
        """Tests that the block's hash changes when the solution changes."""
        block = Block(index=1, transactions={"data": "test"}, previous_hash="abc")
        block.solution = 100
        hash_1 = block.calculate_hash()

        block.solution = 200
        hash_2 = block.calculate_hash()

        self.assertNotEqual(hash_1, hash_2, "Hash should be different for different solutions")

class TestBlockchainProofOfSolution(unittest.TestCase):
    def setUp(self):
        """Set up a new blockchain with a low difficulty for each test."""
        # Using a low difficulty (e.g., 3) makes tests run fast.
        self.difficulty = 3
        self.blockchain = Blockchain(difficulty=self.difficulty)
        self.target_prefix = '0' * self.difficulty

    def test_blockchain_initialization(self):
        """Tests the initial state of the blockchain."""
        self.assertEqual(len(self.blockchain.chain), 1)
        self.assertEqual(self.blockchain.difficulty, self.difficulty)
        # Genesis block should be valid
        self.assertIsNotNone(self.blockchain.get_latest_block().hash)

    def test_solve_problem_for_block(self):
        """Tests that the solver finds a valid solution."""
        block = Block(2, "test transactions", "xyz")
        # The solve_problem_for_block method is now internal to add_block,
        # but we can test the outcome via add_block.
        added_block = self.blockchain.add_block("test transactions")
        self.assertTrue(added_block.hash.startswith(self.target_prefix))
        self.assertIsNotNone(added_block.solution)

    def test_add_block_and_chain_validity(self):
        """Tests adding a block and then validating the entire chain."""
        self.blockchain.add_block({"tx": "A->B: 5"})
        self.blockchain.add_block({"tx": "B->C: 10"})

        self.assertEqual(len(self.blockchain.chain), 3)
        self.assertTrue(self.blockchain.is_chain_valid(), "Chain should be valid after adding blocks")

    def test_chain_invalid_if_data_tampered(self):
        """Tests that tampering with data invalidates the chain."""
        self.blockchain.add_block({"tx": "original data"})

        # Tamper with the data of a block in the middle of the chain
        self.blockchain.chain[1].transactions = "tampered data"

        self.assertFalse(self.blockchain.is_chain_valid(), "Chain should be invalid after data tampering")

    def test_chain_invalid_if_solution_is_wrong(self):
        """Tests that a block with an incorrect hash (invalid solution) is detected."""
        self.blockchain.add_block({"tx": "some data"})

        # Manually set a hash that does not meet the difficulty requirement
        self.blockchain.chain[1].hash = "12345" + self.blockchain.chain[1].hash[5:]

        # The is_chain_valid method should detect that the hash doesn't match the data
        # and also that it doesn't meet the difficulty requirement.
        self.assertFalse(self.blockchain.is_chain_valid(), "Chain should be invalid if a hash is incorrect")

if __name__ == '__main__':
    unittest.main()
