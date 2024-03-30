use std::{collections::HashMap, fmt::Debug, hash::Hash, ops::Mul};
use nalgebra::{Complex, ComplexField, DMatrix, DVector, Normed, Unit};
use num_traits::{One, Zero};

type SparseMatrixRepresenation = HashMap<usize, HashMap<usize, Complex<f32>>>;

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

    fn mul(self, rhs:SparseMatrix) -> Self::Output {
        let mut result : SparseMatrixRepresenation = HashMap::new();
        for (i, row) in self.data.iter() {
            for (j, lhs_coeffcient) in row.iter() {
                if rhs.data.contains_key(j) {
                    for (k,rhs_coeffcient) in rhs.data.get(j).unwrap().iter() {
                        if result.contains_key(i){
                            if result.get_mut(i).unwrap().contains_key(k){
                                *result.get_mut(i).unwrap().get_mut(k).unwrap() += lhs_coeffcient * rhs_coeffcient;
                            }else{
                                result.get_mut(i).unwrap().insert(*k, lhs_coeffcient * rhs_coeffcient);
                            }
                        }else{
                            let mut new_row = HashMap::new();
                            new_row.insert(*k, lhs_coeffcient*rhs_coeffcient);
                            result.insert(*i, new_row);
                        }
                    }
                }
            }
        }
        SparseMatrix::new(self.size, result)
    }
}

impl Mul<SparseMatrix> for Complex<f32> {
    type Output = SparseMatrix;

    fn mul(self, rhs: SparseMatrix) -> Self::Output {
        rhs * self
    }
}

impl Mul<DVector<Complex<f32>>> for SparseMatrix {
    type Output = DVector<Complex<f32>>;

    fn mul(self, rhs: DVector<Complex<f32>>) -> Self::Output {
        let mut result = DVector::zeros(self.size);
        for (i, row) in self.data.iter() {
            for (j, coeffcient) in row.iter() {
                result[*i] += *coeffcient * rhs[*j];
            }
        }
        result

    }
}
impl Mul<Complex<f32>> for SparseMatrix {
    type Output = SparseMatrix;

    fn mul(self, rhs:Complex<f32>) -> Self::Output {
        let mut result: SparseMatrixRepresenation = HashMap::new();
        for (i, row) in self.data.iter() {
            for (j, coeffcient) in row.iter() {
                if result.contains_key(i){
                    result.get_mut(i).unwrap().insert(*j, *coeffcient * rhs);
                }else{
                    let mut new_row = HashMap::new();
                    new_row.insert(*j, *coeffcient * rhs);
                    result.insert(*i, new_row);
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

    pub fn size(&self) -> usize {
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

    pub fn tensor_product(&self, rhs : &Self ) -> Self {
        let mut result: SparseMatrixRepresenation = HashMap::new();

        for (i,row) in self.data.iter() {
            for(j, coeffcient) in row.iter() {
                let start_row = i * rhs.size;
                let start_column = j * rhs.size;
                for (k, rhs_row) in rhs.data.iter() {
                    for (l, rhs_coeffcient) in rhs_row.iter() {
                        if result.contains_key(&(start_row + k)){
                            if result.get_mut(&(start_row + k)).unwrap().contains_key(&(&start_column + l)){
                                *result.get_mut(&(&start_row + k)).unwrap().get_mut(&(start_column + l)).unwrap() += coeffcient * rhs_coeffcient;
                            }else{
                                result.get_mut(&(start_row + k)).unwrap().insert(start_column + l, coeffcient * rhs_coeffcient);
                            }
                        }else{
                            let mut new_row = HashMap::new();
                            new_row.insert(start_column + l, coeffcient*rhs_coeffcient);
                            result.insert(start_row + k, new_row);

                        }
                    }
                }

            }
        }
        SparseMatrix::new(self.size * rhs.size, result)
    

    }

}

#[derive(Clone)]
pub struct SquareMatrix{
    matrix : SparseMatrix,
}

impl Mul<SquareMatrix> for SquareMatrix {
    type Output = SquareMatrix;
    
    fn mul(self, rhs : SquareMatrix) -> Self::Output {
        SquareMatrix::new_unchecked(self.matrix * rhs.matrix)
    }
}

impl Mul<Unit<DVector<Complex<f32>>>> for SquareMatrix {
    type Output = DVector<Complex<f32>>;

    fn mul(self, rhs: Unit<DVector<Complex<f32>>>) -> Self::Output {
        self.matrix * rhs.into_inner()
    }
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
    pub fn from_vec_normalize(size : usize, vec : Vec<Complex<f32>>) -> Self {
        Self::new_unitary(SparseMatrix::from(DMatrix::from_vec(size,size,vec)))
    }

    pub fn get_coeffcient(&self, row: usize, col: usize) -> Complex<f32> {
        self.matrix.get(row, col)
    }

    pub fn identity(size: usize) -> Self {
        Self::new_unchecked(SparseMatrix::identity(size))
    }

    pub fn scale(&self, scalar: Complex<f32>) -> Self {
        Self::new_unitary(self.matrix.clone().mul(scalar))
    }

    pub fn get(&self, i : usize, j : usize) -> Complex<f32> {
        self.matrix.get(i, j)
    } 
    pub fn tensor_product(&self, rhs: &Self) -> Self {
        Self::new_unchecked(self.matrix.tensor_product(&rhs.matrix))
    }

    pub fn size(&self) -> usize {
        self.matrix.size()
    }

    pub fn invert(&self) -> Self {
        Self::new_unchecked(SparseMatrix::from(
            DMatrix::from(
                self.matrix.clone()
            ).try_inverse().expect("All unitary square matrices should be invertible")
        ))
    }
    pub fn permutation(permutation: Vec<usize>) -> Self { 
        assert!(permutation.len() > 0);
        assert!(permutation.iter().zip(permutation.iter().skip(1)).all(|(a,b)| a != b));

        let size = permutation.len();
        let mut data = HashMap::new();
        for (i,j) in permutation.iter().enumerate() {
            let row: HashMap<usize, Complex<f32>> = [(*j, Complex::one())].iter().cloned().collect();
            data.insert(i,row);
        }

        Self::new_unchecked(SparseMatrix::new(size, data))
    }


}