use std::cmp::PartialOrd;
use std::vec::Vec;
use std::fmt;


// Inspired by the doubly linked list implementation 
// found at http://bluss.github.io/ixlist/target/doc/src/ixlist/lib.rs.html
// on 2019-03-20.

const DEBUG: bool = false;

const PARENT: usize = 0;
const LEFT: usize = 1;
const RIGHT: usize = 2;

const EMPTY: usize = std::usize::MAX;

#[derive(Debug, PartialEq)]
enum COLORS {
    RED,
    BLACK
}

struct Node<T> {
	val: T,
    color: COLORS,
	sibs: [usize; 3]
}

impl<T: fmt::Debug> fmt::Debug for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "val: {:#?}, color: {:#?}, parent: {}, left: {}, right: {}",
                self.val, self.color, self.sibs[0] as isize, self.sibs[1] as isize,
                self.sibs[2] as isize)
    }
}

// A red-black tree represented with an adjacency list
#[derive(Debug)]
pub struct BTree<T: PartialOrd> {
    nodes: Vec<Node<T>>,
    root_idx: usize
}

fn assert_colors<T: PartialOrd>(nodes: &Vec<Node<T>>, root_idx: usize) {
    let left_idx = nodes[root_idx].sibs[LEFT];
    let right_idx = nodes[root_idx].sibs[RIGHT];

    if nodes[root_idx].color == COLORS::RED {
        assert!(left_idx == EMPTY || nodes[left_idx].color == COLORS::BLACK,
                "{} is a red left child of red node {}", left_idx, root_idx);
                
        assert!(right_idx == EMPTY || nodes[right_idx].color == COLORS::BLACK,
                "{} is a red right child of red node {}", right_idx, root_idx);
    }

    if left_idx != EMPTY {
        assert_colors(&nodes, left_idx);
    }
    if right_idx != EMPTY {
        assert_colors(&nodes, right_idx);
    }
}

fn assert_black_count<T: PartialOrd>(nodes: &Vec<Node<T>>, root_idx: usize) -> usize {
    let left_idx = nodes[root_idx].sibs[LEFT];
    let right_idx = nodes[root_idx].sibs[RIGHT];

    let count = if nodes[root_idx].color == COLORS::BLACK {
        1
    } else {
        0
    };

    let left = if left_idx != EMPTY {
            assert_black_count(&nodes, left_idx)
    } else {
        1
    };

    let right = if right_idx != EMPTY {
        assert_black_count(&nodes, right_idx)
    } else {
        1
    };

    assert!(left == right, "root_idx: {}, black node counts {{right: {}, left: {}}}", 
            root_idx, right, left);
    return count + left;
}

fn assert_is_rbtree<T: PartialOrd + fmt::Debug>(b: &BTree<T>) -> bool {
    if DEBUG { println!("checking: {:#?}", b); }
    assert_colors::<T>(&b.nodes, b.root_idx);
    assert_black_count::<T>(&b.nodes, b.root_idx);
    // this will only execute if the above tests pass
    return true;
}

impl<T: PartialOrd + fmt::Debug> BTree<T> {

    pub fn new() -> Self {
        BTree { nodes: Vec::<Node<T>>::new(), root_idx: 0 }
    }

    // parent node must exist
    fn btree_sib(nodes: &Vec<Node<T>>, idx: usize) -> usize {
        let par_idx = nodes[idx].sibs[PARENT];

        // uncle will be left if parent was right, and vice versa
        if nodes[par_idx].sibs[RIGHT] == idx {
            nodes[par_idx].sibs[LEFT]
        } else {
            nodes[par_idx].sibs[RIGHT]
        }
    }

    fn is_black(n: &Vec<Node<T>>, idx: usize) -> bool {
        // empty nodes count as black nodes
        idx == EMPTY || n[idx].color == COLORS::BLACK
    }

    // this function assumes the node at new_idx is red
    // new_idx must be in [0, nodes.len()), e.g. not EMPTY
    // this function will never return EMPTY
    fn recolor(nodes: &mut Vec<Node<T>>, mut new_idx: usize) -> usize {
        // if both parent and uncle are red, recolor
        // else, cannot recolor
        
        let mut parent_idx = nodes[new_idx].sibs[PARENT];

        // must have a grandparent to have an uncle
        while parent_idx != EMPTY && nodes[parent_idx].sibs[PARENT] != EMPTY
                && nodes[parent_idx].color == COLORS::RED {
            
            let g_par_idx = nodes[parent_idx].sibs[PARENT];
            let uncle_idx = BTree::btree_sib(nodes, parent_idx);
            // if uncle is black, cannot recolor
            if BTree::is_black(nodes, uncle_idx) { break; }

            nodes[uncle_idx].color = COLORS::BLACK;
            nodes[g_par_idx].color = COLORS::RED;
            nodes[parent_idx].color = COLORS::BLACK;

            new_idx = g_par_idx;
            parent_idx = nodes[new_idx].sibs[PARENT];
        }

        // return however far this function was able to go.
        return new_idx;
    }

