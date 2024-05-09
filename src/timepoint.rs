use crate::quantum_register::QuantumRegister;
use crate::quantum_gate::{Apply, XGate, CXGate, CCXGate};


pub struct TimePoint {
  gates : Vec<Box<dyn Apply>>,
  register : QuantumRegister

}

impl TimePoint {
  pub fn run(&mut self) {
    for gate in &self.gates {
      gate.apply(&mut self.register);
    }
  }

  pub fn add_gate(&mut self, apply : Box<dyn Apply>) {
    self.gates.push(apply); 
  }
  


}



