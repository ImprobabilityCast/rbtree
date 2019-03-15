use std::cmp::PartialOrd;
use std::cmp::Ordering;


#[derive(PartialEq)]
#[derive(Debug)]
pub enum BTree<T: PartialOrd> {
    Empty,
    Branch (Node<T>)
}

#[derive(PartialEq)]
#[derive(Debug)]
pub struct Node<T: PartialOrd> {
    is_red: bool,
    pub root: T,
    pub left: Box<BTree<T>>,
    pub right: Box<BTree<T>>,
}

fn try_recolor<T: PartialOrd>(b: &mut BTree<T>) {

} 

fn fix_insert<T: PartialOrd>(b: &mut BTree<T>) {
    try_recolor(b);
}

pub fn insert<T: PartialOrd>(mut b: &mut BTree<T>, data: T) {
    use BTree::*;
    
    while let Branch(node) = b {
        if data.partial_cmp(&node.root) == Some(Ordering::Less) {
            b = &mut node.left;
        } else {
            b = &mut node.right;
        }
    }
    *b = Branch(Node::<T> {is_red: true, root: data, left: Box::new(Empty), right: Box::new(Empty)});
    fix_insert(b);
}


// to see output run with:
// cargo test -- --nocapture

#[cfg(test)]
mod test {
    use crate::*;
    use crate::BTree::*;

    #[test]
    fn test_ctor_empty() {
        let a = BTree::<i32>::Empty;
        println!("{:#?}", a);
    }

    #[test]
    fn test_insert_one() {
        let b = BTree::<i32>::Empty;
        insert(&mut Empty, 43);
        println!("{:#?}", b);
    }

    #[test]
    fn test_insert_2() {
        let mut b = Empty;
        let mut i = 0;
        while i < 2 {
            insert(&mut b, i);
            i += 1;
        }
        println!("{:#?}", b);
    }

    #[test]
    fn test_insert_3() {
        let mut b = Empty;
        let mut i = 0;
        while i < 3 {
            insert(&mut b, i);
            println!("{:#?}", b);
            i += 1;
        }
    }

    #[test]
    fn test_insert_7() {
        let mut b = Empty;
        let mut i = 0;
        while i < 7 {
            insert(&mut b, i);
            i += 1;
        }
        println!("{:#?}", b);
    }
}

