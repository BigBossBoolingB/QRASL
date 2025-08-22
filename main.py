from src.qrasl.blockchain import Blockchain

def print_graph_state(blockchain):
    """A helper function to print the current state of the DAG."""
    print("\n--- Blockchain Graph State ---")
    # Sort blocks by index for a more readable output
    sorted_blocks = sorted(blockchain.blocks.values(), key=lambda b: b.index)
    for block in sorted_blocks:
        parent_hashes_short = [p[:10] for p in block.parent_hashes]
        print(f"Index: {block.index}, Hash: {block.hash[:10]}..., Parents: {parent_hashes_short}")

    tips = blockchain.get_tips()
    tip_hashes_short = [tip.hash[:10] for tip in tips]
    print(f"Tips: {tip_hashes_short}")
    print("----------------------------")

def main():
    """
    A demonstration of the Directed Acyclic Graph (DAG) blockchain.
    """
    # Using a low difficulty for a quick demonstration
    difficulty = 3
    print(f"--- Creating a new DAG Blockchain with difficulty {difficulty} ---\n")
    dag_chain = Blockchain(difficulty=difficulty)

    # The genesis block is created on initialization.
    genesis_block = dag_chain.get_tips()[0]
    print(f"Genesis Block created.")
    print_graph_state(dag_chain)

    # 1. Create two blocks that both point to the genesis block. This creates a fork.
    print("\nStep 1: Creating two parallel blocks (A and B) from Genesis...")
    block_A = dag_chain.add_block(transactions="Transaction A", parent_hashes=[genesis_block.hash])
    block_B = dag_chain.add_block(transactions="Transaction B", parent_hashes=[genesis_block.hash])
    print_graph_state(dag_chain)

    # 2. Create a third block that references only Block A.
    print("\nStep 2: Building another block (C) on top of Block A...")
    block_C = dag_chain.add_block(transactions="Transaction C", parent_hashes=[block_A.hash])
    print_graph_state(dag_chain)

    # 3. Create a "merge" block that references both Block B and Block C as parents.
    print("\nStep 3: Creating a merge block (D) from B and C...")
    block_D = dag_chain.add_block(
        transactions="Transaction D (merged)",
        parent_hashes=[block_B.hash, block_C.hash]
    )
    print_graph_state(dag_chain)

    # 4. Finally, validate the integrity of the entire DAG.
    print("\n--- Verifying Final DAG Integrity ---")
    is_valid = dag_chain.is_chain_valid()
    print(f"Is the final DAG structure valid? -> {is_valid}")

if __name__ == "__main__":
    main()
