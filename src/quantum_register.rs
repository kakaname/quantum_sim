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

  pub fn new(vec : Vec<Qubit>) -> Self {
    let mut register = Self::create_state_vector_qubits(vec);
    Self { register }
  }

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


  }

}


