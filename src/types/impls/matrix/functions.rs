use crate::types::Matrix;


impl <const R: usize, const C: usize, T> Matrix<R,C,T>
	where T: std::cmp::PartialEq{
	pub fn is_equal(&self, other: &Matrix<R,C,T>)-> bool{
		for r in 0..R {
			for c in 0..C {
				if self[r][c] != other[r][c] {
					return false;
				}
			}
		}
		true
	}
}


// Basic arithmetic operations
impl <const R: usize, const C: usize, T> Matrix<R,C,T>
	where T:	std::marker::Copy + 
				std::ops::Add<Output = T> +
				std::ops::Sub<Output = T> +
				std::ops::Mul<Output = T> +
				std::ops::Div<Output = T>
 {
	pub fn add_matrix(&self, rhs: &Matrix<R,C,T>) -> Matrix<R,C,T> {
		Matrix::fill_with(|r: usize, c: usize| self[r][c] + rhs[r][c])	
	}
	pub fn sub_matrix(&self, rhs: &Matrix<R,C,T>) -> Matrix<R,C,T> {
		Matrix::fill_with(|r: usize, c: usize| self[r][c] - rhs[r][c])
	}
	pub fn add_constant(&self, constant: T) -> Matrix<R,C,T> {
		Matrix::fill_with(|r: usize, c: usize| self[r][c] + constant)
	}
	pub fn sub_constant(&self, constant: T) -> Matrix<R,C,T> {
		Matrix::fill_with(|r: usize, c: usize| self[r][c] - constant)
	}
	pub fn mul_constant(&self, constant: T) -> Matrix<R,C,T> {
		Matrix::fill_with(|r: usize, c: usize| self[r][c] * constant)
	}
	pub fn div_constant(&self, constant: T) -> Matrix<R,C,T> {
		Matrix::fill_with(|r: usize, c: usize| self[r][c] / constant)
	}
}

// Matrix multiplication
impl <const R: usize, const C: usize, T> Matrix<R,C,T>{
	pub fn mul_matrix<const ROW: usize>(&self, rhs: Matrix<ROW,R,T>) -> Matrix<R,R,T> {
		//Matrix::<R,R,T>::fill_with(|r: usize, c: usize| )
		todo!("Matrix multiplication will be implemented")
	}
}