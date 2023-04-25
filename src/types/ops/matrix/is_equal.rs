use std::cmp::PartialEq;
use std::cmp::Eq;
use crate::types::Matrix;

impl <const R: usize, const C: usize, T: PartialEq>PartialEq for Matrix<R,C,T>{
	fn eq(&self, other: &Self) ->bool{
		self.is_equal(other)
	}
}

impl <const R: usize, const C: usize, T: Eq>Eq for Matrix<R,C,T> {}