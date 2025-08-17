// SPDX-License-Identifier: AGPL-3.0-only
pragma solidity ^0.8.20;

import "../chronos_kernel/KernelRegistry.sol";
import "../chronos_kernel/EthicalSubstrate.sol";

/**
 * @title ChronosInterface
 * @author Architects of the Chronos Initiative
 * @notice This is the primary user-facing API for interacting with the Chronos engine.
 * It allows users to submit intractable problems for acausal computation,
 * tracks the status of these problems, and handles the fee mechanism.
 * This contract is designed for deployment on a QRASL General Execution shard (e.g., Shard 0 or 1).
 */
contract ChronosInterface {
    // Enum to track the status of a submitted problem.
    enum ProblemStatus { Submitted, Processing, Solved, Failed }

    // Struct to hold information about a submitted problem.
    struct Problem {
        bytes32 problemHash;      // A hash of the detailed problem specification.
        bytes32 conjectureRoot;   // The root of the M_Conjecture manifold where the problem is detailed.
        address submitter;        // The user who submitted the problem.
        uint256 fee;              // The fee paid to initiate the computation.
        ProblemStatus status;     // The current status of the problem.
        bytes32 proofRoot;        // The root of the M_Proof manifold containing the solution.
    }

    // A mapping from a unique problem ID to the Problem struct.
    mapping(bytes32 => Problem) public problems;

    // Counter for generating unique problem IDs.
    uint256 public problemCounter;

    // The address of the treasury contract where fees are sent.
    address public treasury;

    // The address of the KernelRegistry to check system status.
    KernelRegistry public immutable kernelRegistry;

    // --- Events ---
    event ProblemSubmitted(
        bytes32 indexed problemId,
        bytes32 indexed problemHash,
        address indexed submitter,
        uint256 fee
    );

    event ProblemStatusUpdated(
        bytes32 indexed problemId,
        ProblemStatus newStatus
    );

    event SolutionRegistered(
        bytes32 indexed problemId,
        bytes32 proofRoot
    );

    constructor(address _treasury, address _kernelRegistry) {
        treasury = _treasury;
        kernelRegistry = KernelRegistry(_kernelRegistry);
    }

    /**
     * @notice Submits a new problem to the Chronos engine.
     * @param problemHash A hash of the full problem specification.
     * @param conjectureRoot The root of the M_Conjecture manifold where the problem data is stored
     *                       (as per the Hyper-Dimensional State Manifold Storage Protocol).
     * @return problemId The unique ID for the newly submitted problem.
     *
     * The caller must send a fee in $QRASL tokens with this transaction.
     * The function checks if the Chronos Kernel is stable enough for new tasks.
     */
    function submitProblem(bytes32 problemHash, bytes32 conjectureRoot) external payable returns (bytes32) {
        // Basic check on kernel stability. In a real system, this would be more complex.
        require(kernelRegistry.coherence() > 90, "ChronosInterface: System coherence is too low.");
        require(msg.value > 0, "ChronosInterface: Fee must be paid to submit a problem.");

        // Transfer the fee to the treasury.
        (bool success, ) = treasury.call{value: msg.value}("");
        require(success, "ChronosInterface: Fee transfer failed.");

        bytes32 problemId = keccak256(abi.encodePacked(problemCounter, block.timestamp, msg.sender));
        problems[problemId] = Problem({
            problemHash: problemHash,
            conjectureRoot: conjectureRoot,
            submitter: msg.sender,
            fee: msg.value,
            status: ProblemStatus.Submitted,
            proofRoot: 0x0
        });

        problemCounter++;

        emit ProblemSubmitted(problemId, problemHash, msg.sender, msg.value);
        return problemId;
    }

    /**
     * @notice Allows a trusted entity (e.g., governance or a designated Chronos Node)
     *         to update the status of a problem.
     * @param problemId The ID of the problem to update.
     * @param newStatus The new status.
     *
     * This function should be restricted to authorized callers. For this conceptual implementation,
     * it is left open but would be governed by a role-based access control system.
     */
    function updateProblemStatus(bytes32 problemId, ProblemStatus newStatus) external /* onlyAuthorized */ {
        require(problems[problemId].submitter != address(0), "ChronosInterface: Problem does not exist.");
        problems[problemId].status = newStatus;
        emit ProblemStatusUpdated(problemId, newStatus);
    }

    /**
     * @notice Allows a trusted entity to register the solution (the M_Proof root) for a problem.
     * @param problemId The ID of the problem being solved.
     * @param proofRoot The root of the M_Proof manifold containing the solution.
     */
    function registerSolution(bytes32 problemId, bytes32 proofRoot) external /* onlyAuthorized */ {
        require(problems[problemId].submitter != address(0), "ChronosInterface: Problem does not exist.");
        require(problems[problemId].status == ProblemStatus.Processing, "ChronosInterface: Problem is not being processed.");

        problems[problemId].status = ProblemStatus.Solved;
        problems[problemId].proofRoot = proofRoot;

        emit SolutionRegistered(problemId, proofRoot);
        emit ProblemStatusUpdated(problemId, ProblemStatus.Solved);
    }
}
