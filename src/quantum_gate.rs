use std::{f32::consts::SQRT_2};
use std::fmt::{Formatter, Debug, Display};

use nalgebra::{Complex};
use num_traits::{One, Zero};
use num::integer::gcd;


use crate::{matrix::SquareMatrix, quantum_register::QuantumRegister, qubit::Qubit};


#[derive(Clone, PartialEq)]
pub struct QuantumGate{
    matrix: SquareMatrix
}

impl Debug for QuantumGate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result{
        write!(f, "{:?}", self.matrix)
    }
}
impl Display for QuantumGate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.matrix)
    }
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

    pub fn tensor_product(&self, rhs : Self) -> Self {
        Self::new(self.matrix.tensor_product(&rhs.matrix))
    }

    pub fn compose(&self, rhs: &Self) -> Self {
        Self::new(self.matrix.clone() * rhs.matrix.clone())
    }

    pub fn cnot()-> Self {
        Self::new(SquareMatrix::from_vec_normalize(4, 
            vec![
                Complex::one(), // 00
                Complex::zero(),
                Complex::zero(),
                Complex::zero(),
                Complex::zero(), // 10
                Complex::one(), // 11
                Complex::zero(),
                Complex::zero(),
                Complex::zero(), // 20
                Complex::zero(),
                Complex::zero(),
                Complex::one(), // 23
                Complex::zero(), // 30
                Complex::zero(),
                Complex::one(), // 32
                Complex::zero(),
            ]))
    }

    pub fn controlled_phase_shift(phase : f32) -> Self {
        Self::new(SquareMatrix::from_vec_normalize(4, vec![
            Complex::one(), // 00
            Complex::zero(),
            Complex::zero(),
            Complex::zero(),
            Complex::zero(), // 10
            Complex::one(), // 11
            Complex::zero(),
            Complex::zero(),
            Complex::zero(), // 20
            Complex::zero(),
            Complex::one(), // 22
            Complex::zero(),
            Complex::zero(), // 30
            Complex::zero(),
            Complex::zero(),
            Complex::exp(phase * Complex::i()), // 33
        ]))
    }

    pub fn hadamard() -> Self {
        Self::new(
            SquareMatrix::from_vec_normalize(2, vec![
                Complex::one() * 1./SQRT_2, Complex::one() * 1./SQRT_2,
                Complex::one() * 1./SQRT_2, Complex::one() * -1./SQRT_2,
            ])
        )
    }

    pub fn reverse(&self) -> Self {
        Self::new(self.matrix.clone().invert())
    }

    pub fn n_qubits(&self) -> usize {
        self.matrix.size().ilog2() as usize
    }

    pub fn identity(n_qubits: usize) -> Self{
        assert!(n_qubits > 0);
        Self::new(SquareMatrix::one(2usize.pow(n_qubits as u32)))
    }
    pub fn not() -> Self {
        // Pauli X Gate
        Self::new (
            SquareMatrix::from_vec_normalize(
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

    pub fn apply(&self, register: impl Into<QuantumRegister>) -> QuantumRegister {
        QuantumRegister::new_normalize(self.matrix.clone() * register.into().get_vector())
    }

    pub fn almost_equals(&self, rhs: &Self) -> bool {
        self.matrix.almost_equals(&rhs.matrix)
    }


}


#[cfg(test)]
mod test_quantum_gate {

    use std::iter;
    use nalgebra::Vector2;
    use super::*;

    #[test]
    fn test_not_gate(){

        assert!(
            QuantumGate::not().apply(Qubit::basis_0()).almost_equals(Qubit::basis_1())
        );
        assert!(
            QuantumGate::not().apply(Qubit::basis_1()).almost_equals(Qubit::basis_0())
        );
        assert!(
            QuantumGate::not().apply(Qubit::mix(Qubit::basis_0(), Qubit::basis_1(), 1., 3.)).almost_equals(
                Qubit::mix(Qubit::basis_1(), Qubit::basis_0(), 1., 3.)
            )
        );
    }

    #[test]
    fn test_cnot_gate() {
        let cnot = QuantumGate::cnot();

        let zero: Complex<f32> = Complex::zero();
        let one = Complex::one();

        assert_eq!(cnot.matrix.get(0,0), one.clone());
        assert_eq!(cnot.matrix.get(1,1), one.clone());
        assert_eq!(cnot.matrix.get(2,3), one.clone());
        assert_eq!(cnot.matrix.get(3,2), one.clone());

        let zero_zero = QuantumRegister::basis(2, 0);
        let zero_one = QuantumRegister::basis(2, 1);
        let one_zero = QuantumRegister::basis(2, 2);
        let one_one = QuantumRegister::basis(2, 3);

        assert_eq!(cnot.clone().apply(zero_zero.clone()), zero_zero.clone());
        assert_eq!(cnot.clone().apply(zero_one.clone()), zero_one.clone());
        assert_eq!(cnot.clone().apply(one_zero.clone()), one_one.clone());
        assert_eq!(cnot.clone().apply(one_one.clone()), one_zero.clone());

    }


}