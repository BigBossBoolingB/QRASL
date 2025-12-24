//! The `validation` module contains functions for validating blocks and transactions.

use crate::core::state::ShardState;
use crate::core::types::SimplerAdaptiveDAGBlock;

/// Validates a block and its transactions.
pub fn validate_block(block: &SimplerAdaptiveDAGBlock, state: &mut ShardState) -> Result<(), &'static str> {
    // 1. Verify the block's own hash
    let expected_hash = block.calculate_hash();
    if block.block_hash != expected_hash {
        return Err("Block hash is invalid");
    }

    // 2. Validate and apply each transaction in the block
    // A temporary state is used to ensure that the block is all-or-nothing.
    let mut temp_state = state.clone();
    for tx in &block.transactions {
        if let Err(e) = temp_state.validate_and_apply_transaction(tx) {
            return Err(e);
        }
    }

    // 3. If all transactions are valid, apply the changes to the main state
    *state = temp_state;

    Ok(())
}
