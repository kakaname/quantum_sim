use crate::quantum_register::QuantumRegister;
use crate::quantum_gate::{Apply, XGate, CXGate, CCXGate};


pub struct TimePoint {
  gates : Vec<Box<dyn Apply>>

}

impl TimePoint {
  pub fn run(&self, register : &mut QuantumRegister) {
    for gate in &self.gates {
      gate.apply(register);
    }
  }

  pub fn add_gate(&mut self, apply : Box<dyn Apply>) {
    self.gates.push(apply); 
  }

  pub fn new() -> Self {
    let gates: Vec<Box<dyn Apply>> = vec![];
    Self { gates }
  }
  

}



