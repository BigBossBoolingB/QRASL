# QRASL Consensus Mechanism

This document outlines the hybrid consensus mechanism of the QRASL blockchain, detailing the processes for validator management, block production, and achieving global finality.

## 1. Validator Lifecycle & Selection

The security and integrity of the QRASL network are upheld by a dynamic and permissionless set of validators. The lifecycle of a validator is governed by a clear set of rules enforced by the Beacon Chain.

### 1.1. Staking

- **Action:** To become a validator candidate, a node operator must send a special `stake` transaction to a system-level contract on the Beacon Chain.
- **Requirement:** The transaction must lock up a minimum required amount of `$QRASL` as collateral. This stake serves as a security deposit, demonstrating a commitment to the network's health.
- **Effect:** Upon successful processing of the `stake` transaction, the node's public key is added to the "validator candidate pool."

### 1.2. Registration & Entering the Active Set

- **Process:** The Beacon Chain periodically (e.g., every epoch) selects new validators from the candidate pool to join the "active validator set."
- **Selection Criteria:** The selection is based on the size of the stake (higher stake increases probability) and the overall needs of the network (e.g., maintaining a target number of validators).
- **Activation:** Once selected, a validator enters a queue and becomes active after a defined "activation period." This delay prevents rapid, potentially malicious changes to the validator set.

### 1.3. Shard Assignment

- **Mechanism:** For each epoch, the Beacon Chain is responsible for assigning validators from the active set to each of the 7 shards.
- **Technology:** The assignment process utilizes a combination of:
    - **Proof-of-Stake (PoS):** The amount of stake a validator holds influences their probability of being selected for a committee.
    - **Verifiable Random Functions (VRFs):** A VRF is used to generate a pseudorandom, yet verifiable, output. This output is used to shuffle the validator set and assign them to shards in a deterministic but unpredictable way, preventing collusion and targeted attacks.
- **Rotation:** Validator assignments are rotated every epoch to enhance security and distribute the workload.

### 1.4. Slashing (Penalties)

- **Purpose:** Slashing is the primary mechanism for punishing validators for malicious actions or gross negligence, thereby securing the network against attacks.
- **Conditions for Slashing:**
    - **Double-Signing:** Proposing and signing two different blocks for the same slot/height within a shard.
    - **Contradictory Finality Votes:** Submitting conflicting votes to the Beacon Chain's finality gadget.
    - **Prolonged Downtime:** Failing to participate in consensus (e.g., being offline) for an extended period, which harms the network's liveness.
- **Process:** Evidence of a slashable offense (e.g., two conflicting signed block headers) can be submitted to the Beacon Chain by any node. If the evidence is verified, the offending validator is penalized.
- **Penalty:**
    - A significant portion of the validator's stake is burned (`slashed`).
    - The validator is forcibly removed from the active set and placed in a "slashed" state, preventing them from rejoining for a long period.

## 2. Block Proposal & Validation Flow (Shard-Level)

Within each shard, validators work together to build and agree upon the local chain of blocks. This process is designed for high throughput and rapid confirmation.

### 2.1. Block Proposal

- **Designation:** At each "slot" (a discrete time interval), a single validator from the shard's assigned committee is designated as the block proposer. This designation is also determined by the VRF output for that epoch.
- **Assembly:** The proposer gathers a set of pending transactions from the shard's mempool, as well as any relevant data for the specific shard type (e.g., intents and solutions for an IHDB shard, or ZKP verification requests for the auxiliary shard).
- **Creation:** The proposer constructs a new block, populating the `BlockHeader` with the correct `parent_hash`, `transactions_root`, `state_root`, etc., and signs the block with its private key.
- **Broadcast:** The newly created block is broadcast to all other validators within the same shard.

### 2.2. Validation Rules

Upon receiving a new block, every other validator in the shard's committee must independently verify its validity by performing a series of checks:
- **Signature Verification:** The block's signature must be valid and must belong to the designated proposer for the current slot.
- **Parent Hash:** The `parent_hash` must point to a known and valid block in the shard's local DAG.
- **Transaction Validity:** Each transaction within the block must be valid (correct signature, sufficient funds, valid nonce).
- **State Transition:** The `state_root` in the header must match the result of re-executing all transactions in the block against the parent block's state. This is the most computationally intensive step.
- **Structural Integrity:** The block must conform to the rules of its type (e.g., IHDBs must have valid causality proofs and Micro-Rollup commitments).

### 2.3. Intra-Shard Consensus

- **Protocol:** QRASL shards use a DAG-based consensus protocol (e.g., a variant of GHOSTDAG) to establish a local ordering of blocks.
- **Mechanism:** Unlike a simple linear chain, validators can vote for multiple "tips" (leaf blocks) in the DAG. The protocol's fork-choice rule uses these votes to determine the "heaviest" chain, which is considered the canonical one.
- **Advantage:** This DAG structure allows for parallel block production, increasing throughput and resilience to network latency, as blocks can be produced without waiting for a single, definitive parent. It provides fast, local confirmation, even if the global network finality takes longer.

## 3. Global Finality (Beacon Chain-Level)

While shards provide fast, local consensus, true irreversibility is achieved at the global level by the Beacon Chain. This ensures that the history of the entire QRASL network is consistent and permanent.

### 3.1. Checkpointing

- **Process:** At regular intervals (e.g., every 64 slots), each shard is required to produce a "checkpoint." A checkpoint is a block that summarizes the state of the shard up to that point.
- **Content:** The most critical piece of information in a checkpoint is the `state_root` of the shard's canonical chain, as determined by its local DAG consensus.
- **Submission:** The shard's validators collectively sign this checkpoint, and it is submitted to the Beacon Chain via the ZKP-Recursive Cross-Shard Message Bus.

### 3.2. Finality Gadget

- **Protocol:** The Beacon Chain employs a finality gadget, such as a variant of Casper the Friendly Finality Gadget (FFG), to achieve global consensus on the state of the network.
- **Voting:** Beacon Chain validators observe the checkpoints submitted by all 7 shards. They cast votes on pairs of checkpoints (source and target) that they believe are valid and should be finalized.
- **Justification & Finalization:**
    - A checkpoint becomes **justified** when more than 2/3 of the total staked `$QRASL` (represented by the Beacon Chain validators) have voted for the link between it and a previously justified checkpoint.
    - A checkpoint becomes **finalized** when it is justified, and a subsequent checkpoint that considers it a parent also becomes justified.

### 3.3. The Point of No Return

- **Definition:** A transaction is considered **finalized** and completely irreversible once the block containing it is part of a shard's history that has been included in a finalized checkpoint on the Beacon Chain.
- **Security Guarantee:** To reverse a finalized transaction, an attacker would need to get more than 1/3 of the entire network's staked `$QRASL` to be slashed by making conflicting finality votes, and then gain control of over 2/3 of the stake to finalize a new, conflicting history. This is considered economically infeasible.
- **User Experience:** While a transaction is quickly confirmed at the shard level (a few seconds), users must wait for the next finality cycle on the Beacon Chain (a few minutes) for the highest level of security and assurance that their transaction is permanent.
