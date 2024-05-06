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
    let mut state_vector : Vec<Complex<f32>> = vec![Complex::new(0., 0.); 1 << num_qubits];

    for (i, qubit_state) in vec.iter().enumerate() {
      let shift = num_qubits - i - 1;
      for(j, state) in state_vector.iter_mut().enumerate() {
        let mask = 1 << shift;
        if (j & mask) == 0 {
          state_vector[j] += qubit_state.get_state()[0] * *state;
        } else {
          state_vector[j] += qubit_state.get_state()[1] * *state;
        }
      }

    }
  
    state_vector
  }



}



