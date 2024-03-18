


#[cfg(test)]
mod nalgebra_tests {
    #[test]
    fn test_can_multiply_complex_numbers_by_complex_vectors() {
        let v = UnitVector2::new_normalize(
            Vector2::new(Complex::from(1.0), Complex::from(1.0))
        );

        let v = Complex::new(3.0, 2.0) * v;

        assert!((v.x.re - 3.).abs() < 0.0001, "{}", v.x.re);
        assert!((v.x.im - 2.).abs() < 0.0001, "{}", v.x.im);
        assert!((v.y.re - 3.).abs() < 0.0001, "{}", v.y.re);
        assert!((v.y.im - 2.).abs() < 0.0001, "{}", v.y.im);


    }

}