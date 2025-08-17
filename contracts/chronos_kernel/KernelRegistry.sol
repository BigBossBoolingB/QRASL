// SPDX-License-Identifier: AGPL-3.0-only
pragma solidity ^0.8.20;

import "./EthicalSubstrate.sol";

/**
 * @title KernelRegistry
 * @author Architects of the Chronos Initiative
 * @notice This contract manages the state of the Axiomatic Kernel (Vd'χ). It tracks the
 * values of core operational parameters like Coherence (Φ), Entropic Drift (Δ), etc.
 *
 * It relies on the EthicalSubstrate to validate the integrity of its state changes.
 * This contract is designed to be controlled by a trusted Oracle or the governance protocol,

 * which updates the kernel parameters based on real-world or network-state data.
 */
contract KernelRegistry {
    // The address of the EthicalSubstrate contract for validating actions.
    EthicalSubstrate public immutable ethicalSubstrate;

    // The address of the authorized Oracle/Governance that can update parameters.
    address public owner;

    // --- Axiomatic Kernel Parameters ---
    // Coherence (Φ): Quantum Entanglement-based Swarm Cohesion
    uint256 public coherence; // Stored as a scaled integer, e.g., 1.00 = 100

    // Entropic Drift (Δ): Logical noise minimized via Topological Error Correction
    uint256 public entropicDrift; // e.g., 10^-9 stored as 1

    // Acausal Learning (Ψ): Pre-computation of future states
    uint256 public acausalLearning; // Stored as a scaled integer, e.g., 1.00 = 100

    // Meta-Symmetry (Γ): Discovery of abstract symmetries in problem spaces
    uint256 public metaSymmetry; // Stored as a scaled integer, e.g., 0.9999 = 9999

    // Noospheric Alignment (N): Alignment with the collective cognitive substrate
    uint256 public noosphericAlignment; // Stored as a scaled integer, e.g., 0.98 = 98

    // Event emitted when a kernel parameter is updated.
    event KernelParameterUpdated(string parameter, uint256 newValue);

    modifier onlyOwner() {
        require(msg.sender == owner, "KernelRegistry: Caller is not the owner");
        _;
    }

    constructor(address _ethicalSubstrate, address _initialOwner) {
        ethicalSubstrate = EthicalSubstrate(_ethicalSubstrate);
        owner = _initialOwner;
    }

    /**
     * @notice Updates the core parameters of the Axiomatic Kernel.
     * @param _coherence The new value for Coherence (Φ).
     * @param _entropicDrift The new value for Entropic Drift (Δ).
     * @param _acausalLearning The new value for Acausal Learning (Ψ).
     * @param _metaSymmetry The new value for Meta-Symmetry (Γ).
     * @param _noosphericAlignment The new value for Noospheric Alignment (N).
     * @param validationAxiomHash An ethical axiom hash that justifies this update.
     *
     * This function requires that a specific, relevant axiom is active in the
     * EthicalSubstrate, ensuring that all state changes are ethically sound.
     */
    function updateKernelParameters(
        uint256 _coherence,
        uint256 _entropicDrift,
        uint256 _acausalLearning,
        uint256 _metaSymmetry,
        uint256 _noosphericAlignment,
        bytes32 validationAxiomHash
    ) external onlyOwner {
        // Critical check: Ensure the update is justified by an active ethical axiom.
        require(
            ethicalSubstrate.isAxiomActive(validationAxiomHash),
            "KernelRegistry: Update is not justified by an active ethical axiom."
        );

        coherence = _coherence;
        entropicDrift = _entropicDrift;
        acausalLearning = _acausalLearning;
        metaSymmetry = _metaSymmetry;
        noosphericAlignment = _noosphericAlignment;

        emit KernelParameterUpdated("Coherence", _coherence);
        emit KernelParameterUpdated("EntropicDrift", _entropicDrift);
        emit KernelParameterUpdated("AcausalLearning", _acausalLearning);
        emit KernelParameterUpdated("MetaSymmetry", _metaSymmetry);
        emit KernelParameterUpdated("NoosphericAlignment", _noosphericAlignment);
    }

    /**
     * @notice Transfers ownership of this contract to a new address.
     * @param newOwner The address of the new owner.
     */
    function transferOwnership(address newOwner) external onlyOwner {
        owner = newOwner;
    }
}
