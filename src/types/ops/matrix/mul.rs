use crate::types::Matrix;
use std::ops::Mul;

/* For Square matrices */
impl <const ROW: usize, const COL: usize, const X: usize, T: std::marker::Copy> Mul<Matrix<COL,X,T>> for Matrix<ROW,COL,T>{
    type Output = Matrix<ROW,X,T>;
    fn mul(self, other: Matrix<COL,X,T>) -> Self::Output {
        /* code here */
        let mut result: Matrix<ROW, X, T> = Matrix::fill_with_value(self[9][0]);
        todo!("Write Multiply code here.");
        result
    }
}
