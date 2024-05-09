use std::collections::btree_map::Keys;
use std::f32::consts::{PI, SQRT_2};
use std::ops::{Mul};
use std::vec;

use nalgebra::{ DMatrix, matrix, Complex, dmatrix, Vector2, UnitVector2 };

use num::integer::sqrt;
use num_traits::{ float, One, Zero };
use std::f64::consts::E;
use crate::math::Math;
use crate::quantum_register::QuantumRegister;
use crate::qubit::{self, Qubit};


pub trait Apply {
  fn apply(&self, register : &mut QuantumRegister){

  }
}

#[derive(Clone)]
pub struct XGate {
  q1 : i32,
  matrix : DMatrix<Complex<f32>>,
  sign : char
}

impl XGate{
  pub fn h(q1 : i32) -> Self {
    let matrix : DMatrix<Complex<f32>> =  dmatrix![
      Complex::one() * 1. / SQRT_2, Complex::one() * 1. / SQRT_2;
      Complex::one() * 1. / SQRT_2, Complex::one() * -1. / SQRT_2
    ];
    let sign = 'H';

    Self{q1, matrix, sign}
  }

  pub fn x(q1 : i32) -> Self {
    let matrix : DMatrix<Complex<f32>> =  dmatrix![
      Complex::zero(), Complex::one();
      Complex::one(), Complex::zero()
    ];
    let sign = 'X';

    Self {q1, matrix, sign}
  }
  
  pub fn y(q1 : i32) -> Self {
    let matrix : DMatrix<Complex<f32>> =  dmatrix![
      Complex::zero(), Complex::i();
      Complex::i(), Complex::zero()
    ];
    let sign = 'Y';

    Self {q1, matrix, sign}
  }

  pub fn z(q1 : i32) -> Self{
    let matrix : DMatrix<Complex<f32>> =  dmatrix![
      Complex::one(), Complex::zero();
      Complex::zero(), Complex::one() * -1.
    ];
    let sign = 'Z';

    Self {q1, matrix, sign}
  }

  pub fn get_loc(&self) -> usize {
    self.q1 as usize
  }
  
  pub fn get_gate(&self) -> &DMatrix<Complex<f32>> {
    &self.matrix
  }
}
impl Apply for XGate {
  fn apply(&self, register : &mut QuantumRegister) {
    let size = register.get_size();
    let loc = self.get_loc();
    let identity: DMatrix<Complex<f32>> =  dmatrix![
      Complex::one(), Complex::zero();
      Complex::zero(), Complex::one()
    ];
    let mut matrix : DMatrix<Complex<f32>> = dmatrix![Complex::one()];

    for i in 0..size {
      if i == loc {
        let gate = self.get_gate();
        matrix = Math::kronecker_mat_mul(&matrix, &gate);

      }else{
        matrix = Math::kronecker_mat_mul(&matrix, &identity);

      }

    }

    let val = register.get_state_vector();
    register.set_register(matrix * val);
    
  }
}


#[derive(Clone)]
pub struct CXGate {
  q1 : i32,
  q2 : i32,
  matrix : DMatrix<Complex<f32>>,
  sign : &'static str
}


impl CXGate {
  pub fn c_x(q1 : i32, q2 : i32) -> Self {
    let matrix : DMatrix<Complex<f32>> =  dmatrix![
      Complex::one(), Complex::zero(), Complex::zero(), Complex::zero();
      Complex::zero(), Complex::one(), Complex::zero(), Complex::zero();
      Complex::zero(), Complex::zero(), Complex::zero(), Complex::one();
      Complex::zero(), Complex::zero(), Complex::one(), Complex::zero();
    ];
    let sign = "C_X";

    Self {q1, q2, matrix, sign}
  }

  pub fn c_y(q1 : i32, q2 : i32) -> Self {
    let matrix : DMatrix<Complex<f32>> =  dmatrix![
      Complex::one(), Complex::zero(), Complex::zero(), Complex::zero();
      Complex::zero(), Complex::one(), Complex::zero(), Complex::zero();
      Complex::zero(), Complex::zero(), Complex::zero(), Complex::i() * -1.;
      Complex::zero(), Complex::zero(), Complex::i(), Complex::zero();
    ];
    let sign = "C_Y";

    Self {q1, q2, matrix, sign}
  }

  pub fn c_z(q1 : i32, q2 : i32) -> Self {
    let matrix : DMatrix<Complex<f32>> =  dmatrix![
      Complex::one(), Complex::zero(), Complex::zero(), Complex::zero();
      Complex::zero(), Complex::one(), Complex::zero(), Complex::zero();
      Complex::zero(), Complex::zero(), Complex::one(), Complex::zero();
      Complex::zero(), Complex::zero(), Complex::zero(), Complex::one() * -1.;
    ];
    let sign = "C_Z";

    Self {q1, q2, matrix, sign}

  }

}
impl Apply for CXGate {
  fn apply(&self, register : &mut QuantumRegister) {
      
  }
}

