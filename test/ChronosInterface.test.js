import { expect } from "chai";
import { ethers } from "hardhat";

describe("ChronosInterface", function () {
    let EthicalSubstrate, AxiomaticKernel, ChronosInterface;
    let ethicalSubstrate, axiomaticKernel, chronosInterface;
    let owner, treasury, authorizedNode, user1;
    let updateAxiom;

    beforeEach(async function () {
        [owner, treasury, authorizedNode, user1] = await ethers.getSigners();

        // Deploy EthicalSubstrate
        EthicalSubstrate = await ethers.getContractFactory("EthicalSubstrate");
        ethicalSubstrate = await EthicalSubstrate.deploy(owner.address);

        // Deploy AxiomaticKernel (KernelRegistry)
        AxiomaticKernel = await ethers.getContractFactory("KernelRegistry");
        axiomaticKernel = await AxiomaticKernel.deploy(await ethicalSubstrate.getAddress(), owner.address);

        // Deploy ChronosInterface
        ChronosInterface = await ethers.getContractFactory("ChronosInterface");
        chronosInterface = await ChronosInterface.deploy(treasury.address, await axiomaticKernel.getAddress());

        // Pre-authorize the node for testing
        await chronosInterface.connect(owner).setNodeAuthorization(authorizedNode.address, true);

        // Setup a valid axiom for updating kernel params
        updateAxiom = ethers.keccak256(ethers.toUtf8Bytes("UPDATE_PARAMS_AXIOM"));
        await ethicalSubstrate.ratifyAxiom(updateAxiom, "Allow kernel updates");
    });

    describe("Deployment and Setup", function () {
        it("Should set the correct treasury and kernel addresses", async function () {
            expect(await chronosInterface.treasury()).to.equal(treasury.address);
            expect(await chronosInterface.kernelRegistry()).to.equal(await axiomaticKernel.getAddress());
        });

        it("Should set the owner correctly", async function () {
            expect(await chronosInterface.owner()).to.equal(owner.address);
        });
    });

    describe("Access Control", function () {
        it("Should allow owner to authorize a node", async function () {
            await expect(chronosInterface.connect(owner).setNodeAuthorization(user1.address, true))
                .to.emit(chronosInterface, "NodeAuthorizationChanged")
                .withArgs(user1.address, true);
            expect(await chronosInterface.authorizedNodes(user1.address)).to.be.true;
        });

        it("Should fail if non-owner tries to authorize a node", async function () {
            await expect(chronosInterface.connect(user1).setNodeAuthorization(user1.address, true))
                .to.be.revertedWith("ChronosInterface: Caller is not the owner");
        });
    });

    describe("Problem Submission", function () {
        const problemHash = ethers.keccak256(ethers.toUtf8Bytes("P=NP?"));
        const conjectureRoot = ethers.keccak256(ethers.toUtf8Bytes("Initial conjecture data"));
        const fee = ethers.parseEther("1.0");

        it("Should allow a user to submit a problem with a fee", async function () {
            // Set high coherence for the test
            await axiomaticKernel.connect(owner).updateKernelParameters(100, 1, 100, 9999, 98, updateAxiom);

            const initialTreasuryBalance = await ethers.provider.getBalance(treasury.address);

            const tx = await chronosInterface.connect(user1).submitProblem(problemHash, conjectureRoot, { value: fee });
            const receipt = await tx.wait();
            const problemId = (await chronosInterface.queryFilter(chronosInterface.filters.ProblemSubmitted(), receipt.blockNumber))[0].args.problemId;

            await expect(tx).to.emit(chronosInterface, "ProblemSubmitted");

            const problem = await chronosInterface.problems(problemId);
            expect(problem.submitter).to.equal(user1.address);
            expect(problem.status).to.equal(0); // Enum Submitted

            const finalTreasuryBalance = await ethers.provider.getBalance(treasury.address);
            expect(finalTreasuryBalance - initialTreasuryBalance).to.equal(fee);
        });

        it("Should revert if coherence is too low", async function () {
            // Set low coherence
            await axiomaticKernel.connect(owner).updateKernelParameters(80, 1, 100, 9999, 98, updateAxiom);

            await expect(
                chronosInterface.connect(user1).submitProblem(problemHash, conjectureRoot, { value: fee })
            ).to.be.revertedWith("ChronosInterface: System coherence is too low.");
        });

        it("Should revert if no fee is paid", async function () {
            await axiomaticKernel.connect(owner).updateKernelParameters(100, 1, 100, 9999, 98, updateAxiom);
            await expect(
                chronosInterface.connect(user1).submitProblem(problemHash, conjectureRoot, { value: 0 })
            ).to.be.revertedWith("ChronosInterface: Fee must be paid.");
        });
    });

    describe("Solution Registration", function () {
        let problemId;
        const problemHash = ethers.keccak256(ethers.toUtf8Bytes("P=NP?"));
        const conjectureRoot = ethers.keccak256(ethers.toUtf8Bytes("Initial conjecture data"));
        const proofRoot = ethers.keccak256(ethers.toUtf8Bytes("The proof is trivial."));
        const fee = ethers.parseEther("1.0");

        beforeEach(async function() {
            await axiomaticKernel.connect(owner).updateKernelParameters(100, 1, 100, 9999, 98, updateAxiom);
            const tx = await chronosInterface.connect(user1).submitProblem(problemHash, conjectureRoot, { value: fee });
            const receipt = await tx.wait();
            problemId = (await chronosInterface.queryFilter(chronosInterface.filters.ProblemSubmitted(), receipt.blockNumber))[0].args.problemId;
        });

        it("Should allow an authorized node to update status and register a solution", async function () {
            await expect(chronosInterface.connect(authorizedNode).updateProblemStatus(problemId, 1)) // 1 = Processing
                .to.emit(chronosInterface, "ProblemStatusUpdated").withArgs(problemId, 1);

            await expect(chronosInterface.connect(authorizedNode).registerSolution(problemId, proofRoot))
                .to.emit(chronosInterface, "SolutionRegistered").withArgs(problemId, proofRoot)
                .and.to.emit(chronosInterface, "ProblemStatusUpdated").withArgs(problemId, 2); // 2 = Solved

            const problem = await chronosInterface.problems(problemId);
            expect(problem.status).to.equal(2); // Solved
            expect(problem.proofRoot).to.equal(proofRoot);
        });

        it("Should fail if a non-authorized node tries to update status", async function () {
            await expect(chronosInterface.connect(user1).updateProblemStatus(problemId, 1))
                .to.be.revertedWith("ChronosInterface: Caller is not an authorized node");
        });

        it("Should fail if trying to register a solution for a problem not in 'Processing' state", async function () {
            // Status is 'Submitted' (0), not 'Processing' (1)
            await expect(chronosInterface.connect(authorizedNode).registerSolution(problemId, proofRoot))
                .to.be.revertedWith("ChronosInterface: Problem is not being processed.");
        });
    });
});
