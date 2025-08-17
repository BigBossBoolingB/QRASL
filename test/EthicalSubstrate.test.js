import { expect } from "chai";
import { ethers } from "hardhat";

describe("EthicalSubstrate", function () {
    let EthicalSubstrate;
    let ethicalSubstrate;
    let owner;
    let addr1;

    beforeEach(async function () {
        [owner, addr1] = await ethers.getSigners();
        EthicalSubstrate = await ethers.getContractFactory("EthicalSubstrate");
        ethicalSubstrate = await EthicalSubstrate.deploy(owner.address);
    });

    describe("Deployment", function () {
        it("Should set the right governance address", async function () {
            expect(await ethicalSubstrate.governance()).to.equal(owner.address);
        });

        it("Should fail if governance address is zero", async function () {
            await expect(EthicalSubstrate.deploy(ethers.ZeroAddress)).to.be.revertedWith(
                "EthicalSubstrate: Governance address cannot be zero."
            );
        });

        it("Should not be locked initially", async function () {
            expect(await ethicalSubstrate.isLocked()).to.be.false;
        });
    });

    describe("Axiom Ratification", function () {
        const axiomHash = ethers.keccak256(ethers.toUtf8Bytes("AXIOM_1"));
        const axiomDescription = "Test Axiom 1";

        it("Should allow governance to ratify a new axiom", async function () {
            await expect(ethicalSubstrate.ratifyAxiom(axiomHash, axiomDescription))
                .to.emit(ethicalSubstrate, "AxiomRatified")
                .withArgs(axiomHash, axiomDescription);

            expect(await ethicalSubstrate.axioms(axiomHash)).to.be.true;
            expect(await ethicalSubstrate.isAxiomActive(axiomHash)).to.be.true;
        });

        it("Should fail if a non-governance account tries to ratify an axiom", async function () {
            await expect(
                ethicalSubstrate.connect(addr1).ratifyAxiom(axiomHash, axiomDescription)
            ).to.be.revertedWith("EthicalSubstrate: Caller is not the governance contract");
        });

        it("Should fail to ratify an already existing axiom", async function () {
            await ethicalSubstrate.ratifyAxiom(axiomHash, axiomDescription);
            await expect(
                ethicalSubstrate.ratifyAxiom(axiomHash, axiomDescription)
            ).to.be.revertedWith("EthicalSubstrate: Axiom already exists");
        });
    });

    describe("Substrate Locking", function () {
        it("Should allow governance to lock the substrate", async function () {
            await expect(ethicalSubstrate.lockSubstrate())
                .to.emit(ethicalSubstrate, "SubstrateLocked");

            expect(await ethicalSubstrate.isLocked()).to.be.true;
        });

        it("Should fail if a non-governance account tries to lock the substrate", async function () {
            await expect(
                ethicalSubstrate.connect(addr1).lockSubstrate()
            ).to.be.revertedWith("EthicalSubstrate: Caller is not the governance contract");
        });

        it("Should fail to ratify axioms after the substrate is locked", async function () {
            await ethicalSubstrate.lockSubstrate();
            const newAxiomHash = ethers.keccak256(ethers.toUtf8Bytes("NEW_AXIOM"));
            await expect(
                ethicalSubstrate.ratifyAxiom(newAxiomHash, "A new axiom")
            ).to.be.revertedWith("EthicalSubstrate: The substrate is locked and immutable");
        });

        it("Should fail to lock the substrate if it is already locked", async function () {
            await ethicalSubstrate.lockSubstrate();
            await expect(
                ethicalSubstrate.lockSubstrate()
            ).to.be.revertedWith("EthicalSubstrate: The substrate is locked and immutable");
        });
    });
});
