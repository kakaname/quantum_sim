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
    pub fn basis_0(state : UnitVector2<Complex<f32>>) -> Self{
        Self::new(UnitVector2::new_normalize(Vector2::new(Complex::one(), Complex::zero())))
    }
    pub fn basis_1(state : UnitVector2<Complex<f32>>) -> Self{
        Self::new(UnitVector2::new_normalize(Vector2::new(Complex::zero(), Complex::one())))
    }
    pub fn mix(lhs : Self, rhs: Self, lhs_weight: f32, rhs_weight: f32) -> Self{
        let lhs_coefficent = (lhs_weight / (lhs_weight + rhs_weight)).sqrt();
        let rhs_coefficent = (rhs_weight / (lhs_weight + rhs_weight)).sqrt();
        Self::new_normalize(lhs_coefficent * lhs.state.into_inner() + rhs_coefficent * rhs.state.into_inner())
    }
}

impl Add<Qubit> for Qubit {
    type Output = Self;

    fn add(self, rhs : Self) -> Self::Output {
        let resultant: Vector2<Complex<_>> = self.state.into_inner() + rhs.state.into_inner();
        Self::new(UnitVector2::new_normalize(resultant))

    }
}

impl Mul<Qubit> for f32 {
    type Output = Qubit;

    fn mul(self, rhs : Qubit) -> Self::Output {
        Self::new(self * rhs.state)

    }
}


#[cfg(test)]
mod test_qubit {
    use super::*;

    #[test]
    fn test_qubit_init(){

    } 


}