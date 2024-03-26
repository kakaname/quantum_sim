
use nalgebra::{Complex};
use num_traits::{Zero};
use crate::{quantum_circuitry::QuantumCicuitry, qubit::Qubit};


#[derive(Clone, Debug)]
pub struct QuantumRegister {
    size : usize,
    vec : Vec<Qubit>,
    table : Vec<Complex<f32>>,
    table_valid : bool
}


impl QuantumRegister {
    pub fn new(size : usize) -> Self {
        let base : i32 = 2;
        let base = base.pow(2) as usize;

        let mut vec = vec![];
        let table = vec![Complex::zero(); base];
        for _ in 0..size {
            vec.push(Qubit::basis_0());
        }
        let table_valid = false; 

        QuantumRegister{
            size,
            vec,
            table,
            table_valid,

        }
    }

    pub fn create_valid_table(&mut self) {
        self.table = QuantumCicuitry::measurer(&self.vec);
        self.table_valid = true;


    }    


    pub fn get_size(&self) -> usize {
        self.size
    } 

}

#[cfg(test)]
mod test_quantum_register {
    use super::*;

    #[test]
    fn test_quantum_register_init() {
        let quantum_register = QuantumRegister::new(4);
        assert_eq!(quantum_register.get_size(), 4);
        let mut counter = 0;
        for qubit in quantum_register.vec.iter() {
            counter += 1;
            assert_eq!(*qubit, Qubit::basis_0());
        }
        assert_eq!(counter, 4);
    }

    

}


