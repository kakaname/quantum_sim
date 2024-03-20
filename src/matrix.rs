use std::{collections::HashMap, ops::Mul, fmt::Debug};
use nalgebra::{Complex, ComplexField, DMatrix, Normed, UnitVector2, Vector2};
use num_traits::{One, Zero};

type SparseMatrixRepresenation = HashMap<usize, HashMap<usize, Complex<f32>>>;

use crate::{qubit::Qubit};

#[derive(Debug, Clone)]
pub struct SparseMatrix{
    size : usize,
    data : SparseMatrixRepresenation,
}

impl From<DMatrix<Complex<f32>>> for SparseMatrix {
    fn from(matrix : DMatrix<Complex<f32>>) -> Self {
        let mut sparse_matrix: SparseMatrixRepresenation = HashMap::new();
        for (i, row) in matrix.row_iter().enumerate(){
            for (j, coeffcient) in row.iter().enumerate(){
                if !coeffcient.is_zero() {
                    if sparse_matrix.contains_key(&i){ // row exists
                        sparse_matrix.get_mut(&i).unwrap().insert(j, *coeffcient);

                    }else{ // create new row
                        let mut row = HashMap::new();
                        row.insert(j, *coeffcient);
                        sparse_matrix.insert(i,row);
                    }
                }
            }
        }   
        SparseMatrix::new(matrix.nrows(), sparse_matrix)
    }
}

impl From<SparseMatrix> for DMatrix<Complex<f32>> {
    fn from(sparse_matrix : SparseMatrix) -> Self {
        DMatrix::from_fn(
            sparse_matrix.size,
            sparse_matrix.size,
            |i, j| {
                if sparse_matrix.data.contains_key(&i) {
                    if sparse_matrix.data.get(&i).unwrap().contains_key(&j){
                        return *sparse_matrix.data.get(&i).unwrap().get(&j).unwrap();
                    }
                }
                Complex::zero()
            }
        )
    }

}

impl Mul<SparseMatrix> for SparseMatrix {
    type Output = SparseMatrix;

    fn mul(self, rhs: SparseMatrix ) -> Self::Output {
        let mut result: SparseMatrixRepresenation = HashMap::new();
        for (i, row) in self.data.iter() {
            for (j, lhs_coefficent) in row.iter() {
                if rhs.data.contains_key(j){
                    for (k, rhs_coefficent) in rhs.data.get(j).unwrap().iter() {
                        if result.contains_key(i){
                            if result.get_mut(i).unwrap().contains_key(k) {
                                *result.get_mut(i).unwrap().get_mut(k).unwrap() += lhs_coefficent * rhs_coefficent;
                            }else { 
                                result.get_mut(i).unwrap().insert(*k, lhs_coefficent * rhs_coefficent);
                            }
                        }
                    }
                }
            }
        }
        SparseMatrix::new(self.size, result)
    }

}

impl SparseMatrix {
    pub fn new(size : usize, data : SparseMatrixRepresenation) -> Self{
        Self {
            size,
            data
        }
    }

    pub fn size(&self, i:usize, j:usize) -> usize {
        self.size
    }

    pub fn identity(size: usize) -> Self {
        // create identity matrix of size size
        let mut data:SparseMatrixRepresenation = HashMap::new();
        for i in 0..size {
            let mut row = HashMap::new();
            row.insert(i, Complex::one());
            data.insert(i, row);
        }

        Self::new(size, data)
    }

    pub fn get(&self, i: usize, j: usize) -> Complex<f32>{
        match self.data.get(&i){
            Some(row) => match row.get(&j){
                Some(coefficent) => *coefficent,
                None => Complex::zero(),
            },
            None => Complex::zero(),
        }

    }


    pub fn almost_equals(&self, other : &Self) -> bool {
        for (i, row) in self.data.iter() {
            for (j, coefficient) in row.iter() {
                if !((other.get(*i, *j) - coefficient).abs() < 0.0001) {
                    return false
                }
            }
        }
        true

    }

}

#[derive(Clone)]
pub struct SquareMatrix{
    matrix : SparseMatrix,
}

impl SquareMatrix {
    pub fn new_unchecked(matrix : SparseMatrix) -> Self {
        Self {matrix}
    }
    pub fn new_unitary(matrix: SparseMatrix) -> Self {
        let dmatrix = DMatrix::from(matrix);
        assert!(dmatrix.is_square());
        let determinant_norm: f32 = dmatrix.determinant().norm();
        let normalizer = 1./(determinant_norm.powf(1./(dmatrix.nrows() as f32)));
        let normalized_matrix = dmatrix.scale(normalizer);
        assert!((normalized_matrix.determinant().norm() - 1.).abs() < 0.0001, "Matrix not unitary, {} ", normalized_matrix.determinant().norm());
        Self {matrix: SparseMatrix::from(normalized_matrix)}

    }
    pub fn from_vector_normalize(size : usize, vec : Vec<Complex<f32>>) -> Self {
        Self::new_unitary(SparseMatrix::from(DMatrix::from_vec(size,size,vec)))

    }
    pub fn get(self, i : usize, j : usize) -> Complex<f32>{
        self.matrix.get(i, j)

    }

}