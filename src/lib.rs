use std::cmp::PartialOrd;
use std::cmp::Ordering;
use std::rc::Rc;
use std::cell::RefCell;
use std::fmt;

// I hate this type
type BTree<T> = Rc<RefCell<Node<T>>>;
type BTreeOpt<T> = Option<BTree<T>>;


pub struct Node<T: PartialOrd> {
    is_red: bool,
    is_left: bool,
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

// returns the parent of the inserted node, or None if there is none
fn actual_insert<T: PartialOrd>(b: &mut BTreeOpt<T>, data: T, parent: BTreeOpt<T>, dir: bool) -> BTreeOpt<T> {
    if let Some(ref mut thing) = b {
        if data.partial_cmp(&thing.borrow().root) == Some(Ordering::Less) {
            actual_insert(&mut thing.borrow_mut().left, data, Some(thing.clone()), true)
        } else {
            actual_insert(&mut thing.borrow_mut().right, data, Some(thing.clone()), false)
        }
    } else {


        let n = Node::<T> {is_red: true, root: data,
                left: empty_tree(), right: empty_tree(), parent: parent, is_left: dir};
        let ret = Rc::new(RefCell::new(n));
        *b = Some(ret.clone());

        return ret.borrow().parent.clone();
    }
}

// returns true if tree cannot be recolored
fn color<T: PartialOrd>(parent: &mut Node<T>, uncle: &BTreeOpt<T>) -> bool {
    println!("try color");
    if let Some(u_node) = uncle {
        let mut mu_node = u_node.borrow_mut();
        if mu_node.is_red {
            println!("about to color");
            mu_node.is_red = false;
            parent.is_red = false;
            return false;
        }
    }
    return true;
}

// returns sib of n
fn sib<T: PartialOrd>(n: &Node<T>) -> BTreeOpt<T> {
    if let Some(ref thing) = n.parent {
        if n.is_left {
            if let Some(ref thing2) = thing.borrow().right {
                Some(thing2.clone())
            } else {
                None
            }
        } else {
            if let Some(ref thing2) = thing.borrow().left {
                Some(thing2.clone())
            } else {
                None
            }
        }
    } else {
        None
    }
}

// call this function if parent and one of its children are both red
fn recolor<T: PartialOrd>(parent: &mut Node<T>) {
    println!("parent is red");
        
    let uncle = sib(parent);
    // color parent and uncle black
    if color(parent, &uncle) {
        println!("cannot recolor");
        return;
    }

    if let Some(ref gp_node) = &(*parent).parent {
        //color grandparent red
        gp_node.borrow_mut().is_red = true;

        // recolor up the tree if the grandparent has a parent
        if let Some(ref mut ggp_node) = gp_node.borrow_mut().parent {
            recolor(&mut ggp_node.borrow_mut());
        }
    }
}


pub fn insert<T: PartialOrd>(b: &mut BTreeOpt<T>, data: T) {


    let p = actual_insert(b, data, empty_tree(), true);
    //let parent = &mut (*ins).borrow_mut().parent;

    if let Some(ref node) = p { // prob err cant borrow
        let mut m_node = node.borrow_mut();
        if m_node.is_red {
            println!("recoloring...");
            recolor(&mut m_node);
        }
    };
 
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

    #[test]
    fn test_recolor() {
        let mut b: BTreeOpt<i32> = empty_tree();
        let mut arr = [15, 5, 20, 10];
        let mut idx = 0;
        while idx < arr.len() {
            insert(&mut b, arr[idx]);
            idx += 1;
        }
        println!("{:#?}", b);
    }

}

