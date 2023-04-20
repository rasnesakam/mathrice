use crate::types::Matrix;
use std::ops::Add;

impl <const ROW: usize, const COL: usize, T> Add for Matrix<ROW,COL,T>
    where T:Sized + Copy + Add + Add<Output=T>{
    type Output = Matrix<ROW, COL, T>;
    fn add(self, rhs: Self) -> Self::Output {
		let mut data: [[T;COL];ROW] = self.data;
		for i in 0..ROW {
			for j in 0..COL {
				data[i][j] = self[i][j] + rhs[i][j];
			}
		}

		Matrix {
			col_size: COL,
			row_size: ROW,
			data
		}
    }
}
