use std::cmp::PartialOrd;
use std::cmp::Ordering;
use std::vec::Vec;
use std::fmt;

// Inspired by the doubly linked list implementation 
// found at http://bluss.github.io/ixlist/target/doc/src/ixlist/lib.rs.html
// on 2019-03-20.

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
	idx: usize,
	sibs: Vec<usize>
}

impl<T: fmt::Debug> fmt::Debug for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "val: {:#?}, idx: {}, color: {:#?}, parent: {}, left: {}, right: {}",
                self.val, self.idx, self.color, self.sibs[0] as isize, self.sibs[1] as isize,
                self.sibs[2] as isize)
    }
}

#[derive(Debug)]
pub struct BTree<T: PartialOrd> {
    // This be a Adj List
    nodes: Vec<Node<T>>,
    root_idx: usize
}

impl<T: PartialOrd> BTree<T> {

    pub fn new() -> Self {
        BTree { nodes: Vec::<Node<T>>::new(), root_idx: 0 }
    }

    // parent node must exist
    fn btree_sib(nodes: &Vec<Node<T>>, idx: usize) -> usize {
        let par_idx = nodes[idx].sibs[PARENT];
        //if par_idx == EMPTY { return EMPTY; }

        // uncle will be left if parent was right, and vice versa
        if nodes[idx].val.partial_cmp(&nodes[par_idx].val)
                == Some(Ordering::Less) {
            nodes[par_idx].sibs[RIGHT]
        } else {
            nodes[par_idx].sibs[LEFT]
        }
    }

    // parent_idx must be in [0, nodes.len())
    fn recolor(nodes: &mut Vec<Node<T>>, parent_idx: usize) -> usize {
        // if both parent and uncle are red, recolor
        // else, cannot recolor
        if nodes[parent_idx].color != COLORS::RED { return parent_idx; }

        let g_par_idx = nodes[parent_idx].sibs[PARENT];
        if g_par_idx == EMPTY { return parent_idx; }

        let uncle_idx = BTree::btree_sib(nodes, parent_idx);
        if uncle_idx == EMPTY { return parent_idx; }

        // the actual recoloring
        if nodes[uncle_idx].color == COLORS::RED {
            nodes[uncle_idx].color = COLORS::BLACK;
            nodes[g_par_idx].color = COLORS::RED;
            nodes[parent_idx].color = COLORS::BLACK;

            // let gg_par_idx = nodes[g_par_idx].sibs[PARENT];
            // if gg_par_idx == EMPTY { return; }

            return BTree::recolor(nodes, g_par_idx);
        } else {
            return parent_idx;
        }
    }