#[derive(Clone)]
pub struct CCXGate{
  q1 : i32,
  q2 : i32,
  q3 : i32,
  matrix : DMatrix<Complex<f32>>,
  sign : &'static str
}

impl CCXGate {
  pub fn cc_x(q1 : i32, q2 : i32, q3 : i32) -> Self {
    let matrix : DMatrix<Complex<f32>> =  dmatrix![
      Complex::one(), Complex::zero(), Complex::zero(), Complex::zero(), Complex::zero(), Complex::zero(), Complex::zero(), Complex::zero();
      Complex::zero(), Complex::one(), Complex::zero(), Complex::zero(), Complex::zero(), Complex::zero(), Complex::zero(), Complex::zero();
      Complex::zero(), Complex::zero(), Complex::one(), Complex::zero(), Complex::zero(), Complex::zero(), Complex::zero(), Complex::zero();
      Complex::zero(), Complex::zero(), Complex::zero(), Complex::zero(), Complex::zero(), Complex::zero(), Complex::zero(), Complex::one();
      Complex::zero(), Complex::zero(), Complex::zero(), Complex::zero(), Complex::one(), Complex::zero(), Complex::zero(), Complex::zero();
      Complex::zero(), Complex::zero(), Complex::zero(), Complex::zero(), Complex::zero(), Complex::one(), Complex::zero(), Complex::zero();
      Complex::zero(), Complex::zero(), Complex::zero(), Complex::zero(), Complex::zero(), Complex::zero(), Complex::one(), Complex::zero();
      Complex::zero(), Complex::zero(), Complex::zero(), Complex::one(), Complex::zero(), Complex::zero(), Complex::zero(), Complex::zero();
    ];
    let sign = "CC_X";

    Self {q1, q2, q3, matrix, sign}
  }

  pub fn cc_z(q1 : i32, q2 : i32, q3 : i32) -> Self {
    let matrix : DMatrix<Complex<f32>> =  dmatrix![
      Complex::one(), Complex::zero(), Complex::zero(), Complex::zero(), Complex::zero(), Complex::zero(), Complex::zero(), Complex::zero();
      Complex::zero(), Complex::one(), Complex::zero(), Complex::zero(), Complex::zero(), Complex::zero(), Complex::zero(), Complex::zero();
      Complex::zero(), Complex::zero(), Complex::one(), Complex::zero(), Complex::zero(), Complex::zero(), Complex::zero(), Complex::zero();
      Complex::zero(), Complex::zero(), Complex::zero(), Complex::one(), Complex::zero(), Complex::zero(), Complex::zero(), Complex::zero();
      Complex::zero(), Complex::zero(), Complex::zero(), Complex::zero(), Complex::one(), Complex::zero(), Complex::zero(), Complex::zero();
      Complex::zero(), Complex::zero(), Complex::zero(), Complex::zero(), Complex::zero(), Complex::one(), Complex::zero(), Complex::zero();
      Complex::zero(), Complex::zero(), Complex::zero(), Complex::zero(), Complex::zero(), Complex::zero(), Complex::one(), Complex::zero();
      Complex::zero(), Complex::zero(), Complex::zero(), Complex::zero(), Complex::zero(), Complex::zero(), Complex::zero(), Complex::one() * -1.;
    ];
    let sign = "CC_Z";

    Self {q1, q2, q3, matrix, sign}
  }

}

impl Apply for CCXGate {
  fn apply(&self, register : &mut QuantumRegister) {
      
  }
}


#[cfg(test)]
mod test_register {
  use super::*;

  #[test]
  fn test_apply_XGate() {
    let cnot = XGate::x(0);
    let mut test_register = QuantumRegister::new(vec![Qubit::basis_0()]);
    cnot.apply(&mut test_register);
    let result: DMatrix<Complex<f32>> = test_register.get_state_vector().clone();
    assert_eq!(result, dmatrix![Complex::zero(), Complex::one()].transpose());

    let cnot = XGate::x(0);
    let mut test_register = QuantumRegister::new(vec![Qubit::basis_0(), Qubit::basis_0()]);
    cnot.apply(&mut test_register);
    let result: DMatrix<Complex<f32>> = test_register.get_state_vector().clone();
    assert_eq!(result, dmatrix![Complex::zero(), Complex::zero(), Complex::one(), Complex::zero()].transpose());

  
  }
}