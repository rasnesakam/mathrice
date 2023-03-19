use crate::types::vector::Vector;
use crate::types::errors::IndexOutOfBoundError;
use std::fmt::Display;
use std::ops::Index;

macro_rules! integral_numeric_matrix_impls {
	($($t: ty)*) => ($(
		impl <const ROW: usize> Matrix<ROW, ROW, $t>{
			pub fn identity() ->Matrix<ROW, ROW, $t>{
				let mut data: [[$t;ROW];ROW] = [[0;ROW];ROW];
				for i in 0..ROW {
					data[i][i] = 1;
				}
				Matrix {
					row_size: ROW,
					col_size: ROW,
					data: data
				}
			}
		}
	)*);
}

macro_rules! decimal_numeric_matrix_impls {
	($($t: ty)*) => ($(
		impl <const ROW: usize> Matrix<ROW, ROW, $t>{
			pub fn identity() ->Matrix<ROW, ROW, $t>{
				let mut data: [[$t;ROW];ROW] = [[0.0;ROW];ROW];
				for i in 0..ROW {
					data[i][i] = 1.0;
				}
				Matrix {
					row_size: ROW,
					col_size: ROW,
					data: data
				}
			}
		}
	)*);
}


pub struct Matrix<const ROW: usize, const COL: usize, T: Sized + Copy> {
    pub row_size: usize,
    pub col_size: usize,
    data: [[T;COL];ROW]
}

integral_numeric_matrix_impls!(usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 );

decimal_numeric_matrix_impls!(f32 f64);

impl <const ROW: usize, const COL: usize, T> Matrix<ROW, COL, T>
	where T: Sized + Copy{
	pub fn with(item_suplier: fn() -> T ) -> Matrix<ROW, COL, T>{
		//let vector: Vector<>
		Matrix {
			row_size: ROW,
			col_size: COL,
			data: [[item_suplier();COL];ROW]
		}
	}
	
	pub fn fill_with(item_suplier: fn(row: usize, col: usize) -> T) -> Matrix<ROW, COL, T> {
		let mut datas: [[T;COL];ROW] = [[item_suplier(0, 0);COL];ROW];
		
		for row in 0..ROW {
			for col in 0..COL {
				datas[row] = [item_suplier(row, col);COL];
			}
		}
		Matrix {
			row_size: ROW,
			col_size: COL,
			data: datas
		}
	}
}



impl <const ROW: usize, const COL: usize, T> Matrix<ROW, COL, T> where
	T: Sized + Copy{

	pub fn get(&self, row_index: usize, col_index: usize) -> Result<T,IndexOutOfBoundError> {
		if row_index < self.row_size {
			if col_index < self.col_size {
				return Ok(self.data[row_index][col_index])
			}
			return Err(IndexOutOfBoundError { 
				message: format!("Index out of bounds for column ({}). requested: {}", self.col_size, col_index)
			});
		}
		return Err(IndexOutOfBoundError { 
			message: format!("Index out of bounds for row ({}). requested: {}", self.row_size, row_index )
		});
	}

	pub fn get_col(&self, col_index: usize) -> Result<Matrix<ROW, 1, T>, IndexOutOfBoundError>{
		if col_index >= self.col_size {
            return Err(IndexOutOfBoundError { message: "Invalid column index".to_string() });
        }
		let mut datas: [[T;1];ROW] = [[self.data[0][col_index]];ROW];
		for row in 0..ROW {
			datas[row] = [self.data[row][col_index]];
		}
		Ok(Matrix {
			row_size: ROW,
			col_size: 1,
			data: datas
		})
	}
}

impl <const ROW: usize, const COL: usize, T> Matrix<ROW, COL, T>
	where T: Sized + Copy + Display{
	pub fn print(&self) {
		for i in 0..self.row_size {
			print!("[ ");
			for j in 0..self.col_size {
				print!("{}",self.data[i][j]);
				if j < self.col_size - 1 {
					print!(", ")
				}
			}
			println!(" ]");
		}
	}
}


impl <const COL: usize, const ROW: usize, T: Sized + Copy> Index<usize> for Matrix<ROW, COL, T> {
	type Output = [T;COL];
	fn index(&self, index: usize) -> &Self::Output {
		return &self.data[index];
	}
}

