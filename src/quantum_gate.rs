
use nalgebra::Complex;
use num_traits::{One, Zero};

use crate::matrix::SquareMatrix;


pub struct QuantumGate{
    matrix: SquareMatrix
}

impl QuantumGate {
    pub fn new(matrix : SquareMatrix) -> Self {
        Self {
            matrix
        }

    }
    pub fn identity_gate() -> Self{
        Self::new (
            SquareMatrix::from_vector_normalize(
                2, 
                vec![
                    Complex::one(),
                    Complex::zero(),
                    Complex::zero(),
                    Complex::one(),
                ]
            )
        )

    }
    pub fn not_gate() -> Self {
        // Pauli X Gate
        Self::new (
            SquareMatrix::from_vector_normalize(
                2,
                vec![
                    Complex::zero(),
                    Complex::one(),
                    Complex::one(),
                    Complex::zero(),
                ]
            )
        )


    }


}


#[cfg(test)]
mod test_quantum_gate {
    use super::*;

    #[test]
    fn test_quantum_gate(){

    }
}