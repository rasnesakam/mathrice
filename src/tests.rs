use crate::types::{Vector, Matrix};

#[test]
fn it_works() {
    let vector: Vector<2,i32> = Vector::new([4,2]);
    assert_eq!(vector[0].expect("EYVAH"), 4);
    assert_ne!(vector[1].expect("EYVAH"), 4)
}

#[test]
fn vector_add_vector() {
    let vec1: Vector<2, i32> = Vector::new([5, 6]);
    let vec2: Vector<2, i32> = Vector::new([5, 4]);

    let vec3:Vector<2, i32> = vec1 + vec2;

    for i in 0..vec3.size {
        
        if vec3[i].is_some() {
            let sum = vec1[i].unwrap() + vec2[i].unwrap();
            let data = vec3[i].unwrap();
            assert_eq!(data, sum )
        }
        else {
            assert!(true, "Result is null at {}",i);
        }
    }
}



#[test]
fn print_matrix(){

    let matrix: Matrix<3,3,i32> = Matrix::new([[1,0,0],[0,1,0],[0,0,1]]);
    println!("{}",matrix);

}
