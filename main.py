from src.qrasl.blockchain import Blockchain

def main():
    """
    A simple demonstration of the blockchain.
    """
    # 1. Create a new blockchain instance
    qrasl_chain = Blockchain()
    print("QRASL Blockchain instance created.")
    print(f"Genesis Block created with hash: {qrasl_chain.get_latest_block().hash}")

    # 2. Add a few blocks with sample transactions
    print("\nAdding new blocks...")
    block1 = qrasl_chain.add_block(transactions={"sender": "Alice", "receiver": "Bob", "amount": 50})
    print(f"Added Block 1 with hash: {block1.hash}")

    block2 = qrasl_chain.add_block(transactions={"sender": "Bob", "receiver": "Charlie", "amount": 25})
    print(f"Added Block 2 with hash: {block2.hash}")

    # 3. Print the entire blockchain
    print("\n--- Full Blockchain ---")
    for block in qrasl_chain.chain:
        print(f"Index: {block.index}")
        print(f"Timestamp: {block.timestamp}")
        print(f"Transactions: {block.transactions}")
        print(f"Previous Hash: {block.previous_hash}")
        print(f"Hash: {block.hash}")
        print("-------------------------")

    # 4. Validate the blockchain
    print(f"\nChecking blockchain validity...")
    is_valid = qrasl_chain.is_chain_valid()
    print(f"Is the blockchain valid? -> {is_valid}\n")

    # 5. Demonstrate tampering
    print("--- Tampering Demonstration ---")
    print("Tampering with Block 1 by changing its transactions...")
    # Directly changing the data of a past block
    qrasl_chain.chain[1].transactions = {"sender": "Eve", "receiver": "Alice", "amount": 1000}

    print("Re-validating the chain after tampering...")
    is_valid_after_tamper = qrasl_chain.is_chain_valid()
    print(f"Is the blockchain valid after tampering? -> {is_valid_after_tamper}")
    print("The chain is invalid because the hash of Block 1 no longer matches its content, breaking the link to Block 2.")


if __name__ == "__main__":
    main()
