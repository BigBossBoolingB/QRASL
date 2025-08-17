//! Solver Role
//!
//! Responsible for observing user intents on IHDB shards, finding optimal
//! execution paths using hyper-computation, and submitting solutions.

// --- Core Data Structures ---

/// Represents a user's declared intent (e.g., "solve this intractable problem").
/// The `payload` would point to data stored off-chain as per the HSMSP.
#[derive(Debug, Clone, PartialEq)]
pub struct Intent {
    pub intent_id: String,
    pub user: String,
    pub payload_hash: [u8; 32],
}

/// Represents a proposed solution to an intent.
/// The `proof_root` would be the root of the M_Proof manifold for this solution.
#[derive(Debug, Clone)]
pub struct Solution {
    pub intent_id: String,
    pub solver_id: u64,
    pub proof_root: [u8; 32],
    pub confidence: f64, // Solver's confidence in the solution
}

// --- Solver Implementation ---

/// Represents a Solver node in the QRASL network.
pub struct Solver {
    pub id: u64,
    // A conceptual representation of the solver's computational capacity.
    computational_power: u64,
}

/// The trait defining the core functionalities of a Solver.
pub trait SolverActions {
    /// Observes a list of available intents and selects one to work on.
    fn select_intent(&self, intents: &[Intent]) -> Option<Intent>;

    /// The core hyper-computation logic to find a solution for an intent.
    fn find_solution(&self, intent: &Intent) -> Solution;
}

impl SolverActions for Solver {
    /// A simple selection strategy: take the first intent.
    /// A real implementation would have complex economic logic.
    fn select_intent(&self, intents: &[Intent]) -> Option<Intent> {
        println!("Solver {}: Observing {} intents...", self.id, intents.len());
        intents.first().cloned()
    }

    /// This function represents the core "magic" of the Chronos engine.
    /// It takes an intent and uses conceptual hyper-computation to find a solution.
    fn find_solution(&self, intent: &Intent) -> Solution {
        println!("Solver {}: Beginning hyper-computation for intent {}...", self.id, intent.intent_id);

        // --- CONCEPTUAL HYPER-COMPUTATION PLACEHOLDER ---
        // In a real system, this would be an incredibly complex process involving
        // the QPU and Neuromorphic coprocessors of the HCN.
        // For now, we simulate the result of this computation.

        // Simulate a variable time-to-solution based on computational power.
        let solve_time_ms = 1000 / self.computational_power;
        std::thread::sleep(std::time::Duration::from_millis(solve_time_ms));

        // Generate a deterministic, conceptual "proof" based on the intent's data.
        let mut proof_root = [0u8; 32];
        for i in 0..32 {
            proof_root[i] = intent.payload_hash[i].wrapping_add((self.id as u8) + (i as u8));
        }
        // --- END PLACEHOLDER ---

        println!("Solver {}: Computation complete for intent {}.", self.id, intent.intent_id);

        Solution {
            intent_id: intent.intent_id.clone(),
            solver_id: self.id,
            proof_root,
            confidence: 0.99, // The solver is very confident in its hyper-computation.
        }
    }
}

// --- Unit Tests ---

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_solver() -> Solver {
        Solver {
            id: 42,
            computational_power: 10,
        }
    }

    fn get_test_intents() -> Vec<Intent> {
        vec![
            Intent {
                intent_id: "intent-001".to_string(),
                user: "user-a".to_string(),
                payload_hash: [1; 32],
            },
            Intent {
                intent_id: "intent-002".to_string(),
                user: "user-b".to_string(),
                payload_hash: [2; 32],
            },
        ]
    }

    #[test]
    fn test_solver_creation() {
        let solver = get_test_solver();
        assert_eq!(solver.id, 42);
        assert_eq!(solver.computational_power, 10);
    }

    #[test]
    fn test_intent_selection() {
        let solver = get_test_solver();
        let intents = get_test_intents();
        let selected_intent = solver.select_intent(&intents);

        assert!(selected_intent.is_some(), "Should have selected an intent");
        assert_eq!(selected_intent.unwrap(), intents[0]);
    }

    #[test]
    fn test_find_solution() {
        let solver = get_test_solver();
        let intents = get_test_intents();
        let intent = intents.first().unwrap();

        let solution = solver.find_solution(intent);

        assert_eq!(solution.intent_id, intent.intent_id);
        assert_eq!(solution.solver_id, solver.id);
        assert_eq!(solution.confidence, 0.99);

        // Check that the proof root is deterministically generated
        let mut expected_proof_root = [0u8; 32];
        for i in 0..32 {
            expected_proof_root[i] = intent.payload_hash[i].wrapping_add((solver.id as u8) + (i as u8));
        }
        assert_eq!(solution.proof_root, expected_proof_root);
    }
}
