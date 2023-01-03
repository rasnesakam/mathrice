pub struct Vector<T>{
    size: usize,
    data: Box<[T]>
}

impl<T> Vector<T>{
    fn new(datas: Box<[T]>)-> Vector<T>{
        return Vector{
            size: datas.len(),
            data: datas
        };
    }
}
