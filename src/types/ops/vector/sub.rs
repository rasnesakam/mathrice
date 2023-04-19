use crate::types::Vector;
use core::panic;
use std::ops::Sub;

impl <const N: usize, T> Sub<Vector<N, T>> for Vector<N, T> where
T: Sized + Copy + std::ops::Sub + Sub<Output = T> {
    type Output = Vector<N, T>;
    fn sub(self, rhs: Vector<N, T>) -> Self::Output {
        let vector: Vector<N, T> = Vector::empty();
        if self.size != rhs.size {
            panic!("Vector sizes mut be equal!");
        }
        for i in 0..vector.size {
            let subs = self[i].unwrap() - rhs[i].unwrap();
            vector.set(i, subs);
        }
        vector
    }
}