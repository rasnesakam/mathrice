use crate::{types::Vector, errors::IndexOutOfBoundError};

impl <const N: usize, T: Copy> Vector<N, T> {
	pub fn get(&self, index: usize) -> Result<&T, IndexOutOfBoundError> {
        if index > self.size {
            return Err(IndexOutOfBoundError { message: format!("Index out of bounds for {}",index) });
        }
        Ok(&self.data[index])
    }

    pub fn set(mut self, index: usize, data: T) {
        self.data[index] = data;
    }

	pub fn map<Fun: Fn(&T,usize)-> T>(&self, mapper: Fun) -> Vector<N,T>{
		let mut data: [T;N] = [mapper(&self.data[0],0);N];
		for i in 1..N {
			data[i] = mapper(&self.data[i],i);
		}
		let immutable_data: [T;N] = data;
		Vector::new(immutable_data)
	}
}


// Vector operations

impl <const N: usize, T: Copy> Vector<N,T> 
where T: std::ops::AddAssign + std::ops::Mul<Output = T> {
	pub fn dot(vec1: &Vector<N,T>, vec2: &Vector<N,T>) -> T {
		let mut result: T = vec1[0] * vec2[0];
		for i in 1..N {
			result += vec1[i] * vec2[i];
		}
		result
	}
}