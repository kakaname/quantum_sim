
use nalgebra::Complex;
use num_traits::{One, Zero};
use num::integer::gcd;

use crate::{matrix::SquareMatrix, qubit};


pub struct QuantumGate{
    matrix: SquareMatrix
}

impl QuantumGate {
    pub fn new(matrix : SquareMatrix) -> Self {
        Self {
            matrix
        }

    }
    pub fn get_coeffcient(&self, i: usize, j:usize) -> Complex<f32> {
        self.matrix.get_coeffcient(i, j)
    }
    
    pub fn global_rotation(n_qubits: usize, phase: f32) -> Self {
        assert!(n_qubits > 0);
        Self {matrix : SquareMatrix::identity(2usize.pow(n_qubits as u32)).scale(Complex::exp(phase * Complex::i()))}
    }
    
    pub fn permutation(qubit_permutation: &Vec<usize>) -> Self {
        assert!(qubit_permutation.len() > 0);
        let basis_size = 2usize.pow(qubit_permutation.len() as u32);
        let mut permutation = Vec::new();
        for i in 0..basis_size{
            let mut new_i = 0; 
            for (qubit_start, qubit_end) in qubit_permutation.iter().enumerate(){
                let start_value = i & (1 << qubit_start as u32);
                let start_bit = start_value >> (qubit_start as u32);
                assert!(start_bit == 0 || start_bit ==1);
                let end_value = start_bit << qubit_end;
                new_i = new_i | end_value;
            }
            permutation.push(new_i);
        }
        Self::new(SquareMatrix::permutation(permutation))
    }
    
    pub fn reverse_permutation(reverse_qubit_permutation: &Vec<usize>) -> Self {
        let mut qubit_permutation = vec![0; reverse_qubit_permutation.len()];
        for (i,&j) in reverse_qubit_permutation.iter().enumerate() {
            qubit_permutation[j] = i;
        }
        Self::permutation(&qubit_permutation)
    }

    pub fn multiplications_mod_n(n:usize, a:usize) -> Self {
        Self::multiplications_mod_n_extended(n, n, a)
    }

    pub fn multiplications_mod_n_extended(n_qubits: usize, n:usize, a:usize) -> Self {
        assert!(n_qubits >= n);
        assert!(gcd(n,a) == 1);
        let mut permutation: Vec<usize> = (0..n).map(|i| (i * a) % n).collect();
        permutation.extend::<Vec<usize>>((n..n_qubits).map(|i| i).collect());
        Self::permutation(&permutation)
    }


    pub fn identity_gate() -> Self{
        Self::new (
            SquareMatrix::from_vector_normalize(
                2, 
                vec![
                    Complex::one(),
                    Complex::zero(),
                    Complex::zero(),
                    Complex::one(),
                ]
            )
        )

    }
    pub fn not_gate() -> Self {
        // Pauli X Gate
        Self::new (
            SquareMatrix::from_vector_normalize(
                2,
                vec![
                    Complex::zero(),
                    Complex::one(),
                    Complex::one(),
                    Complex::zero(),
                ]
            )
        )
    }


}


#[cfg(test)]
mod test_quantum_gate {
    use super::*;

    #[test]
    fn test_quantum_gate(){

    }
}