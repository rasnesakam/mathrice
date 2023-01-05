use std::ops::Index;

pub struct Vector<T>{
    size: usize,
    data: Box<[T]>
}

impl<T> Vector<T>{
    fn new(datas: Box<[T]>)-> Vector<T>{
        return Vector{
            size: datas.len(),
            data: datas
        };
    }
}

impl <T> Index<usize> for Vector<T> {
	type Output = T;
	fn index(&self, index: usize) -> &Self::Output {
		return &self.data[index];
	}
}
