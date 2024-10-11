pub mod commit;
use crate::commit::generate_random_matrix;
use crate::commit::matrix_vector_multiply;
use crate::commit::generate_random_vector;
use crate::commit::open_commitment_recursive;

// Define matrix and vector types
type Matrix = Vec<Vec<i64>>;
type Vector = Vec<i64>;

// Define tree node type: internal node or leaf node
enum VerkleNode {
    Leaf { key: Vec<i64>, value: Vec<i64>, commitment: Vec<i64> },
    Internal { children: Vec<VerkleNode>, commitment: Vec<i64> }
}

// Function to commit to a key-value pair (for leaf nodes)
fn commit_leaf(key: Vec<i64>, value: Vec<i64>, A: Matrix) -> VerkleNode {
    let s_l = generate_random_vector(); // Short vector s
    let commitment = commit(&A, &s_l, &value, &key);
    VerkleNode::Leaf { key, value, commitment }
}

// Function to commit to internal nodes (aggregates commitments of children)
fn commit_internal(children: Vec<VerkleNode>, A: Matrix) -> VerkleNode {
    let child_commitments: Vec<Vec<i64>> = children.iter().map(|child| match child {
        VerkleNode::Leaf { commitment, .. } => commitment.clone(),
        VerkleNode::Internal { commitment, .. } => commitment.clone(),
    }).collect();

    let s_l = generate_random_vector(); // Short vector s
    let commitment = commit(&A, &s_l, &child_commitments.concat(), &vec![]); // Commit to concatenated child commitments
    VerkleNode::Internal { children, commitment }
}

// Function to open a commitment (returns the opening vectors)
fn open_commitment(node: &VerkleNode, A: &Matrix) -> Vec<i64> {
    match node {
        VerkleNode::Leaf { key, value, commitment } => {
            let s_l = solve_sis(A, commitment, value); // Short vector found by solving SIS
            s_l
        }
        VerkleNode::Internal { children, commitment } => {
            // Open child commitments recursively
            for child in children {
                open_commitment(child, A);
            }
            // Generate the opening vectors for internal node
            let s_l = solve_sis(A, commitment, &children.iter().flat_map(|child| match child {
                VerkleNode::Leaf { commitment, .. } => commitment.clone(),
                VerkleNode::Internal { commitment, .. } => commitment.clone(),
            }).collect());
            s_l
        }
    }
}

// Function to verify the commitment
fn verify_commitment(commitment: Vec<i64>, opening: Vec<i64>, A: Matrix) -> bool {
    let computed_commitment = matrix_vector_multiply(&A, &opening);
    commitment == computed_commitment
}

fn main() {
    // Parameters for Ajtai-based commitment
    let A = generate_random_matrix(); // Random matrix A for SIS
    let tree_depth = 3;

    // Example: Construct a Verkle tree
    let leaf1 = commit_leaf(vec![1, 2, 3], vec![10, 20, 30], A.clone());
    let leaf2 = commit_leaf(vec![4, 5, 6], vec![40, 50, 60], A.clone());
    let internal_node = commit_internal(vec![leaf1, leaf2], A.clone());

    // Query the tree and open commitments
    let opening = open_commitment(&internal_node, &A);
    println!("Opening vectors: {:?}", opening);

    // Verify the root commitment
    let is_valid = verify_commitment(internal_node.commitment, opening, A);
    println!("Is the commitment valid? {}", is_valid);
}
