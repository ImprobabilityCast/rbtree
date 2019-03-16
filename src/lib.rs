use std::cmp::PartialOrd;
use std::cmp::Ordering;
use std::rc::Rc;
use std::cell::RefCell;
use std::fmt;

// I hate this type
type BTreeOpt<T> = Option<Rc<RefCell<Node<T>>>>;


pub struct Node<T: PartialOrd> {
    is_red: bool,
    root: T,
    left: BTreeOpt<T>,
    right: BTreeOpt<T>,
    parent: BTreeOpt<T>,
}

impl<T: PartialOrd + fmt::Display> fmt::Debug for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Node {{ is_red: {}, root: {},\nleft: {:#?},\nright: {:#?} }}",
                self.is_red, self.root, self.left, self.right)
    }
}

fn empty_tree<T: PartialOrd>() -> BTreeOpt<T> {
    None
}

fn new_tree<T: PartialOrd>(n: Node<T>) -> BTreeOpt<T> {
    Some(Rc::new(RefCell::new(n)))
}

fn actual_insert<T: PartialOrd>(b: &mut BTreeOpt<T>, data: T, parent: BTreeOpt<T>) {
    if let Some(ref mut thing) = b {
        if data.partial_cmp(&thing.borrow().root) == Some(Ordering::Less) {
            actual_insert(&mut thing.borrow_mut().left, data, Some(thing.clone()));
        } else {
            actual_insert(&mut thing.borrow_mut().right, data, Some(thing.clone()));
        }
    } else {
        let mut n = Node::<T> {is_red: true, root: data,
                left: empty_tree(), right: empty_tree(), parent: parent};
        *b = new_tree(n);
    }
}

pub struct Frog {
    pub prop: bool
}

fn try_recolor<T: PartialOrd>(b: &mut BTreeOpt<T>) {

}

fn fix_insert<T: PartialOrd>(b: &mut BTreeOpt<T>) {
    try_recolor(b);
}



pub fn insert<T: PartialOrd>(b: &mut BTreeOpt<T>, data: T) {

    // need to get the parent node somehow
    actual_insert(b, data, empty_tree());

    // if let Branch(ref mut node) = p {
    //     n = if data.partial_cmp(&node.root) == Some(Ordering::Less) {
    //         node.left
    //     } else {
    //         node.right
    //     };
    // } else {
    //     n = Box::new(Empty);
    // }



    //fix_insert(b);
}


// to see output run with:
// cargo test -- --nocapture

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_ctor_empty() {
        let a: BTreeOpt<i32> = empty_tree();
        println!("{:#?}", a);
    }

    #[test]
    fn test_insert_1() {
        let mut b: BTreeOpt<i32> = empty_tree();
        insert(&mut b, 43);
        println!("{:#?}", b);
    }

    #[test]
    fn test_insert_2() {
        let mut b: BTreeOpt<i32> = empty_tree();
        let mut i = 0;
        while i < 2 {
            insert(&mut b, i);
            i += 1;
        }
        println!("{:#?}", b);
    }

    #[test]
    fn test_insert_3() {
        let mut b: BTreeOpt<i32> = empty_tree();
        let mut i = 0;
        while i < 3 {
            insert(&mut b, i);
            println!("{:#?}", b);
            i += 1;
        }
    }

    #[test]
    fn test_insert_7() {
        let mut b: BTreeOpt<i32> = empty_tree();
        let mut i = 0;
        while i < 7 {
            insert(&mut b, i);
            i += 1;
        }
        println!("{:#?}", b);
    }
}

