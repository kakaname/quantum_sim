use nalgebra::DMatrix;
use num::Complex;

use crate::{quantum_register::QuantumRegister, qubit::Qubit, timepoint::TimePoint};
use num_traits::{One, Zero};

pub struct QuantumCircuit {
  timepoints : Vec<TimePoint>,
  register : QuantumRegister
}

impl QuantumCircuit {
  pub fn add_timepoint(&mut self, timepoint : TimePoint) {
    self.timepoints.push(timepoint);
  }

  pub fn create_circuit(size : usize) -> Self {
    let register = QuantumRegister::new_with_size(size);
    let timepoints: Vec<TimePoint> = vec![];
    Self { timepoints, register }
  }

  pub fn create_circuit_with_qubits(qubits : Vec<Qubit>) -> Self {
    let register = QuantumRegister::new_with_qubits(qubits);
    let timepoints: Vec<TimePoint> = vec![];
    Self { timepoints, register }
  }

  pub fn run_timepoints(&mut self) {
    for timepoint in &mut self.timepoints {
      timepoint.run(&mut self.register);
    }
  }

  pub fn get_state(&self) -> &DMatrix<Complex<f32>>{
    self.register.get_state_vector()
  }

}

#[cfg(test)]
mod test_circuit {
  use nalgebra::ComplexField;
use num::complex::ComplexFloat;

use crate::quantum_gate::XGate;

  use super::*;

  #[test]
  fn test_XGate_circuit() {
    let qubits = vec![Qubit::basis_1(); 4];
    let mut circuit = QuantumCircuit::create_circuit_with_qubits(qubits); // state |1111>

    let mut gates : Vec<XGate> = vec![];
    for i in 0..4 {
      gates.push(XGate::x(i));
    }

    let mut timepoint = TimePoint::new();
    for gate in gates {
      timepoint.add_gate(Box::new(gate));
    }

    circuit.add_timepoint(timepoint);

    circuit.run_timepoints();

    let mut true_vector: Vec<Complex<f32>> = vec![Complex::zero(); 16];
    true_vector[0] = Complex::one(); // now should be in state |0000>

    assert_eq!(*circuit.get_state(), DMatrix::from_vec(16,1,true_vector));
    
    circuit.run_timepoints();

    let mut true_vector : Vec<Complex<f32>> = vec![Complex::zero(); 16];
    true_vector[15] = Complex::one(); // now should be back to state |1111>

    assert_eq!(*circuit.get_state(), DMatrix::from_vec(16,1,true_vector));

    // more than one timepoint test
    // here double hadamaard == an indentity gate

    let qubits = vec![Qubit::basis_1(); 4];
    let mut circuit = QuantumCircuit::create_circuit_with_qubits(qubits); // state |1111>

    let mut gates : Vec<XGate> = vec![];
    for i in 0..4 {
      gates.push(XGate::h(i)); 
    }

    let mut timepoint = TimePoint::new();
    let mut timepoint2 = TimePoint::new();
    for gate in gates {
      timepoint.add_gate(Box::new(gate.clone()));
      timepoint2.add_gate(Box::new(gate));
    }

    circuit.add_timepoint(timepoint);
    circuit.add_timepoint(timepoint2);

    circuit.run_timepoints();

    let mut true_vector: Vec<Complex<f32>> = vec![Complex::zero(); 16];
    true_vector[15] = Complex::one(); // now should be in state |1111>

    assert!((circuit.get_state() - DMatrix::from_vec(16,1,true_vector)).sum().re().abs() < 0.0001 );

  }

}

