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