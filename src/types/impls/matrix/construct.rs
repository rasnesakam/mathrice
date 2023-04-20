use crate::types::Matrix;


impl <const R: usize, const C: usize, T: Copy> Matrix<R,C,T> {
	pub fn new(data: [[T;C];R]) -> Matrix<R,C,T> {
		Matrix {
			col_size: C,
			row_size: R,
			data
		}
	}
}

impl <const R: usize, const C: usize, T: Copy> Matrix<R,C,T> {
	pub fn fill_with_value(default: T) -> Matrix<R,C,T> {
		Matrix {
			col_size: C,
			row_size: R,
			data: [[default;C];R]
		}
	}
}

impl <const R: usize, const C: usize, T: Copy> Matrix<R,C,T> {
	pub fn fill_with(supplier: fn(usize,usize) -> T) -> Matrix<R,C,T> {
		let mut data:[[T;C];R] = [[supplier(0,0);C];R];
		for r in 0..R {
			for c in 0..C {
				data[r][c] = supplier(r,c);
			}
		}
		Matrix {
			col_size: C,
			row_size: R,
			data
		}
	}
}