use crate::{types::Vector, errors::IndexOutOfBoundError};

impl <const N: usize, T> Vector<N, T> {
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