use std::ops::Index;

#[derive(Clone, Copy)]
pub struct Vector<const N: usize, T: Sized + Copy>{
    size: usize,
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
}

impl <const N: usize, T: Sized + Copy> Index<usize> for Vector<N, T> {
	type Output = Option<T>;
	fn index(&self, index: usize) -> &Self::Output {
		return &self.data[index];
	}
}
