use crate::types::{Vector, Matrix};

#[test]
fn test_it_works() {
    let vector: Vector<2,i32> = Vector::new([4,2]);
    assert_eq!(vector[0], 4);
    assert_ne!(vector[1], 4)
}

#[test]
fn test_vector_add_vector() {
    let vec1: Vector<2, i32> = Vector::new([5, 6]);
    let vec2: Vector<2, i32> = Vector::new([5, 4]);

    let vec3:Vector<2, i32> = vec1 + vec2;

    for i in 0..vec3.size {
        
        if vec3[i] > 0 {
            let sum = vec1[i] + vec2[i];
            let data = vec3[i];
            assert_eq!(data, sum )
        }
        else {
            assert!(true, "Result is null at {}",i);
        }
    }
}



#[test]
fn test_print_matrix(){

    let matrix: Matrix<3,3,i32> = Matrix::new([[1,0,0],[0,1,0],[0,0,1]]);
    println!("{}",matrix);

}

#[test]
fn test_matrix_equal_op(){
	let matrix_1: Matrix<3,3,i32> = Matrix::new([[1,0,0],[0,1,0],[0,0,1]]);
	let matrix_2: Matrix<3,3,i32> = Matrix::new([[1,0,0],[0,1,0],[0,0,1]]);
	let test: bool = matrix_1 == matrix_2;
	assert!(test,"Matrices are equal");
}

#[test]
fn test_matrix_not_equal_op(){
	let matrix_1: Matrix<3,3,i32> = Matrix::new([[0,0,0],[0,0,0],[0,0,0]]);
	let matrix_2: Matrix<3,3,i32> = Matrix::new([[1,1,1],[1,1,1],[1,1,1]]);
	let test: bool = matrix_1 == matrix_2;
	assert!(!test,"Matrices are not equal");
}

#[test]
fn test_matrix_equal(){
	let matrix_1: Matrix<3,3,i32> = Matrix::new([[1,0,0],[0,1,0],[0,0,1]]);
	let matrix_2: Matrix<3,3,i32> = Matrix::new([[1,0,0],[0,1,0],[0,0,1]]);
	let test: bool = matrix_1.is_equal(&matrix_2);
	assert!(test,"Matrices are equal");
}

#[test]
fn test_matrix_not_equal(){
	let matrix_1: Matrix<3,3,i32> = Matrix::new([[0,0,0],[0,0,0],[0,0,0]]);
	let matrix_2: Matrix<3,3,i32> = Matrix::new([[1,1,1],[1,1,1],[1,1,1]]);
	let test: bool = matrix_1.is_equal(&matrix_2);
	assert!(!test,"Matrices are not equal");
}

#[test]
fn test_matrix_mul(){
	let matrice_a: Matrix<3, 3, i32> = Matrix::new([
		[1,2,3],
		[4,5,6],
		[7,8,9]
	]);
	let matrice_b: Matrix<3,2,i32> = Matrix::new([
		[1,2],
		[3,4],
		[5,6]
	]);
	let matrice_c: Matrix<3, 2, i32> = matrice_a.mul_matrix(matrice_b);
	let matrice_d: Matrix<3, 2, i32> = Matrix::new([
		[22,28],
		[49,64],
		[76,100]
	]);
	assert!(matrice_c == matrice_d,"Expected result:\n{matrice_d}\nFound result:\n{matrice_c}");
}