use crate::quantum_gate::QuantumGate;
use crate::{quantum_circuit::QuantumCircuit};
use crate::quantum_register::QuantumRegister;


#[derive(Debug)]
pub struct ShorsAlgo {
    capital_n : u32,
    x : u32
}

impl ShorsAlgo {
    pub fn new(capital_n : u32, x : u32) -> Self {
        Self { capital_n, x }

    }
    

    pub fn run(&mut self) -> u8 {
        let n_power_2 = self.capital_n * self.capital_n;
        let two_n_power_2  = 2 * n_power_2;

        let t = two_n_power_2.ilog2() as usize;

        let mut argument_register = QuantumRegister::basis(t, 0);

        let mut function_register = QuantumRegister::basis(self.capital_n.ilog2() as usize, 0);
        
        // apply Hadamard gate
        let hadamard = QuantumGate::hadamard();
        argument_register = hadamard.apply(argument_register);
        function_register = hadamard.apply(function_register);



        8
    }

}