    fn link_par2child(nodes: &mut Vec<Node<T>>, child_idx: usize) {
        let p = nodes[child_idx].sibs[PARENT];
        if p != EMPTY {
            if nodes[child_idx].val.partial_cmp(&nodes[p].val) == Some(Ordering::Less) {
                nodes[p].sibs[LEFT] = child_idx;
            } else {
                nodes[p].sibs[RIGHT] = child_idx;
            }
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

        b.nodes[left_idx].sibs[PARENT] = b.nodes[idx].sibs[PARENT];
        b.nodes[idx].sibs[PARENT] = left_idx;
        b.nodes[left_idx].sibs[RIGHT] = idx;

        BTree::link_par2child(&mut b.nodes, left_idx);

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

        b.nodes[right_idx].sibs[PARENT] = b.nodes[idx].sibs[PARENT];
        b.nodes[idx].sibs[PARENT] = right_idx;
        b.nodes[right_idx].sibs[LEFT] = idx;

        BTree::link_par2child(&mut b.nodes, right_idx);

        // set the root if it got shifted
        if idx == b.root_idx {
            b.root_idx = right_idx;
        }
    }

    fn adjust_subtrees(b: &mut BTree<T>, parent_idx: usize, child_idx: usize) {
        let g_par_idx = b.nodes[parent_idx].sibs[PARENT];
        if b.nodes[parent_idx].sibs[LEFT] == child_idx {
            if b.nodes[g_par_idx].sibs[RIGHT] == parent_idx {
                // left of parent, right of grandparnt
                BTree::right_rotate(b, parent_idx);
            }
            // is now left of parent, left of grandparnt
            BTree::right_rotate(b, g_par_idx);
        } else {
            if b.nodes[g_par_idx].sibs[LEFT] == parent_idx {
                // right of parent, left of grandparnt
                BTree::left_rotate(b, parent_idx);
            }
            // is now right of parent, right of grandparnt
            BTree::left_rotate(b, g_par_idx);
        }

        // g_par_idx is now an uncle or sibling
        b.nodes[g_par_idx].color = COLORS::RED;
        let new_g_par_idx = b.nodes[g_par_idx].sibs[PARENT];
        b.nodes[new_g_par_idx].color = COLORS::BLACK;
    }

    fn balence(b: &mut BTree<T>, new_idx: usize) {
        let mut parent_idx = b.nodes[new_idx].sibs[PARENT];
        if parent_idx != EMPTY {
            parent_idx = BTree::recolor(&mut b.nodes, parent_idx);

            // check if there's a grandparent
            if b.nodes[parent_idx].sibs[PARENT] != EMPTY {
                let uncle_idx = BTree::btree_sib(&b.nodes, parent_idx);

                // empty nodes count as black nodes
                if (uncle_idx == EMPTY || b.nodes[uncle_idx].color == COLORS::BLACK)
                        && b.nodes[parent_idx].color == COLORS::RED {
                    BTree::adjust_subtrees(b, parent_idx, new_idx);
                }
            }
        }

        // the root shall always be black
        b.nodes[b.root_idx].color = COLORS::BLACK;
    }

    fn add2list(nodes: &mut Vec<Node<T>>, val: T, neighbors: Vec<usize>) {
        let n = Node { val: val, color: COLORS::RED, idx: nodes.len(), sibs: neighbors};
        nodes.push(n);
    }

    fn parent_idx(b: &BTree<T>, val: &T) -> usize {
        let mut node = &b.nodes[b.root_idx];
        let mut idx = b.root_idx;

        while idx != (EMPTY) {
            node = &b.nodes[idx];
            if val.partial_cmp(&node.val) == Some(Ordering::Less) {
                idx = node.sibs[LEFT];
            } else {
                idx = node.sibs[RIGHT];
            }
        }

        node.idx
    }

    pub fn insert(&mut self, val: T) {
        // new elements are appended to the end of the list
        let new_idx = self.nodes.len();

        let sibs = if new_idx > 0 {
            let idx = BTree::parent_idx(&self, &val);
            let mut new_sibs = vec![EMPTY; 3];
            let node = &mut self.nodes[idx];

            new_sibs[PARENT] = idx;

            if val.partial_cmp(&node.val) == Some(Ordering::Less) {
                node.sibs[LEFT] = new_idx;
            } else {
                node.sibs[RIGHT] = new_idx;
            }

            new_sibs
        } else {
            vec![EMPTY; 3]
        };
        
        BTree::add2list(&mut self.nodes, val, sibs);
        BTree::balence(self, new_idx);
    }
}

// to see output run with:
// cargo test -- --nocapture

#[cfg(test)]
mod test {
    use crate::*;

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

    fn assert_rbtree<T: PartialOrd>(b: &BTree<T>) {
        assert_colors::<T>(&b.nodes, b.root_idx);
        assert_black_count::<T>(&b.nodes, b.root_idx);
    }

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
        assert_rbtree::<i32>(&b);
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
        assert_rbtree::<i32>(&b);
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
        assert_rbtree::<i32>(&b);
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
        assert_rbtree::<i32>(&b);
    }

    #[test]
    fn test_fix_left_3() {
        let mut b = new_tree::<i32>();
        let mut i = 3;
        while i > 0 {
            b.insert(i);
            i -= 1;
        }
        println!("{:#?}", b);
        assert_rbtree::<i32>(&b);
    }

    #[test]
    fn test_fix_left_5() {
        let mut b = new_tree::<i32>();
        let mut i = 5;
        while i > 0 {
            b.insert(i);
            i -= 1;
        }
        println!("{:#?}", b);
        assert_rbtree::<i32>(&b);
    }

        #[test]
    fn test_fix_right_5() {
        let mut b = new_tree::<i32>();
        let mut i = 0;
        while i < 5 {
            b.insert(i);
            i += 1;
        }
        println!("{:#?}", b);
        assert_rbtree::<i32>(&b);
    }

    #[test]
    fn test_recolor() {
        let mut b = new_tree::<i32>();
        let arr = [15, 5, 20, 10];
        let mut idx = 0;
        while idx < arr.len() {
            b.insert(arr[idx]);
            idx += 1;
        }
        println!("{:#?}", b);
        assert_rbtree::<i32>(&b);
    }

    #[test]
    fn test_8() {
        let mut b = new_tree::<i32>();
        let mut i = 0;
        while i < 8 {
            b.insert(i * 7 + (-i % 2) * 13);
            i += 1;
            println!("{:#?}", b);
            assert_rbtree::<i32>(&b);
        }
    }

    #[test]
    fn test_10() {
        let mut b = new_tree::<i32>();
        let mut i = 0;
        while i < 10 {
            b.insert(i * 7 + (-i % 2) * 13);
            i += 1;
        }
        println!("{:#?}", b);
        assert_rbtree::<i32>(&b);
    }

    #[test]
    fn test_20() {
        let mut b = new_tree::<i32>();
        let mut i = 0;
        while i < 20 {
            b.insert(i * 7 + (-i % 2) * 13);
            i += 1;
        }
        println!("{:#?}", b);
        assert_rbtree::<i32>(&b);
    }

}

