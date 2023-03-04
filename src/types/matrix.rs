use crate::types::vector::Vector;
use crate::types::errors::IndexOutOfBoundError;
use std::ops::Index;

pub struct Matrix<const COL: usize, const ROW: usize, T: Sized + Copy> {
    pub row: usize,
    pub col: usize,
    data: Vector<ROW, Vector<COL, T>>
}

impl <const COL: usize, const ROW: usize, T: Sized + Copy> Matrix<COL, ROW, T> {
	pub fn new(datas: [[T;COL];ROW]) -> Matrix<COL, ROW, T>{
		let vector: Vector<ROW, Vector<COL, T>> = Vector::empty();
		Matrix { row: ROW, col: COL, data: vector }
	}
    pub fn get(&self, row: usize, col: usize) -> Result<Option<T>, IndexOutOfBoundError> {
		if row >= self.row {
			return Err(IndexOutOfBoundError {message: format!("Cannot find row {}",row)});
		}
		if col > self.col {
			return Err(IndexOutOfBoundError {message: format!("Cannot find column {}", col)});
		}
        return Ok(self.data[row].and_then(|data_col: Vector<COL, T>| data_col[col]));
    }
}

impl <const COL: usize, const ROW: usize, T: Sized + Copy> Index<usize> for Matrix<COL, ROW, T> {
	type Output = Option<Vector<COL,T>>;
	fn index(&self, index: usize) -> &Self::Output {
		return &self.data[index];
	}
}

