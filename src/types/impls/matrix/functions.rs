use crate::{types::{Matrix, Vector}, errors::IndexOutOfBoundError};

// Equals overload
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

// Getter methods
impl <const R: usize, const C: usize, T> Matrix<R,C,T> {
	
	/// get_element(&self, r: usize, c: usize )
	/// --------------------------------
	/// 
	/// Get elements from desired index row and column
	/// 
	/// ### Parameters
	/// `r: usize`: Index for the row of requested element.
	///   
	/// `c: usize`: Index for the column of requested element.
	/// 
	/// ### Returns
	/// A ***reference*** to the element that contains in specified row and column
	/// 
	/// ### Panics
	/// Panics when `r` or `c` specified higher than capacity
	pub fn get_element(&self,r: usize, c: usize) -> &T {
		if r >= R || c >= C {
			panic!("Index out of bounds for size ({R}x{C}), requested ({r}x{c})");
		}
		&self.data[r][c]
	}

	/// try_get_element(&self, r: usize, c: usize )
	/// --------------------------------
	/// 
	/// Tries to get elements from desired index row and column.
	/// If element does not exist, returns an error
	/// 
	/// ### Parameters
	/// `r: usize`: Index for the row of requested element.
	///   
	/// `c: usize`: Index for the column of requested element.
	/// 
	/// ### Returns
	/// A ***reference*** to the element that contains in specified row and column
	/// 
	/// **OR**
	/// 
	/// `IndexOutOfBoundError` in case of exceed capacity
	/// 
	pub fn try_get_element(&self, r: usize, c: usize) -> Result<&T,IndexOutOfBoundError> {
		if r >= R || c >= C {
			return Err(IndexOutOfBoundError {
				message: format!("Index out of bounds for size ({R}x{C}), requested ({r}x{c})")
			 });
		}
		return Ok(self.get_element(r, c));
	}

	/// get_col(&self, col: usize)
	/// --------------------------
	/// Get desired column in the matrix as an array
	/// 
	/// ### Params
	/// `col: usize`: Column index
	/// 
	/// ### Returns
	/// An ***Array*** that holds **references** of the values in
	/// specified column `col`
	/// 
	/// ### Panics
	/// Panics when col number exceed capacity of the matrix
	pub fn get_col(&self, col: usize) -> [&T;C] {
		let mut data: [&T;C] = [self.get_element(0, col);C];
		let immutable_data: [&T;C];
		for r in 1..R {
			data[r] = self.get_element(r, col);
		}
		immutable_data = data;
		immutable_data
	}

	/// get_row(&self, row: usize)
	/// --------------------------
	/// Get desired row in the matrix as an array
	/// 
	/// ### Params
	/// `row: usize`: Row index
	/// 
	/// ### Returns
	/// An ***Array*** that holds **references** of the values in
	/// specified row `row`
	/// 
	/// ### Panics
	/// Panics when col number exceed capacity of the matrix
	pub fn get_row(&self, row: usize) -> [&T;C] {
		let mut data: [&T;C] = [self.get_element(row, 0);C];
		let immutable_data: [&T;C];
		for c in 1..C {
			data[c] = self.get_element(row,c);
		}
		immutable_data = data;
		immutable_data
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