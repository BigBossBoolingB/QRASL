// SPDX-License-Identifier: AGPL-3.0-only
pragma solidity ^0.8.20;

/**
 * @title EthicalSubstrate
 * @author Architects of the Chronos Initiative
 * @notice This contract establishes the formally verifiable, non-commutative ethical framework (Λ)
 * for the Chronos system. It defines the fundamental rules and constraints that govern
 * all acausal computations. It is designed to be deployed on QRASL's Governance Shard (Shard 6)
 * and managed by the decentralized governance process.
 *
 * The principles herein are designed to be immutable once locked, ensuring perpetual
 * adherence to the core ethical axioms.
 */
contract EthicalSubstrate {
    // The address of the governance contract that can manage this substrate.
    address public immutable governance;

    // A mapping from an axiom's hash to its status (true = active, false = revoked).
    mapping(bytes32 => bool) public axioms;

    // Event emitted when a new axiom is ratified.
    event AxiomRatified(bytes32 indexed axiomHash, string description);

    // Event emitted when the substrate is locked, preventing further changes.
    event SubstrateLocked();

    bool public isLocked = false;

    modifier onlyGovernance() {
        require(msg.sender == governance, "EthicalSubstrate: Caller is not the governance contract");
        _;
    }

    modifier notLocked() {
        require(!isLocked, "EthicalSubstrate: The substrate is locked and immutable");
        _;
    }

    constructor(address _governance) {
        governance = _governance;
    }

    /**
     * @notice Ratifies a new ethical axiom.
     * @param axiomHash A unique hash representing the axiom's principle.
     * @param description A human-readable description of the axiom.
     *
     * Can only be called by the governance contract before the substrate is locked.
     */
    function ratifyAxiom(bytes32 axiomHash, string calldata description) external onlyGovernance notLocked {
        require(!axioms[axiomHash], "EthicalSubstrate: Axiom already exists");
        axioms[axiomHash] = true;
        emit AxiomRatified(axiomHash, description);
    }

    /**
     * @notice Locks the substrate, making all ratified axioms permanent and immutable.
     * This is a one-way operation.
     *
     * Can only be called by the governance contract.
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
