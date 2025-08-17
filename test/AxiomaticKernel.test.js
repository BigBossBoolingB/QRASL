import { expect } from "chai";
import { ethers } from "hardhat";

describe("AxiomaticKernel (KernelRegistry)", function () {
    let EthicalSubstrate, AxiomaticKernel;
    let ethicalSubstrate, axiomaticKernel;
    let owner, addr1;
    let validAxiomHash, invalidAxiomHash;

    beforeEach(async function () {
        [owner, addr1] = await ethers.getSigners();

        // Deploy EthicalSubstrate first
        EthicalSubstrate = await ethers.getContractFactory("EthicalSubstrate");
        ethicalSubstrate = await EthicalSubstrate.deploy(owner.address);

        // Deploy AxiomaticKernel, linking it to the EthicalSubstrate
        AxiomaticKernel = await ethers.getContractFactory("KernelRegistry");
        axiomaticKernel = await AxiomaticKernel.deploy(await ethicalSubstrate.getAddress(), owner.address);

        // Setup a valid axiom for testing
        validAxiomHash = ethers.keccak256(ethers.toUtf8Bytes("UPDATE_PARAMS_AXIOM"));
        invalidAxiomHash = ethers.keccak256(ethers.toUtf8Bytes("INVALID_AXIOM"));
        await ethicalSubstrate.ratifyAxiom(validAxiomHash, "Axiom for updating kernel parameters");
    });

    describe("Deployment", function () {
        it("Should set the correct EthicalSubstrate address", async function () {
            expect(await axiomaticKernel.ethicalSubstrate()).to.equal(await ethicalSubstrate.getAddress());
        });

        it("Should set the correct owner", async function () {
            expect(await axiomaticKernel.owner()).to.equal(owner.address);
        });

        it("Should fail if EthicalSubstrate address is zero", async function () {
            await expect(AxiomaticKernel.deploy(ethers.ZeroAddress, owner.address)).to.be.revertedWith(
                "KernelRegistry: EthicalSubstrate address cannot be zero."
            );
        });
    });

    describe("Ownership", function () {
        it("Should allow the owner to transfer ownership", async function () {
            await expect(axiomaticKernel.transferOwnership(addr1.address))
                .to.emit(axiomaticKernel, "OwnershipTransferred")
                .withArgs(addr1.address);
            expect(await axiomaticKernel.owner()).to.equal(addr1.address);
        });

        it("Should fail if a non-owner tries to transfer ownership", async function () {
            await expect(axiomaticKernel.connect(addr1).transferOwnership(addr1.address))
                .to.be.revertedWith("KernelRegistry: Caller is not the owner");
        });
    });

    describe("Parameter Updates", function () {
        const newParams = {
            coherence: 99,
            entropicDrift: 2,
            acausalLearning: 98,
            metaSymmetry: 9998,
            noosphericAlignment: 97,
        };

        it("Should allow the owner to update parameters with a valid axiom", async function () {
            const tx = await axiomaticKernel.updateKernelParameters(
                newParams.coherence,
                newParams.entropicDrift,
                newParams.acausalLearning,
                newParams.metaSymmetry,
                newParams.noosphericAlignment,
                validAxiomHash
            );

            await expect(tx).to.emit(axiomaticKernel, "KernelParameterUpdated").withArgs("Coherence", newParams.coherence);
            await expect(tx).to.emit(axiomaticKernel, "KernelParameterUpdated").withArgs("NoosphericAlignment", newParams.noosphericAlignment);

            expect(await axiomaticKernel.coherence()).to.equal(newParams.coherence);
            expect(await axiomaticKernel.noosphericAlignment()).to.equal(newParams.noosphericAlignment);
        });

        it("Should fail if a non-owner tries to update parameters", async function () {
            await expect(axiomaticKernel.connect(addr1).updateKernelParameters(
                newParams.coherence,
                newParams.entropicDrift,
                newParams.acausalLearning,
                newParams.metaSymmetry,
                newParams.noosphericAlignment,
                validAxiomHash
            )).to.be.revertedWith("KernelRegistry: Caller is not the owner");
        });

        it("Should fail if an invalid or non-active axiom is provided", async function () {
            await expect(axiomaticKernel.updateKernelParameters(
                newParams.coherence,
                newParams.entropicDrift,
                newParams.acausalLearning,
                newParams.metaSymmetry,
                newParams.noosphericAlignment,
                invalidAxiomHash
            )).to.be.revertedWith("KernelRegistry: Update is not justified by an active ethical axiom.");
        });
    });
});
