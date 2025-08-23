import unittest
import sys
import os

# Add the src directory to the Python path
sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), '..')))

from src.qrasl.sharding import BeaconChain, Shard
from src.qrasl.intent import Intent

class TestGovernanceWorkflow(unittest.TestCase):
    def setUp(self):
        """Set up a network with a governance shard for each test."""
        self.beacon_chain = BeaconChain()
        self.shard_0 = Shard(shard_id=0, difficulty=1)
        # Use a low signal threshold for easy testing
        self.shard_6 = Shard(shard_id=6, shard_type='governance')
        self.shard_6.solver.signal_threshold = 50  # Override default for tests

        self.beacon_chain.register_shard(self.shard_0)
        self.beacon_chain.register_shard(self.shard_6)

    def test_proposal_creation(self):
        """Tests that a 'propose' intent correctly creates a proposal in the governance state."""
        self.assertEqual(len(self.shard_6.proposals), 0, "Proposal state should be initially empty.")

        intent = Intent("GovUser", {'type': 'propose', 'id': 'p1', 'target': 'difficulty', 'value': 3})
        self.shard_6.add_intent(intent)

        # The create_block method on the gov shard processes intents and updates its internal state
        self.shard_6.create_block()

        self.assertEqual(len(self.shard_6.proposals), 1)
        self.assertIn('p1', self.shard_6.proposals)
        self.assertEqual(self.shard_6.proposals['p1'].status, 'signaling')
        self.assertEqual(self.shard_6.proposals['p1'].proposed_value, 3)

    def test_signal_tallying(self):
        """Tests that 'signal' intents correctly update a proposal's signal weight."""
        # First, create the proposal
        self.shard_6.add_intent(Intent("GovUser", {'type': 'propose', 'id': 'p1', 'target': 'difficulty', 'value': 3}))
        self.shard_6.create_block()

        # Now, add signals
        self.shard_6.add_intent(Intent("User1", {'type': 'signal', 'proposal_id': 'p1', 'weight': 20}))
        self.shard_6.add_intent(Intent("User2", {'type': 'signal', 'proposal_id': 'p1', 'weight': 25}))
        self.shard_6.create_block()

        # The total weight should be 45, which is below the threshold of 50
        self.assertEqual(self.shard_6.proposals['p1'].signal_weight, 45)
        self.assertEqual(self.shard_6.proposals['p1'].status, 'signaling', "Proposal should not have passed yet.")

    def test_proposal_execution_flow(self):
        """
        Tests the full flow: proposal passing, action generation, and execution by the Beacon Chain.
        """
        # Create a proposal
        self.shard_6.add_intent(Intent("GovUser", {'type': 'propose', 'id': 'p1', 'target': 'difficulty', 'value': 3}))
        self.shard_6.create_block()

        # Add signals that cross the threshold (30 + 30 = 60 > 50)
        self.shard_6.add_intent(Intent("User1", {'type': 'signal', 'proposal_id': 'p1', 'weight': 30}))
        self.shard_6.add_intent(Intent("User2", {'type': 'signal', 'proposal_id': 'p1', 'weight': 30}))

        # This run of the gov shard should generate an execution action
        _, _, gov_actions = self.shard_6.create_block()

        # Verify the action was generated and the proposal status is 'passed'
        self.assertEqual(len(gov_actions), 1, "An execution action should have been generated.")
        self.assertEqual(self.shard_6.proposals['p1'].status, 'passed')

        # The Beacon Chain now processes the action
        self.assertEqual(self.shard_0.chain.difficulty, 1, "Shard 0 difficulty should be 1 initially.")
        self.beacon_chain.process_shard_outputs([], gov_actions)

        # The difficulty on the other shard should now be updated
        self.assertEqual(self.shard_0.chain.difficulty, 3, "Shard 0 difficulty should have been updated by governance.")

if __name__ == '__main__':
    unittest.main()
