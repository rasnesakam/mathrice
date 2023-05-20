use crate::types::Vector;


impl<const N: usize, T: Sized + Copy> Vector<N, T>{
	pub fn new(datas: [T;N])-> Vector<N, T>{
		Vector{
			size: N,
			data: datas
		}
	}
}

impl <const N: usize, T: Copy + std::default::Default> Vector<N,T> {
	pub fn empty() -> Vector<N, T> {
		Vector {
			size: N,
			data: [Default::default();N]
		}
	}
}