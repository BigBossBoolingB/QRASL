// SPDX-License-Identifier: AGPL-3.0-only
pragma solidity ^0.8.20;

import "../chronos_kernel/AxiomaticKernel.sol";

/**
 * @title ChronosInterface
 * @author Architects of the Chronos Initiative
 * @notice This is the primary user-facing API for interacting with the Chronos engine.
 * @dev It allows users to submit problems, tracks their status, and handles fees.
 * This contract is designed for deployment on a QRASL General Execution shard (e.g., Shard 0 or 1).
 */
contract ChronosInterface {
    // --- State Variables ---

    address public owner;
    address public treasury;
    KernelRegistry public immutable kernelRegistry;

    // A simple access control mechanism for authorized nodes (HCNs).
    // In production, this would be a more robust RBAC system.
    mapping(address => bool) public authorizedNodes;

    uint256 public problemCounter;
    mapping(bytes32 => Problem) public problems;

    // --- Structs and Enums ---

    enum ProblemStatus { Submitted, Processing, Solved, Failed }

    struct Problem {
        bytes32 problemHash;      // Hash of the detailed problem specification.
        bytes32 conjectureRoot;   // Root of the M_Conjecture manifold where the problem data is stored.
        address submitter;        // The user who submitted the problem.
        uint256 fee;              // Fee paid to initiate the computation.
        ProblemStatus status;     // The current status of the problem.
        bytes32 proofRoot;        // Root of the M_Proof manifold containing the solution.
    }

    // --- Events ---

    event OwnershipTransferred(address indexed newOwner);
    event NodeAuthorizationChanged(address indexed node, bool authorized);
    event ProblemSubmitted(bytes32 indexed problemId, bytes32 indexed problemHash, address indexed submitter, uint256 fee);
    event ProblemStatusUpdated(bytes32 indexed problemId, ProblemStatus newStatus);
    event SolutionRegistered(bytes32 indexed problemId, bytes32 proofRoot);

    // --- Modifiers ---

    modifier onlyOwner() {
        require(msg.sender == owner, "ChronosInterface: Caller is not the owner");
        _;
    }

    modifier onlyAuthorizedNode() {
        require(authorizedNodes[msg.sender], "ChronosInterface: Caller is not an authorized node");
        _;
    }

    // --- Functions ---

    constructor(address _treasury, address _kernelRegistry) {
        require(_treasury != address(0), "ChronosInterface: Treasury address cannot be zero.");
        require(_kernelRegistry != address(0), "ChronosInterface: KernelRegistry address cannot be zero.");

        owner = msg.sender;
        treasury = _treasury;
        kernelRegistry = KernelRegistry(_kernelRegistry);
        emit OwnershipTransferred(msg.sender);
    }

    /**
     * @notice Submits a new problem to the Chronos engine.
     * @param problemHash A hash of the full problem specification.
     * @param conjectureRoot The root of the M_Conjecture manifold where the problem data is stored.
     * @return problemId The unique ID for the newly submitted problem.
     * @dev The caller must send a fee in $QRASL. The function checks kernel stability before accepting.
     */
    function submitProblem(bytes32 problemHash, bytes32 conjectureRoot) external payable returns (bytes32) {
        require(kernelRegistry.coherence() > 90, "ChronosInterface: System coherence is too low.");
        require(msg.value > 0, "ChronosInterface: Fee must be paid.");

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
     * @notice Allows an authorized node to update the status of a problem.
     * @param problemId The ID of the problem to update.
     * @param newStatus The new status.
     */
    function updateProblemStatus(bytes32 problemId, ProblemStatus newStatus) external onlyAuthorizedNode {
        Problem storage problem = problems[problemId];
        require(problem.submitter != address(0), "ChronosInterface: Problem does not exist.");
        problem.status = newStatus;
        emit ProblemStatusUpdated(problemId, newStatus);
    }

    /**
     * @notice Allows an authorized node to register the solution for a problem.
     * @param problemId The ID of the problem being solved.
     * @param proofRoot The root of the M_Proof manifold containing the solution.
     */
    function registerSolution(bytes32 problemId, bytes32 proofRoot) external onlyAuthorizedNode {
        Problem storage problem = problems[problemId];
        require(problem.submitter != address(0), "ChronosInterface: Problem does not exist.");
        require(problem.status == ProblemStatus.Processing, "ChronosInterface: Problem is not being processed.");

        problem.status = ProblemStatus.Solved;
        problem.proofRoot = proofRoot;

        emit SolutionRegistered(problemId, proofRoot);
        emit ProblemStatusUpdated(problemId, ProblemStatus.Solved);
    }

    // --- Admin Functions ---

    /**
     * @notice Authorizes or de-authorizes a node to update problem statuses and register solutions.
     * @param node The address of the node.
     * @param authorized The authorization status.
     */
    function setNodeAuthorization(address node, bool authorized) external onlyOwner {
        authorizedNodes[node] = authorized;
        emit NodeAuthorizationChanged(node, authorized);
    }

    /**
     * @notice Transfers ownership of this contract.
     * @param newOwner The address of the new owner.
     */
    function transferOwnership(address newOwner) external onlyOwner {
        require(newOwner != address(0), "ChronosInterface: New owner cannot be the zero address.");
        owner = newOwner;
        emit OwnershipTransferred(newOwner);
    }
}
