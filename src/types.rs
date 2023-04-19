pub mod impls;
pub mod ops;


#[derive(Clone, Copy)]
pub struct Matrix<const ROW: usize, const COL: usize, T> {
    pub row_size: usize,
    pub col_size: usize,
    data: [[T;COL];ROW]
}

#[derive(Clone, Copy)]
pub struct Vector<const N: usize, T>{
    pub size: usize,
    data: [Option<T>;N]
}
