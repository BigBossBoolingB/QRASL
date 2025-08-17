// SPDX-License-Identifier: AGPL-3.0-only
pragma solidity ^0.8.20;

/**
 * @title EthicalSubstrate (Λ)
 * @author Architects of the Chronos Initiative
 * @notice Establishes the formally verifiable, non-commutative ethical framework for the Chronos system.
 * @dev This contract defines the fundamental rules that govern all acausal computations. It is designed
 * to be deployed on QRASL's Governance Shard (Shard 6) and managed by a decentralized governance process.
 * The principles herein are designed to be immutable once locked, ensuring perpetual adherence to core ethical axioms.
 */
contract EthicalSubstrate {
    // The address of the main governance contract that can manage this substrate.
    address public immutable governance;

    // A mapping from an axiom's unique hash to its status (true = active).
    mapping(bytes32 => bool) public axioms;

    // Records whether the substrate has been permanently locked from further changes.
    bool public isLocked;

    // --- Events ---

    /**
     * @notice Emitted when a new ethical axiom is ratified by governance.
     * @param axiomHash The unique hash representing the axiom's principle.
     * @param description A human-readable description of the axiom.
     */
    event AxiomRatified(bytes32 indexed axiomHash, string description);

    /**
     * @notice Emitted when the substrate is locked, preventing any future modifications.
     */
    event SubstrateLocked();

    // --- Modifiers ---

    /**
     * @dev Throws if called by any account other than the designated governance contract.
     */
    modifier onlyGovernance() {
        require(msg.sender == governance, "EthicalSubstrate: Caller is not the governance contract");
        _;
    }

    /**
     * @dev Throws if the contract is in a locked state.
     */
    modifier notLocked() {
        require(!isLocked, "EthicalSubstrate: The substrate is locked and immutable");
        _;
    }

    /**
     * @param _governance The address of the governance contract.
     */
    constructor(address _governance) {
        require(_governance != address(0), "EthicalSubstrate: Governance address cannot be zero.");
        governance = _governance;
    }

    /**
     * @notice Ratifies a new ethical axiom, making it active.
     * @param axiomHash A unique hash representing the axiom's principle.
     * @param description A human-readable description of the axiom.
     * @dev Can only be called by the governance contract before the substrate is locked.
     */
    function ratifyAxiom(bytes32 axiomHash, string calldata description) external onlyGovernance notLocked {
        require(!axioms[axiomHash], "EthicalSubstrate: Axiom already exists");
        axioms[axiomHash] = true;
        emit AxiomRatified(axiomHash, description);
    }

    /**
     * @notice Locks the substrate, making all ratified axioms permanent and immutable.
     * @dev This is a one-way operation and cannot be undone. Can only be called by governance.
     */
    function lockSubstrate() external onlyGovernance notLocked {
        isLocked = true;
        emit SubstrateLocked();
    }

    /**
     * @notice Verifies if a specific axiom is currently active and ratified.
     * @param axiomHash The hash of the axiom to check.
     * @return bool True if the axiom is active, false otherwise.
     */
    function isAxiomActive(bytes32 axiomHash) external view returns (bool) {
        return axioms[axiomHash];
    }
}