    // Links the parent node with the new child. Nothing is done with the old child's link.
    fn replace_child(nodes: &mut Vec<Node<T>>, old_child: usize, new_child: usize) {
        let p = nodes[old_child].sibs[PARENT];
        if p != EMPTY {
            if nodes[p].sibs[LEFT] == old_child {
                nodes[p].sibs[LEFT] = new_child;
            } else {
                nodes[p].sibs[RIGHT] = new_child;
            }
        }

        if new_child != EMPTY {
            nodes[new_child].sibs[PARENT] = p;
        }
    }

    // there must be a left node
    fn right_rotate(b: &mut BTree<T>, idx: usize) {
        let left_idx = b.nodes[idx].sibs[LEFT];

        let right_of_left_idx = b.nodes[left_idx].sibs[RIGHT];
        if right_of_left_idx != EMPTY {
            b.nodes[right_of_left_idx].sibs[PARENT] = idx;
        }
        b.nodes[idx].sibs[LEFT] = right_of_left_idx;

        BTree::replace_child(&mut b.nodes, idx, left_idx);

        b.nodes[idx].sibs[PARENT] = left_idx;
        b.nodes[left_idx].sibs[RIGHT] = idx;

        // set the root if it got shifted
        if idx == b.root_idx {
            b.root_idx = left_idx;
        }
    }

    // there must be a right node
    fn left_rotate(b: &mut BTree<T>, idx: usize) {
        let right_idx = b.nodes[idx].sibs[RIGHT];

        let left_of_right_idx = b.nodes[right_idx].sibs[LEFT];
        if left_of_right_idx != EMPTY {
            b.nodes[left_of_right_idx].sibs[PARENT] = idx;
        }
        b.nodes[idx].sibs[RIGHT] = left_of_right_idx;

        BTree::replace_child(&mut b.nodes, idx, right_idx);

        b.nodes[idx].sibs[PARENT] = right_idx;
        b.nodes[right_idx].sibs[LEFT] = idx;

        // set the root if it got shifted
        if idx == b.root_idx {
            b.root_idx = right_idx;
        }
    }

    fn adjust_subtrees(b: &mut BTree<T>, g_par_idx: usize,
            parent_idx: usize, child_idx: usize) {
        
        if b.nodes[parent_idx].sibs[LEFT] == child_idx {
            if b.nodes[g_par_idx].sibs[RIGHT] == parent_idx {
                // left of parent, right of grandparnt
                BTree::right_rotate(b, parent_idx);
                // now right of parent and right of grandparent
                BTree::left_rotate(b, g_par_idx);
            } else {
                BTree::right_rotate(b, g_par_idx);
            }
        } else {
            if b.nodes[g_par_idx].sibs[LEFT] == parent_idx {
                // right of parent, left of grandparnt
                BTree::left_rotate(b, parent_idx);
                // now left of parent and left of grandparent
                BTree::right_rotate(b, g_par_idx);
            } else {
                BTree::left_rotate(b, g_par_idx);
            }
        }

        // g_par_idx is now an uncle or sibling
        b.nodes[g_par_idx].color = COLORS::RED;
        let new_g_par_idx = b.nodes[g_par_idx].sibs[PARENT];
        b.nodes[new_g_par_idx].color = COLORS::BLACK;
    }

    fn balence(b: &mut BTree<T>, mut new_idx: usize) {
        new_idx = BTree::recolor(&mut b.nodes, new_idx);
        // make sure the first node inserted is black
        b.nodes[b.root_idx].color = COLORS::BLACK;

        // if black, no need to adjust the tree
        if BTree::is_black(&b.nodes, new_idx) { return; }

        let parent_idx = b.nodes[new_idx].sibs[PARENT];
        // if parent is black, no red-red path, so don't adjust the tree
        if BTree::is_black(&b.nodes, parent_idx) { return; }

        // need a grandparent to have an uncle
        let g_par_idx = b.nodes[parent_idx].sibs[PARENT];
        if g_par_idx == EMPTY { return; }

        let uncle_idx = BTree::btree_sib(&b.nodes, parent_idx);
        
        if BTree::is_black(&b.nodes, uncle_idx) {
            BTree::adjust_subtrees(b, g_par_idx, parent_idx, new_idx);
        }
    }

