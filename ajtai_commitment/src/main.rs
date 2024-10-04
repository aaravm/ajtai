extern crate rand;
use rand::Rng;

const VECTOR_SIZE: usize = 4; // Size of the secret vector
const FOLDING_FACTOR: usize = 2; // Kappa (κ), the folding factor

// Define matrix and vector types
type Matrix = Vec<Vec<i64>>;
type Vector = Vec<i64>;

// Generate random matrix A
fn generate_random_matrix(size: usize) -> Matrix {
    let mut rng = rand::thread_rng();
    let mut matrix = vec![vec![0; size]; size];
    for i in 0..size {
        for j in 0..size {
            matrix[i][j] = rng.gen_range(0..10); // Random numbers for matrix elements
        }
    }
    matrix
}

// Generate random secret vector s
fn generate_random_vector(size: usize) -> Vector {
    let mut rng = rand::thread_rng();
    let mut vector = vec![0; size];
    for i in 0..size {
        vector[i] = rng.gen_range(0..10); // Random numbers for vector elements
    }
    vector
}

// Helper function for matrix-vector multiplication (A * s)
fn matrix_vector_multiply(matrix: &Matrix, vector: &Vector) -> Vector {
    let size = vector.len();
    let mut result = vec![0; size];
    for i in 0..size {
        for j in 0..size {
            result[i] += matrix[i][j] * vector[j];
        }
    }
    result
}

// Folding step: Reduce polynomial size by factor of kappa
fn fold_vector(vector: &Vector, kappa: usize) -> Vector {
    let folded_size = vector.len() / kappa;
    let mut folded_vector = vec![0; folded_size];
    for i in 0..folded_size {
        for j in 0..kappa {
            folded_vector[i] += vector[i * kappa + j]; // Simple folding operation
        }
    }
    folded_vector
}

// Commit to a message using matrix A and secret vector s
fn commit(matrix: &Matrix, secret_vector: &Vector, message: &Vector, generator: &Vector) -> Vector {
    let mut result = matrix_vector_multiply(&matrix, &secret_vector);
    for i in 0..message.len() {
        result[i] += message[i] * generator[i]; // A * s + m * g
    }
    result
}

// Opening the commitment: Reveal short opening proof
fn open_commitment(secret_vector: &Vector, folded_vector: &Vector) -> Vector {
    // In the opening phase, we reveal only the folded vector instead of the full secret
    folded_vector.clone()
}

// Verify the commitment with short opening proof
fn verify(matrix: &Matrix, short_opening_proof: &Vector, commitment: &Vector, message: &Vector, generator: &Vector) -> bool {
    // Recompute the expected commitment with the short opening proof
    let expected_commitment = matrix_vector_multiply(&matrix, &short_opening_proof);
    let mut verification_result = expected_commitment.clone();
    for i in 0..message.len() {
        verification_result[i] += message[i] * generator[i]; // A * s + m * g
    }
    verification_result == *commitment
}

fn main() {
    // Setup phase: Generate matrix A and secret vector s
    let matrix_a = generate_random_matrix(VECTOR_SIZE);
    let secret_vector = generate_random_vector(VECTOR_SIZE);

    // Generate a large random message and a generator vector
    let message = vec![1, 2, 3, 4]; // Random message, replace with larger polynomial as needed
    let generator = vec![1, 1, 1, 1]; // Use a simple generator for the commitment

    // Commit to the message
    let commitment = commit(&matrix_a, &secret_vector, &message, &generator);
    println!("Commitment: {:?}", commitment);

    // Folding phase: Reduce message size by folding
    let folded_secret = fold_vector(&secret_vector, FOLDING_FACTOR);
    println!("Folded secret: {:?}", folded_secret);

    // Opening the commitment with short opening proof
    let short_opening_proof = open_commitment(&secret_vector, &folded_secret);
    println!("Short opening proof: {:?}", short_opening_proof);

    // Verify the commitment using the short opening proof
    let is_valid = verify(&matrix_a, &short_opening_proof, &commitment, &message, &generator);
    println!("Is the commitment valid? {}", is_valid);
}
