

impl <T> Matrix<T> {
    fn get(&self, row: u64, col: u64) -> T {
        self.data[row][col];
    }
}
