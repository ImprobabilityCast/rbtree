#[derive(Debug)]
struct BTree<T> {
    left: Option<Box<BTree<T>>>,
    right: Option<Box<BTree<T>>>,
    data: T
}

impl <T> BTree<T> {
    fn new(param: T) -> BTree<T> {
        BTree::<T> {left: None, right: None, data: param}
    }
}

#[derive(Debug)]
enum BTree2<T> {
    Empty,
    Assemble(T, Box<BTree2<T>>, Box<BTree2<T>>)
}


#[test]
fn a_test() {
    let a = BTree::<i32> {left: None, right: None, data: 32};
    println!("{:#?}", a);
}

//#[test]
fn test_ctor_empty() {
    let a = BTree2::<i32>::Empty;
    println!("{:#?}", a);
}

//#[test]
fn test_ctor_non_empty() {
    use BTree2::*;
    let left = Box::new(Empty);
    let right = Box::new(Empty);
    let b = Assemble(200, left, right);
    match b {
        Empty => println!("nothin'"),
        Assemble(data, left, right) => println!("data: {}", data)
    }
}

