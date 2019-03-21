use std::cmp::PartialOrd;
use std::cmp::Ordering;
use std::vec::Vec;
use std::fmt;

// Inspired by the doubly linked list implementation 
// found at http://bluss.github.io/ixlist/target/doc/src/ixlist/lib.rs.html
// on 2019-03-20.

enum INDEXES {
    PARENT = 0,
    LEFT = 1,
    RIGHT = 2,
    // isize and usize have the same sizes, so -1 will be interpreted as std::usize::MAX
    EMPTY = -1
}

struct Node<T> {
	val: T,
	idx: usize,
	sibs: Vec<usize>
}

impl<T: fmt::Debug> fmt::Debug for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "val: {:?}, idx: {}, parent: {}, left: {}, right: {}",
                self.val, self.idx, self.sibs[0] as isize, self.sibs[1] as isize,
                self.sibs[2] as isize)
    }
}

#[derive(Debug)]
struct BTree<T: PartialOrd> {
    // Root of the tree shall be element 0, once it exists.
    // This be a Adj List
    // siblings are stored in the order added
    nodes: Vec<Node<T>>
}

impl<T: PartialOrd> BTree<T> {

    pub fn new() -> Self {
        BTree { nodes: Vec::<Node<T>>::new() }
    }

    fn add(nodes: &mut Vec<Node<T>>, val: T, neighbors: Vec<usize>) {
        let n = Node { val: val, idx: nodes.len(), sibs: neighbors};
        nodes.push(n);
    }

    fn parent_idx(nodes: &Vec<Node<T>>, val: &T) -> usize {
        let mut node = &nodes[0];
        let mut idx = 0;

        while idx != (INDEXES::EMPTY as usize) {
            node = &nodes[idx];
            if val.partial_cmp(&node.val) == Some(Ordering::Less) {
                idx = node.sibs[INDEXES::LEFT as usize];
            } else {
                idx = node.sibs[INDEXES::RIGHT as usize];
            }
        }

        node.idx
    }

    pub fn insert(&mut self, val: T) {
        let sibs = if self.nodes.len() > 0 {
            let idx = BTree::parent_idx(&self.nodes, &val);
            let mut new_sibs = vec![INDEXES::EMPTY as usize; 3];
            let new_idx = self.nodes.len();
            let node = &mut self.nodes[idx];

            new_sibs[INDEXES::PARENT as usize] = idx;

            if val.partial_cmp(&node.val) == Some(Ordering::Less) {
                node.sibs[INDEXES::LEFT as usize] = new_idx;
            } else {
                node.sibs[INDEXES::RIGHT as usize] = new_idx;
            }

            new_sibs
        } else {
            vec![INDEXES::EMPTY as usize; 3]
        };
        
        BTree::add(&mut self.nodes, val, sibs);
    }
}

// to see output run with:
// cargo test -- --nocapture

#[cfg(test)]
mod test {
    use crate::*;

    fn new_tree<T: PartialOrd>() -> BTree<T> {
        BTree::new()
    }

    #[test]
    fn test_ctor_empty() {
        let a = new_tree::<i32>();
        println!("{:#?}", a);
    }

    #[test]
    fn test_insert_1() {
        let mut b = new_tree::<i32>();
        b.insert(43);
        println!("{:#?}", b);
    }

    #[test]
    fn test_insert_2() {
        let mut b = new_tree::<i32>();
        let mut i = 0;
        while i < 2 {
            b.insert(i);
            i += 1;
        }
        println!("{:#?}", b);
    }

    #[test]
    fn test_insert_3() {
        let mut b = new_tree::<i32>();
        let mut i = 0;
        while i < 3 {
            b.insert(i);
            println!("{:#?}", b);
            i += 1;
        }
    }

    #[test]
    fn test_insert_7() {
        let mut b = new_tree::<i32>();
        let mut i = 0;
        while i < 7 {
            b.insert(i);
            i += 1;
        }
        println!("{:#?}", b);
    }

    // #[test]
    // fn test_recolor() {
    //     let mut b = new_tree<i32>();
    //     let mut arr = [15, 5, 20, 10];
    //     let mut idx = 0;
    //     while idx < arr.len() {
    //         b.insert(arr[idx]);
    //         idx += 1;
    //     }
    //     println!("{:#?}", b);
    // }

    // #[test]
    // fn test_right_rotate() {
    //     let mut b = new_tree<i32>();
    //     let mut arr = [15, 5, 20, 10];
    //     let mut idx = 0;
    //     while idx < arr.len() {
    //         b.insert(arr[idx]);
    //         idx += 1;
    //     }
    //     println!("before right rotate {:#?}", b);
    //     if let Some(ref mut thing) = b {
    //         right_rotate(thing);
    //     }
    //     println!("after right rotate {:#?}", b);

    // }

}

