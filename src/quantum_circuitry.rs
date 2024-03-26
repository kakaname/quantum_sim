use nalgebra::{Complex, UnitVector2, Normed};
use num_traits::{One, Zero};
use crate::{qubit::Qubit};
pub struct QuantumCicuitry {


}

impl QuantumCicuitry {
    pub fn full_adder(lhs : Qubit, rhs : Qubit, carry_in : Qubit) -> (Qubit, Qubit) { // (bit, remainder)
        (Qubit::basis_0(), Qubit::basis_0())

    }
    pub fn measurer(vec : &Vec<Qubit>) -> Vec<Complex<f32>>{
        let base : i32 = 2;
        let base = base.pow(vec.len() as u32) as usize;
        let mut return_vec : Vec<Complex<f32>> = Vec::new();

        for i in 0..base{
            return_vec.push(QuantumCicuitry::get_amplitude(i as i32, &vec));
        }

        return_vec
    }
    pub fn measurer_inverse(vec : Vec<Complex<f32>>) -> Vec<Qubit> {

        vec![Qubit::basis_0(); 2]
    }
    pub fn get_amplitude(digit : i32, vec: &Vec<Qubit>) -> Complex<f32> {
        let mut digit = digit;
        let mut val = (vec.len() - 1) as u32;
        let mut amplitude: Complex<f32> = Complex::one();
        let base : i32 = 2;

        // should be more efficent method but bare bones for now
        for qubit in vec {
            let state = qubit.get_state().into_inner();
            if(digit >= base.pow(val as u32)){
                amplitude *= state.y.powi(2);
                digit -= base.pow(val as u32);
            }else{
                amplitude *= state.x.powi(2);
            }

            // because of integer overflow
            if val == 0 {
                break;
            }else{
                val -= 1;
            }
        }
        amplitude
    }
}

#[cfg(test)]
mod test_quantum_circuitry {
    use nalgebra::{ComplexField, Vector2};

    use super::*;

    #[test]
    fn test_get_amplitude(){
        // first we need to test the the sum of the amplitude is almost == 1
        let half_half = Qubit::new(UnitVector2::new_normalize(Vector2::new(Complex::one()*0.5_f32.sqrt(), Complex::one() * 0.25_f32.sqrt())));
        let vec = vec![half_half, half_half];
        let base : i32 = 2;
        let base = base.pow(vec.len() as u32) as usize;
        let mut sum = 0.;

        for i in 0..base{
            let amp = QuantumCicuitry::get_amplitude(i as i32, &vec).real();
            sum += amp;
        }
        assert!((sum - 1.).abs() < 0.0001);

        // check that (basis_1, basis_1) has 100% chance for [11]
        let vec = vec![Qubit::basis_1(), Qubit::basis_1()];
        let sum = QuantumCicuitry::get_amplitude(3, &vec);
        assert!((sum - 1.).abs() < 0.0001);

        // check that (basis_1, basis_0) has 100% chance for [10]
        let vec = vec![Qubit::basis_1(), Qubit::basis_0()];
        let sum = QuantumCicuitry::get_amplitude(2, &vec);
        assert!((sum - 1.).abs() < 0.0001);

        // check that (basis_0, basis_0) has 100% chance for [00]
        let vec = vec![Qubit::basis_0(), Qubit::basis_0()];
        let sum = QuantumCicuitry::get_amplitude(0, &vec);
        assert!((sum - 1.).abs() < 0.0001);


    }

    #[test]
    fn test_measurer() {

    }

}