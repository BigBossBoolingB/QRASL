from src.qrasl.blockchain import Blockchain
from src.qrasl.intent import Intent, Solver
import json

def main():
    """
    A demonstration of the Intent-Driven DAG blockchain.
    """
    # Use a low difficulty for a quick demonstration run
    difficulty = 2
    print(f"--- Creating a new Intent-Driven DAG with difficulty {difficulty} ---\n")
    dag_chain = Blockchain(difficulty=difficulty)
    solver = Solver()

    # 1. Users submit intents to the network's pool
    print("--- Step 1: Users submit intents to the pool ---")
    intent1 = Intent(user="Alice", intent_data={'type': 'transfer', 'amount': 10, 'to': 'Bob'})
    intent2 = Intent(user="Charlie", intent_data={'type': 'transfer', 'amount': 5, 'to': 'David'})
    dag_chain.add_intent(intent1)
    dag_chain.add_intent(intent2)
    print(f"Intent pool now contains {len(dag_chain.intent_pool)} intents.\n")

    # 2. A Solver decides to create a block by processing the available intents
    print("--- Step 2: A Solver processes the pool to create the next block ---")
    new_block_1 = dag_chain.create_new_block(solver)
    print(f"Intent pool is now empty: {len(dag_chain.intent_pool) == 0}\n")

    # 3. More intents are submitted by users
    print("--- Step 3: More intents are submitted ---")
    intent3 = Intent(user="Bob", intent_data={'type': 'transfer', 'amount': 2, 'to': 'Alice'})
    intent4 = Intent(user="David", intent_data={'type': 'transfer', 'amount': 3, 'to': 'Charlie'})
    dag_chain.add_intent(intent3)
    dag_chain.add_intent(intent4)
    print(f"Intent pool now contains {len(dag_chain.intent_pool)} intents.\n")

    # 4. The Solver runs again, creating another block
    print("--- Step 4: The Solver runs again ---")
    new_block_2 = dag_chain.create_new_block(solver)

    # 5. Print the final state of the blockchain for inspection
    print("\n--- Final Blockchain State ---")
    sorted_blocks = sorted(dag_chain.blocks.values(), key=lambda b: b.index)
    for block in sorted_blocks:
        print(f"Index: {block.index}, Hash: {block.hash[:10]}..., Parents: {[p[:10] for p in block.parent_hashes]}")
        print("  Solutions in Block:")
        for sol in block.solutions:
            print(f"  - Intent: {sol.intent_hash[:10]}... | Executed TX: {json.dumps(sol.executed_tx)}")
        if not block.solutions:
            print("  - (Genesis Block)")

    # 6. Validate the integrity of the final DAG
    print("\n--- Verifying Final DAG Integrity ---")
    is_valid = dag_chain.is_chain_valid()
    print(f"Is the final DAG structure valid? -> {is_valid}")

if __name__ == "__main__":
    main()
