// SPDX-License-Identifier: AGPL-3.0-only
pragma solidity ^0.8.20;

import "./EthicalSubstrate.sol";

/**
 * @title KernelRegistry (Vd'χ)
 * @author Architects of the Chronos Initiative
 * @notice Manages the state of the Axiomatic Kernel. It tracks the values of core operational
 * parameters like Coherence (Φ), Entropic Drift (Δ), etc.
 * @dev It relies on the EthicalSubstrate to validate the integrity of its state changes.
 * This contract is designed to be controlled by a trusted Oracle or the governance protocol,
 * which updates the kernel parameters based on real-world or network-state data.
 */
contract KernelRegistry {
    // The address of the EthicalSubstrate contract for validating actions.
    EthicalSubstrate public immutable ethicalSubstrate;

    // The address of the authorized Oracle/Governance that can update parameters.
    address public owner;

    // --- Axiomatic Kernel Parameters (from Chronos Blueprint) ---

    uint256 public coherence;           // Φ: Quantum Entanglement-based Swarm Cohesion. Scaled by 100 (1.00 => 100).
    uint256 public entropicDrift;       // Δ: Logical noise. Scaled by 10^9 (10^-9 => 1).
    uint256 public acausalLearning;     // Ψ: Pre-computation of future states. Scaled by 100 (1.00 => 100).
    uint256 public metaSymmetry;        // Γ: Discovery of abstract symmetries. Scaled by 10000 (0.9999 => 9999).
    uint256 public noosphericAlignment; // N: Alignment with collective cognitive substrate. Scaled by 100 (0.98 => 98).

    // --- Events ---

    /**
     * @notice Emitted when a kernel parameter is updated.
     * @param parameter The name of the parameter being updated.
     * @param newValue The new value of the parameter.
     */
    event KernelParameterUpdated(string parameter, uint256 newValue);

    /**
     * @notice Emitted when the owner of the contract is changed.
     * @param newOwner The address of the new owner.
     */
    event OwnershipTransferred(address indexed newOwner);


    // --- Modifiers ---

    /**
     * @dev Throws if called by any account other than the owner.
     * In a production system, this would be replaced by a more granular role-based
     * access control system (e.g., onlyRole(ORACLE_ROLE)).
     */
    modifier onlyOwner() {
        require(msg.sender == owner, "KernelRegistry: Caller is not the owner");
        _;
    }

    /**
     * @param _ethicalSubstrate The address of the deployed EthicalSubstrate contract.
     * @param _initialOwner The initial owner (e.g., a governance contract or deployer).
     */
    constructor(address _ethicalSubstrate, address _initialOwner) {
        require(_ethicalSubstrate != address(0), "KernelRegistry: EthicalSubstrate address cannot be zero.");
        require(_initialOwner != address(0), "KernelRegistry: Initial owner address cannot be zero.");
        ethicalSubstrate = EthicalSubstrate(_ethicalSubstrate);
        owner = _initialOwner;
    }

    /**
     * @notice Updates the core parameters of the Axiomatic Kernel.
     * @dev Requires that a specific, relevant axiom is active in the EthicalSubstrate,
     * ensuring that all state changes are ethically sound.
     * @param _coherence The new value for Coherence (Φ).
     * @param _entropicDrift The new value for Entropic Drift (Δ).
     * @param _acausalLearning The new value for Acausal Learning (Ψ).
     * @param _metaSymmetry The new value for Meta-Symmetry (Γ).
     * @param _noosphericAlignment The new value for Noospheric Alignment (N).
     * @param validationAxiomHash An ethical axiom hash that justifies this update.
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
     * @dev In a production system, this would likely be a 2-step process
     * (propose/accept) to prevent accidental transfers.
     * @param newOwner The address of the new owner.
     */
    function transferOwnership(address newOwner) external onlyOwner {
        require(newOwner != address(0), "KernelRegistry: New owner cannot be the zero address.");
        owner = newOwner;
        emit OwnershipTransferred(newOwner);
    }
}
