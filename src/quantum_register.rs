
use nalgebra::{Complex, DVector, Unit};

use crate::qubit::Qubit;
use crate::quantum_gate::QuantumGate;



#[derive(Clone, PartialEq)]
pub struct QuantumRegister {
  register : Vec<Qubit>
}

impl QuantumRegister {
  pub fn new(size : usize) -> Self {
    let mut register = vec![Qubit::basis_0(); size];

    Self { register }
  }

  pub fn apply_one(&mut self, i : usize, gate : QuantumGate) {
    self.register[i].apply(gate);
  }

  pub fn apply_two(&mut self, i : usize, j : usize, gate_function : fn(Qubit, Qubit) -> (Qubit, Qubit)) {
    let (qubit1, qubit2) = gate_function(self.register[i], self.register[j]);
    self.register[i] = qubit1;
    self.register[j] = qubit2;

  }


}



