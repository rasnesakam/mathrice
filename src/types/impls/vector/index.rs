use std::ops::{Index, IndexMut};

use crate::types::Vector;

impl <const N: usize, T> Index<usize> for Vector<N, T> {
	type Output = T;
	fn index(&self, index: usize) -> &Self::Output {
		return &self.data[index];
	}
}

impl <const N: usize, T> IndexMut<usize> for Vector<N, T> {
	fn index_mut(self: &mut Vector<N, T>, index: usize) -> &mut Self::Output {
		return &mut self.data[index];
	}
}
