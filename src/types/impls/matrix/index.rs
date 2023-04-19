use std::ops::Index;

use crate::types::Matrix;



impl <const COL: usize, const ROW: usize, T: Sized + Copy> Index<usize> for Matrix<ROW, COL, T> {
	type Output = [T;COL];
	fn index(&self, index: usize) -> &Self::Output {
		return &self.data[index];
	}
}
