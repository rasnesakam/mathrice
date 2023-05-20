use std::ops::Add;
use crate::types::Vector;

macro_rules! add_impl {
    ($($t: ty)*) => ($(

        impl <const N: usize, T> Add<Vector<N, T>> for $t where
        T: Sized + Copy + std::ops::Add + Add<$t, Output = T> {
            
            type Output = Vector<N, T>;
            fn add(self, rhs: Vector<N, T>) -> Self::Output {
                todo!("")
            }
        }

    )*);
}

impl <const N: usize, T> Add<Vector<N, T>> for Vector<N, T> where 
    T: Sized + Copy + std::ops::Add + Add<Output = T> {
    
    type Output = Vector<N, T>;
    
    fn add(self, rhs: Vector<N, T>) -> Self::Output {
		todo!("")
    }
}

add_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64 }
