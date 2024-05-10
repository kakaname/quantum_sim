use nalgebra::{dmatrix, Complex, DMatrix, DVector, Unit, UnitVector2};

use crate::qubit::Qubit;
use crate::quantum_gate::*;
use crate::math::Math;
use num_traits::{One, Zero};



#[derive(Clone, PartialEq)]
pub struct QuantumRegister {
  register : DMatrix<Complex<f32>>
}

impl QuantumRegister {
  // initalizes a quantum register of size vec_size that has state vector == 0
  pub fn new_with_size(vec_size : usize) -> Self {
    let mut register = Self::create_state_vector_qubits(vec![Qubit::basis_0(); vec_size]);
    Self { register }
  }

  pub fn new_with_qubits(vec : Vec<Qubit>) -> Self {
    let mut register = Self::create_state_vector_qubits(vec);
    Self { register }
  }
  // creates a state vector for qubits such that 
  // [0] == |00...00>
  // [2^n] == |11..11> , with n being the size of the register
  pub fn create_state_vector_qubits(vec : Vec<Qubit>) -> DMatrix<Complex<f32>> {
    let num_qubits = vec.len();  
    let states: Vec<UnitVector2<Complex<f32>>> = vec.iter().map(|num| num.get_state()).collect();
    let mut result: DMatrix<Complex<f32>> = dmatrix![Complex::one()];
    for vector in states.iter() {
      result = result.kronecker(vector);
    }
    result
  }

  pub fn get_size(&self) -> usize {
    self.register.len().ilog2() as usize
  }

  pub fn print_register(&self) {

  }

  pub fn set_register(&mut self, register : DMatrix<Complex<f32>>) {
    self.register = register;
  }

  pub fn get_state_vector(&self) -> &DMatrix<Complex<f32>> {
    &self.register
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
    let true_state = dmatrix![Complex::zero(), Complex::zero(), Complex::one(), Complex::zero()];

    assert_eq!(true_state, state_vector.transpose());
    

    // state |0000>
    let mut vec = vec![Qubit::basis_0(); 4];
    let state_vector = QuantumRegister::create_state_vector_qubits(vec);
    let mut true_vector : Vec<Complex<f32>> = vec![Complex::zero(); 16];
    true_vector[0] = Complex::one();
    assert_eq!(state_vector, DMatrix::from_vec(16, 1, true_vector));

    // state |0001>
    let mut vec = vec![Qubit::basis_0(); 4];
    vec[3] = Qubit::basis_1();
    let state_vector = QuantumRegister::create_state_vector_qubits(vec);
    let mut true_vector : Vec<Complex<f32>> = vec![Complex::zero(); 16];
    true_vector[1] = Complex::one();
    assert_eq!(state_vector, DMatrix::from_vec(16, 1, true_vector));

    // state |1000>
    let mut vec = vec![Qubit::basis_0(); 4];
    vec[0] = Qubit::basis_1();
    let state_vector = QuantumRegister::create_state_vector_qubits(vec);
    let mut true_vector : Vec<Complex<f32>> = vec![Complex::zero(); 16];
    true_vector[8] = Complex::one();
    assert_eq!(state_vector, DMatrix::from_vec(16, 1, true_vector));

    // state |1111>
    let mut vec = vec![Qubit::basis_1(); 4];
    let state_vector = QuantumRegister::create_state_vector_qubits(vec);
    let mut true_vector : Vec<Complex<f32>> = vec![Complex::zero(); 16];
    true_vector[15] = Complex::one();
    assert_eq!(state_vector, DMatrix::from_vec(16, 1, true_vector));

  }


}


