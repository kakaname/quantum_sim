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

  pub fn kronecker_mat_mul(a : &DMatrix<Complex<f32>>, b : &DMatrix<Complex<f32>>) -> DMatrix<Complex<f32>> {
    let (a_rows, a_cols) = a.shape();
    let (b_rows, b_cols) = b.shape();

    let result_elements = (0..a_rows).into_par_iter().flat_map_iter(|r_a| {
        (0..a_cols).flat_map(move |c_a| {
            let value = a[(r_a, c_a)];
            (0..b_rows).flat_map(move |r_b| {
                (0..b_cols).map(move |c_b| {
                    ((r_a * b_rows + r_b, c_a * b_cols + c_b), value * b[(r_b, c_b)])
                })
            })
        })
    }).collect::<Vec<_>>();

    let mut result = DMatrix::zeros(a_rows * b_rows, a_cols * b_cols);
    for ((r, c), val) in result_elements {
        result[(r, c)] = val;
    }

    result
  }

}
