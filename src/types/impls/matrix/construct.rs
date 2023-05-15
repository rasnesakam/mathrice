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
	pub fn fill_with<F: Fn(usize,usize) -> T>(supplier: F) -> Matrix<R,C,T> {
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

// numeric initializers

macro_rules! numeric_instantiates {
	($($ty: ty)*) => ($(
		impl <const R: usize, const C: usize> Matrix<R,C,$ty> {
			pub fn identity() -> Matrix<R,C,$ty> {
				Matrix::fill_with(|r: usize, c: usize| if r == c {1.0 as $ty} else { 0.0 as $ty})
			}
			pub fn zero() -> Matrix<R,C,$ty> {
				Matrix::fill_with_value(0.0 as $ty)
			}
		}
	)*);
}

numeric_instantiates!(usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64);
