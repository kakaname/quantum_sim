use nalgebra::{Complex, DVector, Unit, UnitVector2};

use crate::qubit::Qubit;
use crate::quantum_gate::*;
use crate::math::Math;



#[derive(Clone, PartialEq)]
pub struct QuantumRegister {
  register : Vec<Complex<f32>>
}

impl QuantumRegister {
  pub fn new(vec : Vec<Qubit>) -> Self {
    let register = Self::create_state_vector_qubits(vec);
    Self { register }

  }

  pub fn create_state_vector_qubits(vec : Vec<Qubit>) -> Vec<Complex<f32>> {
    let num_qubits = vec.len();  
    let states: Vec<UnitVector2<Complex<f32>>> = vec.iter().map(|num| num.get_state()).collect();
    for i in 0..num_qubits {
      println!("state_vector 1: {}", states[i][0]);
      println!("state_vector 2: {}", states[i][1]);
    }

    let mut result = vec![states[0].to_owned()[0] , states[0].to_owned()[1]];
    for vector in states.iter().skip(1) {
      result = Math::kronecker(&result, vector);
    }

    result
  }



}


#[cfg(test)]
mod test_register {
  use super::*;
  use num_traits::{One, Zero};

  #[test]
  fn test_create_state_vector_qubits() {
    let vec = vec![Qubit::basis_1(), Qubit::basis_0()];

    let state_vector = QuantumRegister::create_state_vector_qubits(vec);
    let true_state = vec![Complex::zero(), Complex::zero(), Complex::one(), Complex::zero()];

    assert_eq!(true_state, state_vector);


  }

}


