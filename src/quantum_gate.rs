use std::f32::consts::{PI, SQRT_2};
use std::ops::{Mul};
use std::vec;

use nalgebra::{ DMatrix, Complex, dmatrix, Vector2, UnitVector2 };

use num::integer::sqrt;
use num_traits::{ float, One, Zero };
use std::f64::consts::E;
use crate::qubit::{self, Qubit};


#[derive(Clone, PartialEq)]
pub struct XGate {
  q1 : i32,
  matrix : DMatrix<Complex<f32>>,
  sign : char
}

impl XGate{
  pub fn h(q1 : i32) -> Self {
    let matrix : DMatrix<Complex<f32>> = dmatrix![
      Complex::one() * 1. / SQRT_2, Complex::one() * 1. / SQRT_2;
      Complex::one() * 1. / SQRT_2, Complex::one() * -1. / SQRT_2
    ];
    let sign = 'H';

    Self{q1, matrix, sign}
  }

  pub fn x(q1 : i32) -> Self {
    let matrix : DMatrix<Complex<f32>> = dmatrix![
      Complex::zero(), Complex::one();
      Complex::one(), Complex::zero()
    ];
    let sign = 'X';

    Self {q1, matrix, sign}
  }
  
  pub fn y(q1 : i32) -> Self {
    let matrix : DMatrix<Complex<f32>> = dmatrix![
      Complex::zero(), Complex::i();
      Complex::i(), Complex::zero()
    ];
    let sign = 'Y';

    Self {q1, matrix, sign}
  }

  pub fn z(q1 : i32) -> Self{
    let matrix : DMatrix<Complex<f32>> = dmatrix![
      Complex::one(), Complex::zero();
      Complex::zero(), Complex::one() * -1.
    ];
    let sign = 'Z';

    Self {q1, matrix, sign}
  }

}


pub struct CXGate {
  q1 : i32,
  q2 : i32,
  matrix : DMatrix<Complex<f32>>,
  sign : &'static str
}


impl CXGate {
  pub fn c_x(q1 : i32, q2 : i32) -> Self {
    let matrix : DMatrix<Complex<f32>> = dmatrix![
      Complex::one(), Complex::zero(), Complex::zero(), Complex::zero();
      Complex::zero(), Complex::one(), Complex::zero(), Complex::zero();
      Complex::zero(), Complex::zero(), Complex::zero(), Complex::one();
      Complex::zero(), Complex::zero(), Complex::one(), Complex::zero();
    ];
    let sign = "C_X";

    Self {q1, q2, matrix, sign}
  }

  pub fn c_y(q1 : i32, q2 : i32) -> Self {
    let matrix : DMatrix<Complex<f32>> = dmatrix![
      Complex::one(), Complex::zero(), Complex::zero(), Complex::zero();
      Complex::zero(), Complex::one(), Complex::zero(), Complex::zero();
      Complex::zero(), Complex::zero(), Complex::zero(), Complex::i() * -1.;
      Complex::zero(), Complex::zero(), Complex::i(), Complex::zero();
    ];
    let sign = "C_Y";

    Self {q1, q2, matrix, sign}
  }

  pub fn c_z(q1 : i32, q2 : i32) -> Self {
    let matrix : DMatrix<Complex<f32>> = dmatrix![
      Complex::one(), Complex::zero(), Complex::zero(), Complex::zero();
      Complex::zero(), Complex::one(), Complex::zero(), Complex::zero();
      Complex::zero(), Complex::zero(), Complex::one(), Complex::zero();
      Complex::zero(), Complex::zero(), Complex::zero(), Complex::one() * -1.;
    ];
    let sign = "C_Z";

    Self {q1, q2, matrix, sign}

  }

}

pub struct CCXGate{
  q1 : i32,
  q2 : i32,
  q3 : i32,
  matrix : DMatrix<Complex<f32>>,
  sign : &'static str
}

impl CCXGate {
  pub fn cc_x(q1 : i32, q2 : i32, q3 : i32) -> Self {
    let matrix : DMatrix<Complex<f32>> = dmatrix![
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
    let matrix : DMatrix<Complex<f32>> = dmatrix![
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

