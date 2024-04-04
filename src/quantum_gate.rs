use std::f32::consts::{PI, SQRT_2};
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
  pub fn cnot(qubit1: Qubit, qubit2: Qubit) -> (Qubit, Qubit) {
    // takes:
    // (Control, NOT)
    // returns:
    // NOT
    let state1 = qubit1.get_state();
    let state2 = qubit2.get_state();

    let vector: UnitVector2<Complex<f32>> = UnitVector2::new_normalize(Vector2::new(
      state1.y * state2.y + state1.x * state2.x,
      state1.y * state2.x + state1.x * state2.y
    ));
    println!("vector: {}", vector.into_inner());

    (
      Qubit::new(state1),
      Qubit::new(vector)
    )
  }

  pub fn not(&self, qubit1 : Qubit) -> Qubit {
    // the not gate
    let state1 = qubit1.get_state();
    let vector: UnitVector2<Complex<f32>> = UnitVector2::new_normalize(Vector2::new(
      state1.y,
      state1.x
    ));

    Qubit::new(vector)
  }



}
#[cfg(test)]
mod quantum_gate_test {
  
  use super::*;

  #[test]
  fn test_cnot() {
    // Should not change when control is 0
    let (qubit1, qubit2) = QuantumGate::cnot(Qubit::basis_0(), Qubit::basis_1());
    assert_eq!(qubit2, Qubit::basis_1());

    // Should switch to 1 when control is 1
    let (qubit1, qubit2) = QuantumGate::cnot(Qubit::basis_1(), Qubit::basis_0());
    assert_eq!(qubit2, Qubit::basis_1());

    // Should switch to 0 when control is 1
    let (qubit1, qubit2) = QuantumGate::cnot(Qubit::basis_1(), Qubit::basis_1());
    assert_eq!(qubit2, Qubit::basis_0());

    // Should stay the same when control is 1
    let (qubit1, qubit2) = QuantumGate::cnot(Qubit::basis_1(), Qubit::half_half());
    assert_eq!(qubit2, Qubit::half_half());
  }

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