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
    for i in 0..100 {
        let mut algorithm = ShorsAlgo::new(6);
        let circuit = algorithm.get_circuit();
        assert_eq!(circuit.n_qubits(), 12);
        assert_eq!(circuit.n_gates(), 8);

        let gates = circuit.get_gates();
        assert!(gates[0].almost_equals(&QuantumGate::identity(6).tensor_product(QuantumCircuit::fourier_transform(6).as_gate())));

        assert!(gates[circuit.n_gates() - 1].almost_equals(&QuantumGate::identity(6).tensor_product(QuantumCircuit::inverse_fourier_transform(6).as_gate())));

        let result = algorithm.run();

        println!("loop : {} Result : {} ", i, result);

    }



}
