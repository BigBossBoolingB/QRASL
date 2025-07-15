# Post-Quantum Cryptography (PQC) on QRASL

## 1. Introduction

As a quantum-resistant blockchain, QRASL will use Post-Quantum Cryptography (PQC) for all of its cryptographic primitives. This document outlines the recommended PQC algorithm suite for digital signatures and key exchange.

## 2. Digital Signatures

For digital signatures, we recommend the use of **CRYSTALS-Dilithium**. Dilithium is a lattice-based digital signature scheme that has been selected by the National Institute of Standards and Technology (NIST) as a primary standard for PQC.

### Rationale

- **Security:** Dilithium is based on the well-studied hardness of lattice problems, which are believed to be resistant to attacks from both classical and quantum computers.
- **Performance:** Dilithium offers a good balance between signature size, public key size, and performance. This is important for a high-throughput blockchain like QRASL.
- **Standardization:** As a NIST standard, Dilithium has been extensively studied and vetted by the cryptographic community.

## 3. Key Exchange

For key exchange, we recommend the use of **CRYSTALS-Kyber**. Kyber is a lattice-based key encapsulation mechanism (KEM) that has also been selected by NIST as a primary standard for PQC.

### Rationale

- **Security:** Like Dilithium, Kyber is based on the hardness of lattice problems and is believed to be secure against quantum attacks.
- **Performance:** Kyber is one of the most performant KEMs in the NIST PQC competition, with small key and ciphertext sizes.
- **Standardization:** As a NIST standard, Kyber is a trusted and well-understood choice for PQC.

## 4. Hash-Based Signatures

As a backup and for specific use cases where a different security trade-off is desired, we will also support the use of **SPHINCS+**. SPHINCS+ is a stateless hash-based signature scheme that is also a NIST PQC standard.

### Rationale

- **Security:** The security of SPHINCS+ is based on the security of the underlying hash function, which is a very well-understood cryptographic primitive.
- **Minimal Assumptions:** Hash-based signatures require minimal security assumptions, making them a robust choice.
- **Different Trust Assumptions:** By supporting both lattice-based and hash-based signatures, QRASL provides a more resilient and diverse cryptographic foundation.

## 5. Implementation

The implementation of the PQC algorithm suite will be done in a modular and extensible way. This will allow for the easy addition of new algorithms in the future as the field of PQC continues to evolve. All PQC implementations will be subject to a rigorous security audit before being deployed on the mainnet.
