use nalgebra::{Complex, DVector, Unit};

use crate::qubit::Qubit;
use crate::quantum_gate::*;



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
    let mut state_vector : Vec<Complex<f32>> = vec![Complex::new(0., 0.); num_qubits];

    for (i, qubit) in vec.iter().enumerate() {

    }

  
    state_vector
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


