


#[cfg(test)]
mod test_qubit {
    use super::*;

    #[test]
    fn test_qubit_init(){
        assert_eq!(Qubit::basis(0), Qubit::new(1.0, 0.0));
        assert_eq!(Qubit::basis(1), Qubit::new(0.0, 1.0));

    } 


}