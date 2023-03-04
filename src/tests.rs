use crate::types::vector::Vector;

#[test]
fn it_works() {
    let vector: Vector<2,i32> = Vector::new([4,2]);
    assert_eq!(vector[0].expect("EYVAH"), 4);
    assert_ne!(vector[1].expect("EYVAH"), 4)
}
