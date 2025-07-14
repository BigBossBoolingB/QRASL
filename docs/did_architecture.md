# QRASL Decentralized Identity (DID) Architecture

This document specifies the architecture for the native Decentralized Identity system on the QRASL blockchain.

## 1. QRASL DID Method

The QRASL DID method conforms to the W3C DID Core specification.

### 1.1. DID Format

A QRASL DID has the following format:

`did:qrasl:<shard-id>:<unique-identifier>`

- `did:qrasl`: The method identifier.
- `<shard-id>`: The ID of the shard where the DID is registered (typically the Governance & Data Bridge Shard).
- `<unique-identifier>`: A unique, randomly generated string that identifies the DID.

**Example:** `did:qrasl:1:2aF4...b8E1`

## 2. DID Document

The DID Document is a JSON object associated with a DID that contains cryptographic material, service endpoints, and other metadata.

### 2.1. DID Document Structure

```json
{
  "@context": "https://www.w3.org/ns/did/v1",
  "id": "did:qrasl:1:2aF4...b8E1",
  "verificationMethod": [{
    "id": "did:qrasl:1:2aF4...b8E1#keys-1",
    "type": "Ed25519VerificationKey2018",
    "controller": "did:qrasl:1:2aF4...b8E1",
    "publicKeyBase58": "..."
  }],
  "authentication": [
    "did:qrasl:1:2aF4...b8E1#keys-1"
  ],
  "assertionMethod": [
    "did:qrasl:1:2aF4...b8E1#keys-1"
  ],
  "capabilityDelegation": [
    "did:qrasl:1:2aF4...b8E1#keys-1"
  ],
  "capabilityInvocation": [
    "did:qrasl:1:2aF4...b8E1#keys-1"
  ]
}
```
