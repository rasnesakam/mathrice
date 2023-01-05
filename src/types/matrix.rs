use crate::types::vector::Vector;
use crate::types::errors::IndexOutOfBoundError;
use std::ops::Index;

pub struct Matrix<T> {
    pub row: usize,
    pub col: usize,
    data: Vector<Vector<T>>
}

impl <T> Matrix<T> {
	pub fn new()
    pub fn get(&self, row: usize, col: usize) -> Result<&T, IndexOutOfBoundError> {
		if row >= self.row {
			return Err(IndexOutOfBoundError {message: format!("Cannot find row {}",row)});
		}
		if col > self.col {
			return Err(IndexOutOfBoundError {message: format!("Cannot find column {}", col)});
		}
        return Ok(&self.data[row as usize][col as usize]);
    }
}

impl <T> Index<usize> for Matrix<T> {
	type Output = Vector<T>;
	fn index(&self, index: usize) -> &Self::Output {
		return &self.data[index];
	}
}

impl <T> Index<(usize, usize)> for Matrix<T>{
	type Output = T;
	fn index(&self, index: (usize, usize)) -> &Self::Output {
		return &self.data[index.0][index.1];
	}
}
