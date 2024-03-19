use std::ops::{Mul, Add};

use nalgebra::{UnitVector2, Complex, Vector2, Unit, Vector};
use num_traits::identities::One;
use num_traits::Zero;




#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Qubit {
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
    pub fn mix(lhs : Self, rhs: Self, lhs_weight: impl Into<Complex<f32>>, rhs_weight: impl Into<Complex<f32>>) -> Self{
        let lhs_coefficent = lhs_weight.into();
        let rhs_coefficent = rhs_weight.into();
        Self::new_normalize(lhs.state.into_inner().mul(lhs_coefficent) + rhs.state.into_inner().mul(rhs_coefficent))
    }
    pub fn almost_equals(&self , rhs : &Self) -> bool {
        (self.state.into_inner() - rhs.state.into_inner()).norm() < 0.0001
    }
}


#[cfg(test)]
mod test_qubit {
    use super::*;

    #[test]
    fn test_qubit_initalizes(){
        assert_eq!(Qubit::basis_0(), Qubit::new(UnitVector2::new_normalize(Vector2::new(Complex::one(), Complex::zero()))));
        assert_eq!(Qubit::basis_1(), Qubit::new(UnitVector2::new_normalize(Vector2::new(Complex::one(), Complex::zero()))));
    } 

    #[test]
    fn test_qubit_mixes() { 
        let mixed_state = Qubit::mix(Qubit::basis_0(), Qubit::basis_1(), 1, 3.0_f32.sqrt());
        assert!(mixed_state.almost_equals(&Qubit::new(UnitVector2::new_normalize(Vector2::new(Complex::one() * 0.25_f32.sqrt(), Complex::one() * 0.75_f32.sqrt())))));
    }


}