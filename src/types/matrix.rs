use crate::types::Vector;

pub struct Matrix<T> {
    pub row: i32,
    pub col: i32,
    data: Vector<Vector>
}

impl <T> Matrix<T> {
    pub fn get(&self, row: u64, col: u64) -> T {
        self.data[row][col];
    }
}
