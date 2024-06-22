/// An implementation fo Freivalds algorithm 
use ark_bls12_381::Fr;
use ndarray::{Array1, Array2};
use rand::Rng;
use std::iter::Iterator;

/// Struct to generate a sequence of field element
struct Univar {
    product: Fr,
    r: Fr,
}

impl Univar {
    /// Create a new Univar iterator
    fn new(r: Fr) -> Self {
        Self {
            product: 1.into(),
            r,
        }
    }
}

impl Iterator for Univar {
    type Item = Fr;
    /// Generate the next element in the sequence
    fn next(&mut self) -> Option<Self::Item> {
        self.product *= self.r;
        Some(self.product)
    }
}

/// Generate a random field element
pub fn generate_random_fr() -> Fr {
    rand::thread_rng().gen()
}

/// Generate a vector of field elements using the Univar Iterator
pub fn generate_vector(r: Fr, n: usize) -> Array1<Fr> {
    Univar::new(r).take(n).collect()
}

/// Verify matrix multiplication using Freivald's algortihm
pub fn freivald_verify(a: &Array2<Fr>, b: &Array2<Fr>, c: &Array2<Fr>) -> bool {
    assert!(check_matrix_dimensions(a, b, c));

    let v = generate_vector(generate_random_fr(), c.ncols());
    a.dot(&b.dot(&v)) == c.dot(&v)
}

// Directly verify matrix multiplication by comparing A * B == c
pub fn direct_verify(a: &Array2<Fr>, b: &Array2<Fr>, c: &Array2<Fr>) -> bool{
    assert!(check_matrix_dimensions(a, b, c));
    a.dot(b) == c
}

/// Check if the dimensions of the matrices are valid for multiplication
pub fn check_matrix_dimensions(a: &Array2<Fr>, b: &Array2<Fr>, c: &Array2<Fr>) -> bool {
    a.nrows() == c.nrows()
        && b.ncols() == c.ncols()
        && a.ncols() == b.nrows()
}


#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;

    #[test]
    fn test_direct_verify() {
        let a = array![[1.into(), 2.into()], [3.into(), 4.into()]];
        let b = array![[5.into(), 6.into()], [7.into(), 8.into()]];
        let c = array![[19.into(), 22.into()], [43.into(), 50.into()]];

        assert!(direct_verify(&a, &b, &c));
    }

    #[test]
    fn test_freivald_verify() {
        let a = array![[1.into(), 2.into()], [3.into(), 4.into()]];
        let b = array![[5.into(), 6.into()], [7.into(), 8.into()]];
        let c = array![[19.into(), 22.into()], [43.into(), 50.into()]];

        assert!(freivald_verify(&a, &b, &c));
    }
}