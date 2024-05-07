use nalgebra::UnitVector2;
use num::Complex;

pub struct Math {

}

impl Math {
  pub fn kronecker(a: &Vec<Complex<f32>>, b: &UnitVector2<Complex<f32>>) -> Vec<Complex<f32>> {
    let mut result = Vec::new();
    for &ai in a.iter() {
      for &bi in b.iter() {
        result.push(ai* bi);
      }
    }
    result
  }

}
