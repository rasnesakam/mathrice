use crate::types::matrix::Matrix;
use std::ops::Add;

/*
impl <const ROW: usize, const COL: usize, T> Add for Matrix<ROW,COL,T>
    where T:Sized + Copy + Add{
    type Output = Matrix<ROW, COL, T>;
    fn add(self, rhs: Self) -> Self::Output {
        let matrix : Matrix<COL,ROW, T>;
    }
}
*/