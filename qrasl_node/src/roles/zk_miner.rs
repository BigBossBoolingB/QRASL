//! ZK-Miner (Prover) Role
//!
//! Responsible for generating Zero-Knowledge Proofs for various
//! network operations, such as Micro-Rollups, cross-shard messages,
//! and Verifiable Off-Chain Computation (VOC).

// --- Core Data Structures ---

/// A request for a proof to be generated.
/// `data_to_prove` would be a commitment to a set of computational steps or data.
#[derive(Debug, Clone)]
pub struct ProofRequest {
    pub request_id: String,
    pub data_to_prove: [u8; 32],
    pub proof_type: String, // e.g., "rollup", "voc", "message"
}

/// A generated Zero-Knowledge Proof.
/// `proof_data` is the actual, compact proof.
#[derive(Debug, Clone, PartialEq)]
pub struct ZeroKnowledgeProof {
    pub request_id: String,
    pub proof_data: Vec<u8>,
    pub prover_id: u64,
}

// --- ZK-Miner Implementation ---

/// Represents a ZK-Miner node in the QRASL network.
pub struct ZkMiner {
    pub id: u64,
    // Represents the miner's specialized hardware capacity for ZKP generation.
    proving_power: u64,
}

/// The trait defining the core functionalities of a ZK-Miner.
pub trait ZkMinerActions {
    /// The core proving logic. Takes a request and generates a ZK proof.
    fn generate_proof(&self, request: &ProofRequest) -> ZeroKnowledgeProof;
}

impl ZkMinerActions for ZkMiner {
    /// This function represents the core cryptographic work of the ZK-Miner.
    /// It takes a request and simulates the generation of a complex ZK-SNARK/STARK.
    fn generate_proof(&self, request: &ProofRequest) -> ZeroKnowledgeProof {
        println!("ZK-Miner {}: Starting proof generation for request {}...", self.id, request.request_id);

        // --- CONCEPTUAL PROOF GENERATION PLACEHOLDER ---
        // This simulates a computationally intensive task. The time taken
        // would depend on the complexity of the proof and the miner's hardware.
        let prove_time_ms = 2000 / self.proving_power;
        std::thread::sleep(std::time::Duration::from_millis(prove_time_ms));

        // Generate a deterministic, conceptual "proof" based on the input data.
        let mut proof_bytes = Vec::with_capacity(128);
        for i in 0..128 {
            let val = request.data_to_prove[i % 32].wrapping_add(self.id as u8).wrapping_add(i as u8);
            proof_bytes.push(val);
        }
        // --- END PLACEHOLDER ---

        println!("ZK-Miner {}: Proof generated for request {}.", self.id, request.request_id);

        ZeroKnowledgeProof {
            request_id: request.request_id.clone(),
            proof_data: proof_bytes,
            prover_id: self.id,
        }
    }
}

// --- Unit Tests ---

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_zk_miner() -> ZkMiner {
        ZkMiner {
            id: 77,
            proving_power: 20,
        }
    }

    fn get_test_proof_request() -> ProofRequest {
        ProofRequest {
            request_id: "rollup-batch-123".to_string(),
            data_to_prove: [5; 32],
            proof_type: "rollup".to_string(),
        }
    }

    #[test]
    fn test_zk_miner_creation() {
        let miner = get_test_zk_miner();
        assert_eq!(miner.id, 77);
        assert_eq!(miner.proving_power, 20);
    }

    #[test]
    fn test_proof_generation() {
        let miner = get_test_zk_miner();
        let request = get_test_proof_request();

        let proof = miner.generate_proof(&request);

        assert_eq!(proof.request_id, request.request_id);
        assert_eq!(proof.prover_id, miner.id);
        assert_eq!(proof.proof_data.len(), 128, "Proof data should have the correct length");

        // Check that the proof data is deterministically generated
        let mut expected_proof_bytes = Vec::with_capacity(128);
        for i in 0..128 {
            let val = request.data_to_prove[i % 32].wrapping_add(miner.id as u8).wrapping_add(i as u8);
            expected_proof_bytes.push(val);
        }
        assert_eq!(proof.proof_data, expected_proof_bytes);
    }
}
