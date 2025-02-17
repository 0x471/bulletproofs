extern crate ark_bn254;
extern crate ark_ff;
extern crate sha2;

use ark_bn254::{Fq, G1Affine};
use ark_ff::{Field, PrimeField};
use sha2::{Sha256, Digest};

/// Generates n points on BN254 curve with unknown discrete logs
pub fn generate_vector_basis(n: usize, seed: &str) -> Vec<G1Affine> {
    let b = Fq::from(3u64); // bn254 curve parameter
    let mut vector_basis = Vec::with_capacity(n);
    let mut entropy = 0u64;
    
    // Initial x value from seed
    let mut hasher = Sha256::new();
    hasher.update(seed.as_bytes());
    let hash = hasher.finalize();
    let mut x = Fq::from_be_bytes_mod_order(&hash);
    
    while vector_basis.len() < n {
        // Try to find point on curve
        let mut found_point = false;
        while !found_point {
            // Check if x^3 + b has a square root
            let y_squared = x * x * x + b;
            if let Some(y) = y_squared.sqrt() {
                // Choose y or -y based on entropy
                let point = if entropy % 2 == 0 {
                    G1Affine::new(x, y)
                } else {
                    G1Affine::new(x, -y)
                };
                
                if point.is_on_curve() {
                    vector_basis.push(point);
                    found_point = true;
                }
            }   
            
            // Try next x value if no point found
            x += Fq::from(1);
            entropy += 1;
        }
        
        // Generate new x value for next point
        let mut hasher = Sha256::new();
        hasher.update(x.to_string().as_bytes());
        let hash = hasher.finalize();
        x = Fq::from_be_bytes_mod_order(&hash);
    }
    
    vector_basis
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_vector_basis() {
        let points = generate_vector_basis(2, "Bulletproofs!");
        assert_eq!(points.len(), 2);
        for point in points.iter() {
            assert!(point.is_on_curve());
        }
    }
}

