extern crate rand;

use rand::Rng;
use std::vec::Vec;

const DIM: usize = 32; // Lattice dimension
const MODULUS: u128 = 97; // Prime modulus for the lattice operations

// Generate a random lattice matrix with given dimensions and modulus
fn generate_lattice_matrix() -> Vec<Vec<u128>> {
    let mut rng = rand::thread_rng();
    let mut matrix: Vec<Vec<u128>> = Vec::new();
    
    for _ in 0..DIM {
        let row: Vec<u128> = (0..DIM).map(|_| rng.gen_range(0..MODULUS)).collect();
        matrix.push(row);
    }
    
    matrix
}

// Generate random 32-byte key-value pair
fn generate_random_key_value() -> (Vec<u8>, Vec<u8>) {
    let mut rng = rand::thread_rng();
    let key: Vec<u8> = (0..32).map(|_| rng.gen()).collect();
    let value: Vec<u8> = (0..32).map(|_| rng.gen()).collect();
    
    (key, value)
}

// Encode message (key or value) into a lattice vector
fn encode_to_lattice_vector(data: Vec<u8>) -> Vec<u128> {
    data.iter().map(|&x| x as u128).collect()
}

// Perform lattice commitment: matrix * vector + randomness
fn lattice_commit(matrix: &Vec<Vec<u128>>, message: Vec<u128>, randomness: Vec<u128>) -> Vec<u128> {
    let mut commitment = vec![0u128; DIM];
    
    for i in 0..DIM {
        let mut sum = 0u128;
        for j in 0..DIM {
            sum = (sum + (matrix[i][j] * message[j])) % MODULUS;
        }
        commitment[i] = (sum + randomness[i]) % MODULUS;
    }
    
    commitment
}

// Verify the commitment by recalculating the commitment with the original message and randomness
fn verify_commit(matrix: &Vec<Vec<u128>>, message: Vec<u128>, randomness: Vec<u128>, commitment: Vec<u128>) -> bool {
    let recalculated_commitment = lattice_commit(matrix, message, randomness);
    recalculated_commitment == commitment
}

fn main() {
    // Step 1: Generate random lattice matrix
    let lattice_matrix = generate_lattice_matrix();
    
    // Step 2: Generate random key-value pair (32 bytes each)
    let (key, value) = generate_random_key_value();
    println!("Generated 32-byte Key: {:?}", key);
    println!("Generated 32-byte Value: {:?}", value);
    
    // Step 3: Encode key and value into lattice vectors
    let encoded_key = encode_to_lattice_vector(key.clone());
    let encoded_value = encode_to_lattice_vector(value.clone());
    
    // Step 4: Generate random vector for commitment (for security)
    let mut rng = rand::thread_rng();
    let randomness: Vec<u128> = (0..DIM).map(|_| rng.gen_range(0..MODULUS)).collect();
    
    // Step 5: Commit to the key
    let key_commitment = lattice_commit(&lattice_matrix, encoded_key.clone(), randomness.clone());
    println!("Key Commitment: {:?}", key_commitment);
    
    // Step 6: Verify the commitment
    let is_valid = verify_commit(&lattice_matrix, encoded_key, randomness.clone(), key_commitment.clone());
    println!("Commitment verification: {}", is_valid);
    
    // Similarly, you can create a commitment for the value
}
