use crate::types::Vector;
use core::panic;
use std::ops::Sub;

impl <const N: usize, T> Sub<Vector<N, T>> for Vector<N, T> where
T: Sized + Copy + std::ops::Sub + Sub<Output = T> {
    type Output = Vector<N, T>;
    fn sub(self, rhs: Vector<N, T>) -> Self::Output {
		todo!()
    }
}