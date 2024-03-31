use crate::qubit::Qubit;
use crate::quantum_gate::QuantumGate;
use crate::quantum_register::QuantumRegister;

use std::f32::consts::{TAU, SQRT_2};
use std::fmt::{Display, Debug, Formatter};
use num::integer::gcd;


use nalgebra::{Complex};

type QuantumGateWithInputs = (QuantumGate, Vec<usize>);

#[derive(Debug, Clone, PartialEq)]
pub struct QuantumCircuit {
    n_qubits : usize,
    gates : Vec<QuantumGate>,
}

impl Display for QuantumCircuit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = self.gates.iter().map(|gate| gate.to_string()).collect::<Vec<String>>().join("\n");
        write!(f, "Circuit: \n\n {}", s)
    }
}

impl QuantumCircuit {
    pub fn new(n_qubits: usize) -> Self {
        Self{n_qubits, gates: Vec::new()}
    }

    pub fn add_gate(&mut self, gate: QuantumGate, input_qubits: Vec<usize> ){
        assert!(gate.n_qubits() > 0);
        assert!(gate.n_qubits() == input_qubits.len());

        let mut sorted_qubits = input_qubits.clone();
        sorted_qubits.sort();
        assert!(sorted_qubits.iter().zip(sorted_qubits.iter().skip(1)).all(|(a, b)| a != b));
        
        let mut next = 0;
        let mut completed_qubits = input_qubits.clone();

        while completed_qubits.len() < self.n_qubits {
            if completed_qubits.contains(&next) {
                next += 1;
            }else{
                completed_qubits.push(next);
                next += 1;
            }
        }

        let input_qubits_to_first_n = QuantumGate::permutation(&completed_qubits);
        let first_n_to_input_qubits = QuantumGate::reverse_permutation(&completed_qubits);
        let gate_to_add = if input_qubits.len() == self.n_qubits {
            gate
        } else {
            let identity_gate = QuantumGate::identity(self.n_qubits - input_qubits.len());
            identity_gate.clone().tensor_product(gate)
        };

        let gate_to_add = gate_to_add.compose(&input_qubits_to_first_n);
        let gate_to_add = first_n_to_input_qubits.compose(&gate_to_add);

        self.gates.push(gate_to_add);

    }

    pub fn singleton(gate : QuantumGate) -> Self {
        let mut circuit = Self::new(gate.n_qubits());
        circuit.add_gate(gate.clone(), (0..gate.n_qubits()).collect());
        circuit
    }

    pub fn run(&self, register: impl Into<QuantumRegister>) -> QuantumRegister {
        let inner_register = register.into();
        assert_eq!(self.n_qubits, inner_register.n_qubits());
        let mut intermediate_register = inner_register.clone();
        for gate in &self.gates {
            intermediate_register = gate.apply(intermediate_register);
        }
        intermediate_register

    }

    pub fn n_qubits(&self) -> usize {
        self.n_qubits
    }

    pub fn n_gates(&self) -> usize {
        self.gates.len()
    }

    pub fn get_gates(&self) -> Vec<QuantumGate> {
        self.gates.clone()
    }

    pub fn as_gate(&self) -> QuantumGate {
        let mut gate = QuantumGate::identity(self.n_qubits);
        for g in &self.gates {
            gate.compose(g);
        }
        gate
    }

    pub fn fourier_transform(n_qubits: usize) -> Self {
        let mut circuit = Self::new(n_qubits);
        circuit.add_gate(QuantumGate::permutation(&(0..n_qubits).rev().collect()), (0..n_qubits).collect());
        for i in 0..n_qubits {
            let partial = Self::partial_fourier_transform(n_qubits, i);
            circuit.extend(&partial);
        }
        circuit
    }
    
    pub fn partial_fourier_transform(n_qubits: usize, start_id: usize) -> Self {
        let mut circuit = Self::new(n_qubits);
        circuit.add_gate(QuantumGate::hadamard(), vec![start_id]);

        let mut k = 1;
        for i in (start_id + 1)..n_qubits{
            let k_pow = 2usize.pow((k+1) as u32);
            let phase_shift_gate = QuantumGate::controlled_phase_shift(TAU / (k_pow as f32));
            circuit.add_gate(phase_shift_gate, vec![i, start_id]);
            k += 1;
        }
        circuit
    }

