use std::{fmt::{Debug, Display ,Formatter}};

use nalgebra::{Complex, DVector, Unit, dvector};
use num_traits::One;

use rand::Rng;

use crate::qubit::{Measurement, Qubit};



#[derive(Clone, PartialEq)]
pub struct QuantumRegister {
    register : Unit<DVector<Complex<f32>>>,
}

impl Debug for QuantumRegister {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.register)
    }
}

impl Display for QuantumRegister {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = self.register.iter().map(|x| format!("{:?}", x)).collect::<Vec<String>>().join("\n");
        write!(f, "{}", s)
    }
}

impl From<Qubit> for QuantumRegister {
    fn from(qubit: Qubit) -> Self {
        Self::singleton(qubit)
    }
}

impl QuantumRegister {

    pub fn new(register: Unit<DVector<Complex<f32>>>) -> Self {
        Self{ register }
    }

    pub fn new_normalize(register : DVector<Complex<f32>>) -> Self {
        Self::new(Unit::<DVector<Complex<f32>>>::new_normalize(register))
    }

    pub fn from_vec_normalize(vec : Vec<Complex<f32>>) -> Self {
        Self::new_normalize(DVector::from_vec(vec))
    }

    pub fn from_int(n_qubits: usize, value: usize) -> Self {
        let size = 2_usize.pow(n_qubits as u32);
        assert!(value < size);
        let mut register = DVector::zeros(size);
        register[value] = Complex::one();

        Self::new_normalize(register)
    }

    pub fn singleton(qubit : Qubit) -> Self {
        Self::new_normalize(DVector::from_vec(vec![qubit.get_state().into_inner().x, qubit.get_state().into_inner().y]))
    }

    pub fn basis(n_qubits: usize, i:usize) -> Self {
        Self::from_int(n_qubits, i)
    }

    pub fn all_bases(n_qubits: usize) -> Vec<Self> {
        let mut bases = Vec::new();
        for i in 0..2_usize.pow(n_qubits as u32) {
            bases.push(Self::basis(n_qubits, i));
        }

        return bases;        
    }

    pub fn get_vector(&self) -> Unit<DVector<Complex<f32>>>{
        self.register.clone()
    }

    pub fn n_qubits(&self) -> usize {
        self.register.len().ilog2() as usize
    }

    pub fn get_probability(&self, i:usize) -> f32 {
        self.get_coeffcient(i).norm().powi(2)
    }

    pub fn get_coeffcient(&self, i:usize) -> Complex<f32> {
        self.register[i]
    }

    pub fn measure(&self) -> (Measurement, Self) {
        let mut rng = rand::thread_rng();
        let random_number = rng.gen_range(0.0_f32..1.0);
        let mut prob_so_far = 0.;

        for i in 0..self.len() {
            prob_so_far += self.get_probability(i);
            if random_number <= prob_so_far {

                let new_register = Self::basis(self.n_qubits(), i);
                return (i as u8, new_register);

            }
        }

        panic!("Measurement went wrong somehow");

    }

    pub fn len(&self) -> usize {
        self.register.len()
    }

    pub fn almost_equals(&self, rhs : impl Into<Self>) -> bool {
        let inner_rhs = rhs.into();
        assert_eq!(self.n_qubits(), inner_rhs.clone().n_qubits());
        (self.register.clone().into_inner() - inner_rhs.register.clone().into_inner()).norm() < 0.0001

    }

    pub fn tensor_product(&self, other : &Self) -> Self {
        Self::new_normalize(self.register.kronecker(&other.register))
    }


}



