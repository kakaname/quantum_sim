use crate::{quantum_circuit::QuantumCircuit};
use crate::quantum_register::QuantumRegister;


#[derive(Debug)]
pub struct ShorsAlgo {
    capital_n : u32,
    circuit : QuantumCircuit
}

impl ShorsAlgo {
    pub fn new(capital_n : u32) -> Self {
        Self { capital_n, circuit: QuantumCircuit::ShorsAlgo(capital_n as usize) }

    }
    
    pub fn get_circuit(&self) -> &QuantumCircuit {
        &self.circuit
    }

    pub fn run(&mut self) -> u8 {
        let n_qubits = self.circuit.n_qubits() / 2;
        let control_register = QuantumRegister::basis(n_qubits, 0);
        let target_register = QuantumRegister::basis(n_qubits, 1);

        let register = control_register.tensor_product(&target_register);

        let output = self.circuit.run(register);

        let (measurement, _) = output.measure();

        measurement
    }

}