    pub fn extend(&mut self, circuit: &Self) {
        assert_eq!(self.n_qubits, circuit.n_qubits);
        self.gates.extend(circuit.gates.iter().map(|gate| gate.clone()));

    }

    pub fn inverse_fourier_transform(n_qubits: usize) -> Self{
        let mut circuit = Self::new(n_qubits);
        for i in 0..n_qubits {
            let partial = Self::partial_inverse_fourier_transform(n_qubits, i);
            circuit.extend(&partial);
        }
        circuit.add_gate(QuantumGate::reverse_permutation(&(0..n_qubits).rev().collect()), (0..n_qubits).collect());
        circuit
    }

    pub fn partial_inverse_fourier_transform(n_qubits: usize, start_id: usize) -> Self {
        let mut circuit = Self::new(n_qubits);

        let mut k = start_id;
        for i in (n_qubits - start_id)..n_qubits {
            let k_pow = 2usize.pow((k+1) as u32);
            let phase_shift_gate = QuantumGate::controlled_phase_shift(-TAU / (k_pow as f32));
            circuit.add_gate(phase_shift_gate, vec![(n_qubits - 1) - start_id, (n_qubits - 1) - (i - (n_qubits - start_id))]);
            k -= 1;
        }
        circuit.add_gate(QuantumGate::hadamard(), vec![n_qubits - start_id - 1]);

        circuit

    }

    pub fn variably_controlled_gate(gate : QuantumGate) -> Self {
        let mut circuit = Self::new(2*gate.n_qubits());
        for i in 0..gate.n_qubits() {
            let controlled_gate = gate.clone().tensor_product(QuantumGate::identity(1));
            assert_eq!(controlled_gate.n_qubits(), gate.n_qubits()+1);

            let mut input_qubits = vec![gate.n_qubits() + i];
            input_qubits.extend((0..gate.n_qubits()).collect::<Vec<_>>());

            assert_eq!(input_qubits.len(), controlled_gate.n_qubits());
            circuit.add_gate(controlled_gate, input_qubits);
        }
        circuit

    }

    pub fn ShorsAlgo(capital_n : usize) -> Self {
        let n_qubits = 2 * (capital_n.ilog(2) + 1) as usize;
        let control_qft = QuantumCircuit::fourier_transform(n_qubits).as_gate();

        let a = (2..capital_n).filter(|a| gcd(*a, capital_n) == 1).next().expect("should be values with gcd == 1");
        let periodic_function = QuantumCircuit::variably_controlled_gate(QuantumGate::multiplications_mod_n_extended(n_qubits, capital_n, a));

        let control_ift = QuantumCircuit::inverse_fourier_transform(n_qubits).as_gate();
        let mut circuit = QuantumCircuit::new(2*n_qubits);
        circuit.add_gate(control_qft, (0..n_qubits).collect());
        circuit.extend(&periodic_function);
        circuit.add_gate(control_ift, (0..n_qubits).collect());

        circuit

    }

}


#[cfg(test)]
mod test_quantum_circuit {
    use std::process::Output;

    use super::*;

    #[test]
    fn test_fourier_transform() {
        let x_0 = QuantumRegister::singleton(Qubit::mix(Qubit::basis_0(), Qubit::basis_1(), 1., Complex::exp(TAU * 3./4. * Complex::i())));
        let x_1 = QuantumRegister::singleton(Qubit::mix(Qubit::basis_0(), Qubit::basis_1(), 1., Complex::exp(TAU * 1./2. * Complex::i())));
    
        let expected = x_1.tensor_product(&x_0);

        assert!(
            QuantumCircuit::fourier_transform(2).run(QuantumRegister::from_int(2, 3)).almost_equals(expected.clone()),
            "\n\n{}\n\n vs \n\n {}",
            QuantumCircuit::fourier_transform(2).run(QuantumRegister::from_int(2, 3)),
            expected
        );
    }
    #[test]
    fn test_fourier_and_inverse_fourier() {
        let ft = QuantumCircuit::fourier_transform(4);
        let ift = QuantumCircuit::inverse_fourier_transform(4);

        for i in 0..15 {
            let input = QuantumRegister::from_int(4, i);
            let output = ft.run(input.clone());
            let output2 = ift.run(output);
            assert!(output2.almost_equals(input));
        }
    }
}