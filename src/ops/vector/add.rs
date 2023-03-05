use std::ops::Add;
use crate::types::vector::Vector;

macro_rules! add_impl {
    ($($t: ty)*) => ($(

        impl <const N: usize, T> Add<Vector<N, T>> for $t where
        T: Sized + Copy + std::ops::Add + Add<$t, Output = T> {
            
            type Output = Vector<N, T>;
            fn add(self, rhs: Vector<N, T>) -> Self::Output {
                let vector: Vector<N, T> = Vector::empty();
                for i in 0..rhs.size {
                    let sum: T =  rhs.get(i).unwrap().unwrap() + self;
                    vector.set(i, sum);
                }
                vector
            }
        }

    )*);
}

impl <const N: usize, T> Add<Vector<N, T>> for Vector<N, T> where 
    T: Sized + Copy + std::ops::Add + Add<Output = T> {
    
    type Output = Vector<N, T>;
    
    fn add(self, rhs: Vector<N, T>) -> Self::Output {
    
        if self.size == rhs.size {
            let vector: Vector<N, T> = Vector::<N, T>::empty();
            for i in 0..vector.size {
                let sum: T = self.get(i).unwrap().unwrap() + rhs.get(i).unwrap().unwrap();
                vector.set(i, sum);
            }
            return vector;
        }
        panic!("EYVAH: Vector sizes are not equal");
    }
}

add_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64 }
