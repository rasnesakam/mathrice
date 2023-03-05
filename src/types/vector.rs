use std::ops::{Index, IndexMut};

use super::errors::IndexOutOfBoundError;

#[derive(Clone, Copy)]
pub struct Vector<const N: usize, T: Sized + Copy>{
    pub size: usize,
    data: [Option<T>;N]
}


impl<const N: usize, T: Sized + Copy> Vector<N, T>{
    pub fn new(datas: [T;N])-> Vector<N, T>{
        Vector{
            size: N,
            data: datas.map( | x | Some(x))
        }
    }
    pub fn empty() -> Vector<N, T> {
        Vector {
            size: N,
            data: [None;N]
        }
    }

    pub fn get(&self, index: usize) -> Result<Option<T>, IndexOutOfBoundError> {
        if index > self.size {
            return Err(IndexOutOfBoundError { message: format!("Index out of bounds for {}",index) });
        }
        Ok(self.data[index])
    }

    pub fn set(mut self, index: usize, data: T) {
        self.data[index] = Some(data);
    }
}

impl <const N: usize, T: Sized + Copy> Index<usize> for Vector<N, T> {
	type Output = Option<T>;
	fn index(&self, index: usize) -> &Self::Output {
		return &self.data[index];
	}
}

impl <const N: usize, T: Sized + Copy> IndexMut<usize> for Vector<N, T> {
	fn index_mut(self: &mut Vector<N, T>, index: usize) -> &mut Option<T> {
		return &mut self.data[index];
	}
}
