use quantum_circuit::QuantumCircuit;
use quantum_algorithm::ShorsAlgo;

use crate::quantum_gate::QuantumGate;

mod qubit;
mod quantum_gate;
mod matrix;
mod quantum_register;
mod quantum_circuit;
mod quantum_algorithm;


fn main() {
    let mut algorithm = ShorsAlgo::new(15, 3);




}
