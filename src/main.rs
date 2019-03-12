

#[derive(Debug)]
struct BTree<T> {
    left: Option<Box<BTree<T>>>,
    right: Option<Box<BTree<T>>>,
    data: T
}


impl BTree<i32> {
    pub fn new(d: i32) -> BTree<i32> {
        BTree {
            left: Option::None,
            right: Option::None,
            data: d
        }
    }


}

fn main() {
    let bt = BTree::new(23);
    println!("left: {:#?}, right: {:#?}, data: {}", bt.left, bt.right, bt.data);
}
