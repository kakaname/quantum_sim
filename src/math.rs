use nalgebra::{dmatrix, DMatrix, UnitVector2};
use num::Complex;
use std::ops::{Mul};
use rayon::prelude::*;
pub struct Math {

}

impl Math {
  pub fn kronecker_create_state_vector_qubits(a: &Vec<Complex<f32>>, b: &UnitVector2<Complex<f32>>) -> Vec<Complex<f32>> {
    let mut result = vec![];
    for &ai in a.iter() {
      for &bi in b.iter() {
        result.push(ai* bi);
      }
    }
    result
  }


}
