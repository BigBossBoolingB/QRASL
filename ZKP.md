# Zero-Knowledge Proof (ZKP) Implementation on QRASL

## 1. Introduction

Zero-Knowledge Proofs (ZKPs) are a fundamental technology for QRASL, enabling scalability, privacy, and interoperability. This document outlines the specific ZKP schemes that will be used for various components of the QRASL network.

## 2. Cross-Shard Message Bus

The cross-shard message bus will use a **recursive ZKP system** to ensure the integrity and confidentiality of cross-shard communication. Specifically, we will use a **PLONK-based proof system** with a custom gate for recursion.

### Rationale

- **Recursion:** A recursive ZKP system allows us to create a proof of a proof, which is essential for efficiently verifying the state of the entire network.
- **PLONK:** PLONK is a universal and updatable proof system, which means that we can use the same proving system for different types of transactions without having to perform a new trusted setup for each one.
- **Performance:** PLONK offers a good balance between prover time, verifier time, and proof size.

## 3. Micro-Rollups

The Micro-Rollups on the IHDB shards will use a **ZK-STARK** (Zero-Knowledge Scalable Transparent Argument of Knowledge) based system.

### Rationale

- **Scalability:** ZK-STARKs are highly scalable, with logarithmic proof size and verification time. This is ideal for the high-throughput environment of the Micro-Rollups.
- **Transparency:** ZK-STARKs do not require a trusted setup, which improves the security and decentralization of the system.
- **Quantum Resistance:** ZK-STARKs are believed to be resistant to attacks from quantum computers, which aligns with the overall security goals of QRASL.

## 4. Verifiable Off-Chain Computation (VOC)

For Verifiable Off-Chain Computation, we will use a **Groth16-based proof system**.

### Rationale

- **Succinctness:** Groth16 proofs are very small and fast to verify, which is important for on-chain verification of off-chain computation.
- **Maturity:** Groth16 is a mature and well-understood proof system with a wide range of existing tools and libraries.
- **Efficiency:** While Groth16 requires a trusted setup for each new type of computation, it is highly efficient for the prover and verifier, making it a good choice for VOC.

## 5. ZK Proof Market

The Open ZK Proof Market will be designed to be agnostic to the underlying proof system. This will allow ZK Miners to use the most efficient hardware and software for each type of proof, fostering a competitive and innovative market for ZKP generation.

## 6. Implementation

The implementation of the ZKP systems will be done in a modular and extensible way. We will use existing libraries and frameworks where possible, and will contribute to the development of new tools and techniques for ZKP engineering. All ZKP implementations will be subject to a rigorous security audit before being deployed on the mainnet.
