import hashlib
import json

class Proposal:
    """Represents a governance proposal."""
    def __init__(self, proposal_id, target_parameter, proposed_value, created_at_block):
        self.id = proposal_id
        self.target_parameter = target_parameter  # e.g., 'difficulty'
        self.proposed_value = proposed_value
        self.created_at_block = created_at_block
        self.status = 'signaling'  # signaling, passed, failed, executed
        self.signal_weight = 0  # A simple sum of weights for this model

    def to_dict(self):
        """Returns a dictionary representation of the Proposal."""
        return self.__dict__

class GovernanceSolver:
    """
    A specialized solver for processing governance-related intents.
    It encapsulates the logic of the 'Signal-Execute' model.
    """
    def __init__(self, signal_threshold=100):
        # A proposal passes if its signal weight exceeds this threshold.
        self.signal_threshold = signal_threshold

    def process_governance_intents(self, intents, proposals_state, current_block_index):
        """
        Processes 'propose' and 'signal' intents against the current proposal state.

        Returns:
            - A dictionary of updated proposals.
            - A list of actions to be executed by the Beacon Chain.
        """
        execution_actions = []

        # Use a copy to avoid mutating the original state during iteration
        updated_proposals = proposals_state.copy()

        for intent in intents:
            data = intent.data
            intent_type = data.get('type')

            if intent_type == 'propose':
                proposal_id = data.get('id')
                if proposal_id not in updated_proposals:
                    new_proposal = Proposal(
                        proposal_id=proposal_id,
                        target_parameter=data.get('target'),
                        proposed_value=data.get('value'),
                        created_at_block=current_block_index
                    )
                    updated_proposals[proposal_id] = new_proposal
                    print(f"GovSolver: New proposal '{new_proposal.id}' created.")

            elif intent_type == 'signal':
                proposal_id = data.get('proposal_id')
                weight = data.get('weight', 1)  # Default signal weight is 1

                if proposal_id in updated_proposals and updated_proposals[proposal_id].status == 'signaling':
                    updated_proposals[proposal_id].signal_weight += weight
                    print(f"GovSolver: Signal of weight {weight} added to proposal '{proposal_id}'. New weight: {updated_proposals[proposal_id].signal_weight}")

        # After processing all intents, check if any proposals have passed the threshold
        for proposal in updated_proposals.values():
            if proposal.status == 'signaling' and proposal.signal_weight >= self.signal_threshold:
                proposal.status = 'passed'
                action = {
                    'type': 'execute_proposal',
                    'proposal_id': proposal.id,
                    'target': proposal.target_parameter,
                    'value': proposal.proposed_value
                }
                execution_actions.append(action)
                print(f"GovSolver: Proposal '{proposal.id}' has PASSED threshold!")

        return updated_proposals, execution_actions
