use std::cmp::PartialOrd;
use std::cmp::Ordering;


#[derive(Debug)]
pub enum BTree<T: PartialOrd> {
    Empty,
    Branch(Node<T>, Box<BTree<T>>, Box<BTree<T>>)
}

#[derive(Debug)]
pub struct Node<T: PartialOrd> {
    pub root: T,
    is_red: bool
}

fn fix_insert<T: PartialOrd>(b: &mut BTree<T>) {

}

pub fn insert<T: PartialOrd>(b: &mut BTree<T>, data: T) {
    use BTree::*;
    match b {
        Empty => *b = Branch(Node::<T> {root: data, is_red: true}, Box::new(Empty), Box::new(Empty)),
        Branch(node, left, right) => {
            if data.partial_cmp(&node.root) == Some(Ordering::Less) {
                insert(&mut *left, data);
            } else {
                insert(&mut *right, data);
            }
        }
    }
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
    fn test_data_access() {
        let left = Box::new(Empty);
        let right = Box::new(Empty);
        let b = Branch(Node::<i32>{root: 200, is_red: true}, left, right);
        match b {
            Empty => println!("nothin'"),
            Branch(data, left, right) => println!("data: {}", data.root)
        }
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

