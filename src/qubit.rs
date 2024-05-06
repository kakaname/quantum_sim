  
use nalgebra::{ComplexField, DMatrix};

use std::f32::consts::SQRT_2;
use std::fmt::{Display}; 

use nalgebra::{UnitVector2, Complex, Vector2};
use num_traits::identities::One;
use num_traits::Zero;

use rand::prelude::*;

use crate::quantum_gate::{CCXGate, CXGate, XGate};

pub type Measurement = u8;

impl Display for Qubit {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "{:?}", self.state) 
  }
}

impl PartialEq for Qubit {
  fn eq(&self, other: &Self) -> bool {
      self.almost_equals(other)
  }
}

#[derive(Clone, Copy, Debug)]
pub struct Qubit {
  // x: probability of 0
  // y: probability of 1
  state : UnitVector2<Complex<f32>>,
}


impl Qubit {

  pub fn new(state : UnitVector2<Complex<f32>>) -> Self{
    Self {state}
  }

  pub fn new_normalize(state : Vector2<Complex<f32>>) -> Self{
    Self::new(UnitVector2::new_normalize(state))
    
  }

  pub fn basis_0() -> Self{
    Self::new(UnitVector2::new_normalize(Vector2::new(Complex::one(), Complex::zero())))
  }

  pub fn basis_1() -> Self{
    Self::new(UnitVector2::new_normalize(Vector2::new(Complex::zero(), Complex::one())))
  }

  pub fn half_half() -> Self{
    Self::new(UnitVector2::new_normalize(Vector2::new(
      Complex::one(), 
      Complex::one() 
    )))
  }

  pub fn almost_equals(&self , rhs : &Self) -> bool {
    (self.state.into_inner() - rhs.state.into_inner()).norm() < 0.0001
  }

  pub fn get_state(&self) -> UnitVector2<Complex<f32>> {
    self.state
  }

  pub fn change_state(&mut self, state : UnitVector2<Complex<f32>>){
    self.state = state;
  }

  pub fn measure(&self) -> (Measurement, Self) {
    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(0.0_f32..1.0);
    let probability_of_0 = self.state.into_inner().x.abs().powi(2);
    if random_number < probability_of_0 {
      (0, Self::basis_0())
    }else{
      (1, Self::basis_1())
    }
  }

}


#[cfg(test)]
mod test_qubit {
  use super::*;

  #[test]
  fn test_qubit_initalizes(){
    assert_eq!(Qubit::basis_0(), Qubit::new(UnitVector2::new_normalize(Vector2::new(Complex::one(), Complex::zero()))));
    assert_eq!(Qubit::basis_1(), Qubit::new(UnitVector2::new_normalize(Vector2::new(Complex::zero(), Complex::one()))));
  } 

  #[test]
  fn test_qubit_measurement() {
    // should be 0
    let (basis_0_measure, new_state_qubit) = Qubit::basis_0().measure();
    assert_eq!(basis_0_measure, 0);
    assert_eq!(new_state_qubit, Qubit::basis_0());

    // should be 1
    let (basis_1_measure, new_state_qubit) = Qubit::basis_1().measure();
    assert_eq!(basis_1_measure, 1);
    assert_eq!(new_state_qubit, Qubit::basis_1());

    // should add tests to check randomization correctness

  }


}