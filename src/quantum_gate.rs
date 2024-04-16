use std::f32::consts::{PI, SQRT_2};
use std::ops::{Mul};
use std::vec;

use nalgebra::{ DMatrix, Complex, dmatrix, Vector2, UnitVector2 };

use num::integer::sqrt;
use num_traits::{ float, One, Zero };
use std::f64::consts::E;
use crate::qubit::{self, Qubit};

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
  pub fn hadamard(qubit1: &mut Qubit ) {
    let factor = 1.0/SQRT_2;
    let state1 = qubit1.get_state();
    let vector: UnitVector2<Complex<f32>> = UnitVector2::new_normalize(Vector2::new(
      (state1.x + state1.y) * factor,
      (state1.x - state1.y) * factor
    ));

    qubit1.change_state(vector);
  }

  pub fn not(qubit1: &mut Qubit) {
    // the not gate
    let state1 = qubit1.get_state();
    let vector: UnitVector2<Complex<f32>> = UnitVector2::new_normalize(Vector2::new(
      state1.y,
      state1.x
    ));

    qubit1.change_state(vector);
  }


  pub fn swap(qubit1: &mut Qubit, qubit2: &mut Qubit) {
    let state1 = qubit1.get_state();
    qubit1.change_state(qubit2.get_state());
    qubit2.change_state(state1);

  }

  // coeffcient values and corresponding gates:
  // 
  pub fn phase(qubit1: &mut Qubit, coeffcient: f32){
    // gate :
    // 1  0 
    // 0  e^(i*pi/coeffcient)
    let turn_coeffcient = Complex::from(E as f32).powc(Complex::i()*(PI/coeffcient));
    let state1 = qubit1.get_state();
    let vector: UnitVector2<Complex<f32>> = UnitVector2::new_normalize(Vector2::new(
      state1.x, 
      state1.y * turn_coeffcient 
    ));

    qubit1.change_state(vector);
  }

  pub fn pauli_z(qubit1: &mut Qubit) {
    let state1 = qubit1.get_state();
    let vector: UnitVector2<Complex<f32>> = UnitVector2::new_normalize(Vector2::new(
      state1.x, 
      -state1.y
    ));
    qubit1.change_state(vector);
  }

  pub fn pauli_y(qubit1: &mut Qubit) {
    let state1 = qubit1.get_state();
    let vector: UnitVector2<Complex<f32>> = UnitVector2::new_normalize(Vector2::new(
      state1.y * -Complex::i(), 
      state1.x * Complex::i()
    ));
    qubit1.change_state(vector);
  }

  // qubit1 : control
  // qubit2 : NOT
  pub fn cnot(qubit1: &mut Qubit, qubit2: &mut Qubit) {
    let state1 = qubit1.get_state();
    let state2 = qubit2.get_state();
    let vector: UnitVector2<Complex<f32>> = UnitVector2::new_normalize(Vector2::new(
      state1.x * state2.x + state1.y * state2.y,
      state1.x * state2.y + state1.y * state2.x
    ));

    qubit2.change_state(vector);
  }


}
#[cfg(test)]
mod quantum_gate_test {
  
  use super::*;

  #[test]
  fn test_cnot() {
    // |11> -> |10>
    let mut qubit1 = Qubit::basis_1();
    let mut qubit2 = Qubit::basis_0();

    QuantumGate::cnot(&mut qubit1, &mut qubit2);
    assert_eq!(qubit2, Qubit::basis_1());

    // |11> -> |10>
    qubit1 = Qubit::basis_1();
    qubit2 = Qubit::basis_1();

    QuantumGate::cnot(&mut qubit1, &mut qubit2);
    assert_eq!(qubit2, Qubit::basis_0());

    // |01> -> |01>
    qubit1 = Qubit::basis_0();
    qubit2 = Qubit::basis_1();

    QuantumGate::cnot(&mut qubit1, &mut qubit2);
    assert_eq!(qubit2, Qubit::basis_1());

    // |1[1/2, 1/2]> -> |1[1/2,1/2]>
    qubit1 = Qubit::basis_1();
    qubit2 = Qubit::half_half();

    QuantumGate::cnot(&mut qubit1, &mut qubit2);
    assert_eq!(qubit2, Qubit::half_half());
  }

  #[test]
  fn test_not() {
    // |1> -> |0>
    let mut qubit1 = Qubit::basis_0();
    QuantumGate::not(&mut qubit1);
    assert_eq!(qubit1, Qubit::basis_1());

    // |0> -> |1>
    qubit1 = Qubit::basis_1();
    QuantumGate::not(&mut qubit1);
    assert_eq!(qubit1, Qubit::basis_0());
  }

  #[test]
  fn test_pauli_z() {
    let mut qubit1 = Qubit::basis_0();
    QuantumGate::pauli_z(&mut qubit1);
    assert_eq!(qubit1.get_state().x, Complex::one());
    assert_eq!(qubit1.get_state().y, Complex::zero());

    let mut qubit1 = Qubit::basis_1();
    QuantumGate::pauli_z(&mut qubit1);
    assert_eq!(qubit1.get_state().x, Complex::zero());
    assert_eq!(qubit1.get_state().y, -Complex::one());
  }

  #[test]
  fn test_pauli_y() {
    let mut qubit1 = Qubit::basis_0();
    QuantumGate::pauli_y(&mut qubit1);
    assert_eq!(qubit1.get_state().x, Complex::zero());
    assert_eq!(qubit1.get_state().y, Complex::i());

    let mut qubit1 = Qubit::basis_1();
    QuantumGate::pauli_y(&mut qubit1);
    assert_eq!(qubit1.get_state().x, -Complex::i());
    assert_eq!(qubit1.get_state().y, Complex::zero());

  }

  #[test]
  fn quantum_gate_phase_test() {
    // state:
    // sqrt(0.5)  
    // sqrt(0.5)
    let qubit1 = Qubit::half_half();

    // halfturn

  }


}