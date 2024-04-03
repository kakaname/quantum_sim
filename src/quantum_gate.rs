use std::f32::consts::SQRT_2;
use std::ops::{Mul};

use nalgebra::{ DMatrix, Complex, dmatrix, Vector2, UnitVector2 };

use num::integer::sqrt;
use num_traits::{ One, Zero };
use crate::qubit::Qubit;

pub struct QuantumGate {
  matrix : DMatrix<Complex<f32>>,
}

impl Mul<Qubit> for QuantumGate {
  type Output = Qubit;
  fn mul(self, rhs : Qubit) -> Self::Output {
    let matrix = self.matrix * rhs.get_state().into_inner();
    Qubit::new( 
      UnitVector2::new_normalize(
        Vector2::new(
        matrix[0],
        matrix[1]
        )
      )
    )
  }
}

impl QuantumGate {
  pub fn hadamard() -> Self {
    // returns hadamard gate

    let matrix: DMatrix<Complex<f32>> = dmatrix![
    Complex::one() * 1. / SQRT_2, Complex::one() * 1. / SQRT_2;
    Complex::one() * 1. / SQRT_2, Complex::one() * -1. / SQRT_2
    ];
    Self {
      matrix
    }
  }
}
#[cfg(test)]
mod quantum_gate_test {
  
  use super::*;

  #[test]
  fn quantum_gate_multiply_test() {

    let vec0_hadamard = 
    UnitVector2::new_normalize(
      Vector2::new(
        Complex::one() * 1. / SQRT_2,
        Complex::one() * 1. / SQRT_2
      )
    );
    let gate = QuantumGate::hadamard();
    let qubit_0 = gate * Qubit::basis_0();

    assert_eq!(qubit_0.get_state(), vec0_hadamard);

    let vec1_hadamard = 
    UnitVector2::new_normalize(
      Vector2::new(
        Complex::one() * 1. / SQRT_2,
        Complex::one() * -1. / SQRT_2
      )
    );

    let gate = QuantumGate::hadamard();
    let qubit_1 =  gate * Qubit::basis_1();

    assert_eq!(qubit_1.get_state(), vec1_hadamard);






  }


}