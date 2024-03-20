use crate::{qubit::Qubit};


#[derive(Clone, Debug)]
pub struct QuantumRegister{
    size : usize,
    vec : Vec<Qubit>,

}

impl QuantumRegister {
    pub fn new(size : usize) -> Self {
        let mut vec = vec![];
        for _ in 0..size {
            vec.push(Qubit::basis_0());
        }
        QuantumRegister{
            size,
            vec,
        }
    }


}

#[cfg(test)]
mod test_quantum_register {
    use super::*;

    #[test]
    fn test_quantum_gate_init() {

    }



}


