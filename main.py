from src.qrasl.blockchain import Blockchain

def main():
    """
    A demonstration of the blockchain with 'Proof of Solution'.
    """
    # Instantiate the blockchain with a difficulty.
    # A higher number makes the "problem" harder to solve.
    difficulty = 4
    print(f"--- Creating a new QRASL Blockchain with a difficulty of {difficulty} ---\n")
    qrasl_chain = Blockchain(difficulty=difficulty)
    print(f"Genesis Block created: {qrasl_chain.get_latest_block().hash}\n")
    print("-" * 20)

    # Add a few blocks to see the "Proof of Solution" in action
    print("Adding Block 1...")
    qrasl_chain.add_block(transactions={"sender": "Alice", "receiver": "Bob", "amount": 100})
    print("-" * 20)

    print("Adding Block 2...")
    qrasl_chain.add_block(transactions={"sender": "Bob", "receiver": "Charlie", "amount": 50})
    print("-" * 20)

    # Print the full blockchain to inspect its contents
    print("\n--- Full Blockchain Details ---")
    for block in qrasl_chain.chain:
        print(f"Index: {block.index}")
        print(f"Timestamp: {block.timestamp}")
        print(f"Transactions: {block.transactions}")
        print(f"Previous Hash: {block.previous_hash}")
        print(f"Solution: {block.solution}")
        print(f"Hash: {block.hash}")
        print("-------------------------")

    # Validate the blockchain's integrity
    print(f"\n--- Verifying Blockchain Integrity ---")
    is_valid = qrasl_chain.is_chain_valid()
    print(f"Is the current blockchain valid? -> {is_valid}\n")

    # Demonstrate the effect of tampering
    print("--- Tampering Demonstration ---")
    print("Tampering with Block 1's transaction data...")
    qrasl_chain.chain[1].transactions = {"sender": "Eve", "receiver": "Mallory", "amount": 9999}

    print("Re-validating the chain after tampering...")
    is_valid_after_tamper = qrasl_chain.is_chain_valid()
    print(f"Is the blockchain valid after tampering? -> {is_valid_after_tamper}")
    print("The chain is now invalid because changing the data means the stored hash is no longer correct.")

if __name__ == "__main__":
    main()
