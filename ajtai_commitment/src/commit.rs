extern crate nalgebra as na;
use na::{DMatrix, DVector};
use rand::Rng;
// Define matrix and vector types
type Matrix = Vec<Vec<i64>>;
type Vector = Vec<i64>;

// Generate Gadget matrix G
pub fn gadget_matrix(size: usize) -> DMatrix<i64> {
    DMatrix::from_fn(size, size, |i, j| if i == j { 1 } else { 0 })
    // let mut g_matrix = vec![vec![0; log_q]; n];
    // for i in 0..n {
    //     for j in 0..log_q {
    //         g_matrix[i][j] = 1 << j; // 2^j
    //     }
    // }
    // g_matrix
}

// Inverse of Gadget matrix G (this is just the identity matrix in our simplified example)
pub fn gadget_matrix_inverse(g: &DMatrix<i64>) -> DMatrix<i64> {
    // g.try_inverse().expect("Gadget matrix is not invertible!")
    g.clone()
}
// Generate random matrix A
pub fn generate_random_matrix(size: usize) -> Matrix {
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
pub fn generate_random_vector(size: usize) -> Vector {
    let mut rng = rand::thread_rng();
    let mut vector = vec![0; size];
    for i in 0..size {
        vector[i] = rng.gen_range(0..10); // Random numbers for vector elements
    }
    vector
}

// Matrix-vector multiplication (I_k ⊗ A)
pub fn matrix_vector_multiply(matrix: &DMatrix<i64>, vector: &DVector<i64>) -> DVector<i64> {
    matrix * vector
}

// Open the commitment by reducing the size of the vector
pub fn open_commitment_recursive(
    f_l: DVector<i64>,   // The initial message vector f_l
    g: &DMatrix<i64>,    // Gadget matrix G
    a: &DMatrix<i64>,    // Matrix A
    levels: usize        // Number of levels to reduce
) -> Vec<DVector<i64>> {
    let mut openings = Vec::new();
    let mut current_f = f_l;

    for level in (1..=levels).rev() {
        // Compute s_(l-1) = G^-1(f_l)
        let g_inv = gadget_matrix_inverse(&g);
        let s = &g_inv * &current_f;
        openings.push(s.clone());

        // Compute the next f_(l-1) = (I_k ⊗ A) * s_(l-1)
        current_f = matrix_vector_multiply(&a, &s);
    }

    openings
}

// Reverse the process to verify the opening by recreating the commitment
pub fn recreate_commitment(
    opening_vectors: Vec<DVector<i64>>, // The short opening vectors s_1, s_2, ..., s_(l-1)
    g: &DMatrix<i64>,                   // Gadget matrix G
    a: &DMatrix<i64>                    // Matrix A
) -> DVector<i64> {
    let mut f_1 = opening_vectors[0].clone();

    for s in opening_vectors.into_iter().skip(1) {
        // Compute f_(i+1) = (I_k ⊗ A) * s_i
        f_1 = matrix_vector_multiply(&a, &s);
    }

    // Finally compute f_l using G * s_1
    let f_l = g * f_1;
    f_l
}

fn main() {
    // Parameters
    let levels = 3;
    let size = 3; // Size of matrix and vector

    // Example input f_l (initial message vector)
    let f_l = DVector::from_vec(vec![10, 20, 30]);

    // Gadget matrix G (Identity for simplification)
    let g = gadget_matrix(size);

    // Matrix A (random example matrix)
    let a = DMatrix::from_vec(size, size, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);

    // Step 1: Open the commitment by computing the short vectors
    let opening_vectors = open_commitment_recursive(f_l.clone(), &g, &a, levels);

    // Print the opening vectors
    println!("Opening Vectors:");
    for (i, s) in opening_vectors.iter().enumerate() {
        println!("s_{}: {:?}", i + 1, s);
    }

    // Step 2: Recreate the commitment by reversing the process
    let recreated_f_l = recreate_commitment(opening_vectors, &g, &a);

    // Print the recreated commitment
    println!("\nRecreated f_l: {:?}", recreated_f_l);

    // Check if the recreated f_l matches the original f_l
    println!("Is the recreated f_l correct? {}", recreated_f_l == f_l);
}