    // finds idx of val, or parent of val if val not in tree
    fn find(b: &BTree<T>, val: &T) -> usize {
        let mut idx = b.root_idx;
        let mut idx_ret;

         loop {
            idx_ret = idx;
            let node = &b.nodes[idx];

            idx = if val.lt(&node.val) {
                node.sibs[LEFT]
            } else {
                node.sibs[RIGHT]
            };

            if idx == EMPTY { break; }
        }

        idx_ret
    }

    // makes sure a node's children link to it
    fn link_with_children(nodes: &mut Vec<Node<T>>, idx: usize) {
        let left_idx = nodes[idx].sibs[LEFT];
        if left_idx != EMPTY {
            nodes[left_idx].sibs[PARENT] = idx;
        }
    
        let right_idx = nodes[idx].sibs[RIGHT];
        if right_idx != EMPTY {
            nodes[right_idx].sibs[PARENT] = idx;
        }
    }

    fn swap(nodes: &mut Vec<Node<T>>, i: usize, j: usize) {
        let holder = nodes[j].sibs;
        nodes[j].sibs = nodes[i].sibs;
        nodes[i].sibs = holder;

        BTree::link_with_children(nodes, i);
        BTree::link_with_children(nodes, j);

        // links i to j's parent & vice versa
        BTree::replace_child(nodes, i, j);
        BTree::replace_child(nodes, j, i);
    }

    fn min_in_subtree(nodes: &Vec<Node<T>>, mut idx: usize) -> usize {
        while nodes[idx].sibs[LEFT] != EMPTY {
            idx = nodes[idx].sibs[LEFT];
        }
        idx
    }

    // returns a tuple containing the index where readjustment may be required
    // and the value requested.
    fn bst_remove(b: &mut BTree<T>, key: T) -> (usize, T) {
        let idx = BTree::find(&b, &key);
        let shift;
        
        let replacement = if b.nodes[idx].sibs[RIGHT] == EMPTY {
            shift = b.nodes[idx].sibs[LEFT];
            shift
        } else if b.nodes[idx].sibs[LEFT] == EMPTY {
            shift = b.nodes[idx].sibs[RIGHT];
            shift
        } else {
            // has two children, must find replacement
            let min = BTree::min_in_subtree(&b.nodes, b.nodes[idx].sibs[RIGHT]);
            if min != idx {
                BTree::swap(&mut b.nodes, idx, min);
            }
            // need to shift up the old min's child into it's place
            shift = b.nodes[idx].sibs[RIGHT];
            
            min
        };

        // shift the proper child up into idx's place
        BTree::replace_child(&mut b.nodes, idx, shift);

        // adjust root, if needed
        if idx == b.root_idx {
            b.root_idx = replacement;
        }
        
        // remove idx from the list and replace it with whatever node is at the end of the list
        let n = b.nodes.swap_remove(idx);
        // idx's replacement now has and index of idx. Update sibs to reflect this.
        BTree::link_with_children(&mut b.nodes, idx);
        BTree::replace_child(&mut b.nodes, idx, idx);
        
        return (shift, n.val);
    }

    pub fn insert(&mut self, val: T) {
        // new elements are appended to the end of the list
        let new_idx = self.nodes.len();

        let sibs = if new_idx > 0 {
            // this will return the parent of where val should go
            let idx = BTree::find(&self, &val);
            let mut new_sibs = [EMPTY; 3];
            let node = &mut self.nodes[idx];

            new_sibs[PARENT] = idx;

            if val.lt(&node.val) {
                node.sibs[LEFT] = new_idx;
            } else {
                node.sibs[RIGHT] = new_idx;
            }

            new_sibs
        } else {
            [EMPTY; 3]
        };
        
        let n = Node {
            val: val,
            color: COLORS::RED,
            sibs: sibs
        };
        self.nodes.push(n);
        BTree::balence(self, new_idx);
        debug_assert!(assert_is_rbtree(&self));
    }

    // key must be in tree
    pub fn remove(&mut self, key: T) -> T {
        let idx = BTree::find(&self, &key);
        let replacement = BTree::get_replacement(&self.nodes, idx);
        let p = self.nodes[idx].sibs[PARENT];

        if replacement == EMPTY {
            
        } else {
            // only need to shift up one
            if self.nodes[replacement].sibs[PARENT] == idx {
                if p != EMPTY {
                    if self.nodes[p].sibs[LEFT] == idx {
                        self.nodes[p].sibs[LEFT] = replacement;
                    } else {
                        self.nodes[p].sibs[RIGHT] = replacement;
                    }
                }
                self.nodes[replacement].sibs[PARENT] = p;
            } else {
                //BTree::replace_neighbors(&mut self.nodes, idx, replacement);
            }
        }

        

        // don't remove, because it will shift indexes
        // TODO: swap removed position with last positoin in vector
        // then delete last position in vector
        return key; //self.nodes[idx].val;
    }

