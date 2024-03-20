use std::{ops::Mul};
use nalgebra::{Complex, UnitVector2, Vector2};
use num_traits::{One, Zero};

use crate::{matrix::SquareMatrix, qubit::Qubit};


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
    // made for testing single bits and gates
    pub fn apply_bit(&self, qubit : Qubit) -> Qubit {
        let state = qubit.get_state();
        Qubit::new(
            UnitVector2::new_normalize(
                Vector2::new(
                    self.matrix.get(0,0) * state.x + self.matrix.get(0,1) * state.y,
                    self.matrix.get(1,0) * state.x + self.matrix.get(1,1) * state.y
                )
            )
        )

    }


}



#[cfg(test)]
mod test_quantum_gate {
    use crate::qubit::Qubit;

    use super::*;

    #[test]
    fn test_identity_gate(){
        let basis_0 = Qubit::basis_0();
        let identity_gate = QuantumGate::identity_gate();
        
        assert_eq!(Qubit::basis_0(), identity_gate.apply_bit(basis_0));

        let basis_1 = Qubit::basis_1();
        let identity_gate = QuantumGate::identity_gate();
        
        assert_eq!(Qubit::basis_1(), identity_gate.apply_bit(basis_1));

    }

    #[test]
    fn test_not_gate(){

    }
}