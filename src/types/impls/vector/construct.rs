use crate::types::Vector;


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