    pub fn size(&self) -> usize{
        self.nodes.len()
    }
}


// to see output run with: cargo test -- --nocapture
#[cfg(test)]
mod test {
    use std::io::prelude::*;
    use std::io::Result;
    use std::io::BufReader;
    use std::fs::File;
    use crate::*;

    fn new_tree<T: PartialOrd + fmt::Debug>() -> BTree<T> {
        BTree::new()
    }

    fn from_file(name: &str) -> Result<()> {
        let mut b = new_tree::<char>();
        let mut line = String::new();
        let f = File::open(name)?;
        let mut buf = BufReader::new(f);

        buf.read_line(&mut line)?;
        // line will be empty at the end of the file
        while b.size() < 150 && line.len() > 0 {
            while line.len() > 0 {
                if let Some(ch) = line.pop() {
                    if DEBUG { println!("inserting: {} to: {:#?}", ch, b); }
                    b.insert(ch);
                }
            }
            buf.read_line(&mut line)?;
        }
        assert_is_rbtree::<char>(&b);

        Ok(())
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
        assert_is_rbtree::<i32>(&b);
    }

    #[test]
    fn test_insert_2() {
        let mut b = new_tree::<i32>();
        let mut i = 0;
        while i < 2 {
            b.insert(i);
            i += 1;
        }
        assert_is_rbtree::<i32>(&b);
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
        assert_is_rbtree::<i32>(&b);
    }

    #[test]
    fn test_insert_7() {
        let mut b = new_tree::<i32>();
        let mut i = 0;
        while i < 7 {
            b.insert(i);
            i += 1;
        }
        assert_is_rbtree::<i32>(&b);
    }

    #[test]
    fn test_fix_left_3() {
        let mut b = new_tree::<i32>();
        let mut i = 3;
        while i > 0 {
            b.insert(i);
            i -= 1;
        }
        assert_is_rbtree::<i32>(&b);
    }

    #[test]
    fn test_fix_left_5() {
        let mut b = new_tree::<i32>();
        let mut i = 5;
        while i > 0 {
            b.insert(i);
            i -= 1;
        }
        assert_is_rbtree::<i32>(&b);
    }

    #[test]
    fn test_fix_right_5() {
        let mut b = new_tree::<i32>();
        let mut i = 0;
        while i < 5 {
            b.insert(i);
            i += 1;
        }
        assert_is_rbtree::<i32>(&b);
    }

    #[test]
    fn test_double_red_1() {
        let mut b = new_tree::<i32>();
        let arr = [15, 5, 20, 0];
        let mut idx = 0;
        while idx < arr.len() {
            b.insert(arr[idx]);
            idx += 1;
        }
        assert_is_rbtree::<i32>(&b);
    }

    #[test]
    fn test_double_red_2() {
        let mut b = new_tree::<i32>();
        let arr = [15, 5, 20, 10];
        let mut idx = 0;
        while idx < arr.len() {
            b.insert(arr[idx]);
            idx += 1;
        }
        assert_is_rbtree::<i32>(&b);
    }

    #[test]
    fn test_double_red_3() {
        let mut b = new_tree::<i32>();
        let arr = [15, 5, 20, 17];
        let mut idx = 0;
        while idx < arr.len() {
            b.insert(arr[idx]);
            idx += 1;
        }
        assert_is_rbtree::<i32>(&b);
    }

    #[test]
    fn test_double_red_4() {
        let mut b = new_tree::<i32>();
        let arr = [15, 5, 20, 30];
        let mut idx = 0;
        while idx < arr.len() {
            b.insert(arr[idx]);
            idx += 1;
        }
        assert_is_rbtree::<i32>(&b);
    }

    #[test]
    fn test_8() {
        let mut b = new_tree::<i32>();
        let mut i = 0;
        while i < 8 {
            b.insert(i * 7 + (-i % 2) * 13);
            i += 1;
            println!("{:#?}", b);
            assert_is_rbtree::<i32>(&b);
        }
    }

    #[test]
    fn test_20() {
        let mut b = new_tree::<i32>();
        let mut i = 0;
        while i < 20 {
            b.insert(i * 7 + (-i % 2) * 13);
            i += 1;
        }
        assert_is_rbtree::<i32>(&b);
    }

    #[test]
    fn test_big() -> Result<()> {
        from_file(".gitignore")
    }

    #[test]
    fn test_bigger() -> Result<()> {
        from_file("src/lib.rs")
    }

}
