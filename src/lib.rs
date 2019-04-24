use std::cmp::PartialOrd;
use std::vec::Vec;
use std::fmt;


// Inspired by the doubly linked list implementation 
// found at http://bluss.github.io/ixlist/target/doc/src/ixlist/lib.rs.html
// on 2019-03-20.

const DEBUG: bool = true;

const EMPTY: usize = std::usize::MAX;

const RED: bool = true;
const BLACK: bool = false;

struct Node<T> {
	val: T,
    color: bool,
	parent: usize,
    left: usize,
    right: usize
}

struct RemovalData<T> {
    parent: usize,
    shifted: usize,
    color: bool,
    val: T
}

impl<T: fmt::Debug> fmt::Debug for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), std::fmt::Error> {
        let color = if self.color == RED {
            "RED"
        } else {
            "BLACK"
        };
        write!(f, "val: {:#?}, color: {}, parent: {}, left: {}, right: {}",
                self.val, color, self.parent as isize, self.left as isize,
                self.right as isize)
    }
}

// A red-black tree represented with an adjacency list
#[derive(Debug)]
pub struct BTree<T: PartialOrd> {
    nodes: Vec<Node<T>>,
    root_idx: usize
}

fn assert_colors<T: PartialOrd>(nodes: &Vec<Node<T>>, root_idx: usize) {
    let left_idx = nodes[root_idx].left;
    let right_idx = nodes[root_idx].right;

    if nodes[root_idx].color == RED {
        assert!(left_idx == EMPTY || nodes[left_idx].color == BLACK,
                "{} is a red left child of red node {}", left_idx, root_idx);
                
        assert!(right_idx == EMPTY || nodes[right_idx].color == BLACK,
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
    let count = if nodes[root_idx].color == BLACK {
        1
    } else {
        0
    };

    let left_idx = nodes[root_idx].left;
    let left = if left_idx != EMPTY {
        assert_black_count(&nodes, left_idx)
    } else {
        1
    };

    let right_idx = nodes[root_idx].right;
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

fn assert_is_bst<T: PartialOrd + fmt::Debug>(nodes: &Vec<Node<T>>, idx: usize) -> bool {
    let left_idx = nodes[idx].left;

    if left_idx != EMPTY {
        assert!(nodes[idx].val >= nodes[left_idx].val,
            "({}, {:#?}): , has left child ({}, {:#?})",
            idx, nodes[idx].val, left_idx, nodes[left_idx].val);
        assert_is_bst(nodes, left_idx);
    }

    let right_idx = nodes[idx].right;
    if right_idx != EMPTY {
        assert!(nodes[idx].val <= nodes[right_idx].val,
            "({}, {:#?}): , has right child ({}, {:#?})",
            idx, nodes[idx].val, right_idx, nodes[right_idx].val);
        assert_is_bst(nodes, right_idx);
    }
    return true;
}

fn assert_is_dlinked<T: PartialOrd + fmt::Debug>(nodes: &Vec<Node<T>>, idx: usize) -> bool{
    let left_idx = nodes[idx].left;

    if left_idx != EMPTY {
        assert!(nodes[left_idx].parent == idx,
            "({}, {:#?}) not linked to parent ({}, {:#?})",
            left_idx, nodes[left_idx].val, idx, nodes[idx].val);
        assert_is_dlinked(nodes, left_idx);
    }

    let right_idx = nodes[idx].right;
    if right_idx != EMPTY {
        assert!(nodes[right_idx].parent == idx,
            "({}, {:#?}) not linked to parent ({}, {:#?})",
            right_idx, nodes[right_idx].val, idx, nodes[idx].val);
        assert_is_dlinked(nodes, right_idx);
    }
    return true;
}

fn assert_all<T: PartialOrd + fmt::Debug>(b: &BTree<T>) -> bool {
    if b.root_idx != EMPTY {
        assert_is_dlinked(&b.nodes, b.root_idx);
        assert_is_bst(&b.nodes, b.root_idx);
        assert_is_rbtree(&b);
    }
    // this will only execute if the above tests pass
    return true;
}

impl<T: PartialOrd + fmt::Debug> BTree<T> {

    pub fn new() -> Self {
        BTree { nodes: Vec::<Node<T>>::new(), root_idx: 0 }
    }

    // parent node must exist
    fn btree_sib(nodes: &Vec<Node<T>>, idx: usize) -> usize {
        let par_idx = nodes[idx].parent;

        // uncle will be left if parent was right, and vice versa
        if nodes[par_idx].right == idx {
            nodes[par_idx].left
        } else {
            nodes[par_idx].right
        }
    }

    fn is_black(n: &Vec<Node<T>>, idx: usize) -> bool {
        // empty nodes count as black nodes
        idx == EMPTY || n[idx].color == BLACK
    }

    // this function assumes the node at new_idx is red
    // new_idx must be in [0, nodes.len()), e.g. not EMPTY
    // this function will never return EMPTY
    fn recolor(nodes: &mut Vec<Node<T>>, mut new_idx: usize) -> usize {
        // if both parent and uncle are red, recolor
        // else, cannot recolor
        
        let mut parent_idx = nodes[new_idx].parent;

        // must have a grandparent to have an uncle
        while parent_idx != EMPTY && nodes[parent_idx].parent != EMPTY
                && nodes[parent_idx].color == RED {
            
            let g_par_idx = nodes[parent_idx].parent;
            let uncle_idx = BTree::btree_sib(nodes, parent_idx);
            // if uncle is black, cannot recolor
            if BTree::is_black(nodes, uncle_idx) { break; }

            nodes[uncle_idx].color = BLACK;
            nodes[g_par_idx].color = RED;
            nodes[parent_idx].color = BLACK;

            new_idx = g_par_idx;
            parent_idx = nodes[new_idx].parent;
        }

        // return however far this function was able to go.
        return new_idx;
    }

    // Links the parent node with the new child. Nothing is done with the old child's link.
    fn replace_child(nodes: &mut Vec<Node<T>>, old_child: usize, new_child: usize) {
        let p = nodes[old_child].parent;
        if p != EMPTY {
            if nodes[p].left == old_child {
                nodes[p].left = new_child;
            } else {
                nodes[p].right = new_child;
            }
        }
    }

    // there must be a left node
    fn right_rotate(b: &mut BTree<T>, idx: usize) {
        let left_idx = b.nodes[idx].left;

        let right_of_left_idx = b.nodes[left_idx].right;
        if right_of_left_idx != EMPTY {
            b.nodes[right_of_left_idx].parent = idx;
        }
        b.nodes[idx].left = right_of_left_idx;

        BTree::replace_child(&mut b.nodes, idx, left_idx);
        b.nodes[left_idx].parent = b.nodes[idx].parent;

        b.nodes[idx].parent = left_idx;
        b.nodes[left_idx].right = idx;

        // set the root if it got shifted
        if idx == b.root_idx {
            b.root_idx = left_idx;
        }
    }

    // there must be a right node
    fn left_rotate(b: &mut BTree<T>, idx: usize) {
        let right_idx = b.nodes[idx].right;

        let left_of_right_idx = b.nodes[right_idx].left;
        if left_of_right_idx != EMPTY {
            b.nodes[left_of_right_idx].parent = idx;
        }
        b.nodes[idx].right = left_of_right_idx;

        BTree::replace_child(&mut b.nodes, idx, right_idx);
        b.nodes[right_idx].parent = b.nodes[idx].parent;

        b.nodes[idx].parent = right_idx;
        b.nodes[right_idx].left = idx;

        // set the root if it got shifted
        if idx == b.root_idx {
            b.root_idx = right_idx;
        }
    }

    fn adjust_subtrees(b: &mut BTree<T>, g_par_idx: usize,
            parent_idx: usize, child_idx: usize) {
        
        if b.nodes[parent_idx].left == child_idx {
            if b.nodes[g_par_idx].right == parent_idx {
                // left of parent, right of grandparnt
                BTree::right_rotate(b, parent_idx);
                // now right of parent and right of grandparent
                BTree::left_rotate(b, g_par_idx);
            } else {
                BTree::right_rotate(b, g_par_idx);
            }
        } else {
            if b.nodes[g_par_idx].left == parent_idx {
                // right of parent, left of grandparnt
                BTree::left_rotate(b, parent_idx);
                // now left of parent and left of grandparent
                BTree::right_rotate(b, g_par_idx);
            } else {
                BTree::left_rotate(b, g_par_idx);
            }
        }

        // g_par_idx is now an uncle or sibling
        b.nodes[g_par_idx].color = RED;
        let new_g_par_idx = b.nodes[g_par_idx].parent;
        b.nodes[new_g_par_idx].color = BLACK;
    }

    fn balence_insert(b: &mut BTree<T>, mut new_idx: usize) {
        new_idx = BTree::recolor(&mut b.nodes, new_idx);
        // make sure the first node inserted is black
        b.nodes[b.root_idx].color = BLACK;

        // if black, no need to adjust the tree
        if BTree::is_black(&b.nodes, new_idx) { return; }

        let parent_idx = b.nodes[new_idx].parent;
        // if parent is black, no red-red path, so don't adjust the tree
        if BTree::is_black(&b.nodes, parent_idx) { return; }

        // need a grandparent to have an uncle
        let g_par_idx = b.nodes[parent_idx].parent;
        if g_par_idx == EMPTY { return; }

        let uncle_idx = BTree::btree_sib(&b.nodes, parent_idx);
        
        if BTree::is_black(&b.nodes, uncle_idx) {
            BTree::adjust_subtrees(b, g_par_idx, parent_idx, new_idx);
        }
    }

    // finds a node that is able to have val as a child
    fn find_available_parent(b: &BTree<T>, val: &T) -> usize {
        let mut idx = b.root_idx;
        let mut idx_ret;

         loop {
            idx_ret = idx;
            let node = &b.nodes[idx];

            idx = if val.lt(&node.val) {
                node.left
            } else {
                node.right
            };

            if idx == EMPTY { break; }
        }

        idx_ret
    }

    // finds idx of val
    fn find(b: &BTree<T>, val: &T) -> usize {
        let mut idx = b.root_idx;
        let mut idx_ret;

         loop {
            idx_ret = idx;
            let node = &b.nodes[idx];

            if val.eq(&node.val) { break; }

            idx = if val.lt(&node.val) {
                node.left
            } else {
                node.right
            };
        }

        idx_ret
    }

    // makes sure a node's children link to it
    fn link_with_children(nodes: &mut Vec<Node<T>>, idx: usize) {
        let left_idx = nodes[idx].left;
        if left_idx != EMPTY {
            nodes[left_idx].parent = idx;
        }
    
        let right_idx = nodes[idx].right;
        if right_idx != EMPTY {
            nodes[right_idx].parent = idx;
        }
    }

    // shifts src up into dest's place. dest's other child must be empty
    fn shift_up(b: &mut BTree<T>, src: usize, dest: usize) -> usize {
        if src != EMPTY {
            b.nodes[src].parent = b.nodes[dest].parent;
        }
        BTree::replace_child(&mut b.nodes, dest, src);

        if b.root_idx == dest {
            b.root_idx = src;
        }

        dest
    }

    // Moves src over the top of dest, makes dest an orphan.
    // src must be a leaf
    fn overwrite(b: &mut BTree<T>, src: usize, dest: usize) -> usize {
        if DEBUG { println!("before mov: {:#?}", b.nodes); }
        if src != EMPTY {
            b.nodes[src].left = b.nodes[dest].left;
            b.nodes[src].right = b.nodes[dest].right;
            b.nodes[src].parent = b.nodes[dest].parent;
            if DEBUG { println!("during mov: {:#?}", b.nodes); }
            BTree::link_with_children(&mut b.nodes, src);
        }
        BTree::replace_child(&mut b.nodes, dest, src);

        if b.root_idx == dest {
            b.root_idx = src;
        }
        if DEBUG { println!("after mov: {:#?}", b.nodes); }
        dest
    }

    fn min_in_subtree(nodes: &Vec<Node<T>>, mut idx: usize) -> usize {
        while nodes[idx].left != EMPTY {
            idx = nodes[idx].left;
        }
        idx
    }

    // remove the node from the list, replacing its position with the last
    // member of the list
    fn remove_node(nodes: &mut Vec<Node<T>>, to_remove: usize) -> Node<T> {
        let last = nodes.len() - 1;
        if DEBUG  { println!("removing idx: {}", to_remove); }
        return if to_remove == last {
            BTree::replace_child(nodes, last, EMPTY);
            nodes.swap_remove(to_remove)
        } else {
            BTree::replace_child(nodes, last, to_remove);
            let hold_this = nodes.swap_remove(to_remove);
            BTree::link_with_children(nodes, to_remove);
            hold_this
        };
    }

    fn min_shift(b: &mut BTree<T>, idx: usize, min: usize, right_of_min: usize) -> usize {
        let parent = if b.nodes[min].parent == idx {
            min
        } else {
            b.nodes[min].parent
        };

        if parent == min { // min must be a right child for this to happen
            b.nodes[min].left = b.nodes[idx].left;
            b.nodes[min].parent = b.nodes[idx].parent;
            BTree::replace_child(&mut b.nodes, idx, min);
            BTree::link_with_children(&mut b.nodes, min);
            if b.root_idx == idx {
                b.root_idx = min;
            }
        } else {
            BTree::shift_up(b, right_of_min, min); // min is now orph
            BTree::overwrite(b, min, idx);  // idx is now orph
        }
        return parent;
    }

    fn set_r_data(nodes: &Vec<Node<T>>, idx: usize, shift: usize,
            r_data: &mut RemovalData<T>) {
        r_data.shifted = shift;
        r_data.parent = nodes[idx].parent;
        r_data.color = if BTree::is_black(nodes, shift) {
            BLACK
        } else {
            RED
        };
    }

    fn adjust_for_removal(b: &mut BTree<T>, idx: usize, r_data: &mut RemovalData<T>) {
        let last = b.nodes.len() - 1;
        if r_data.parent == last {
            r_data.parent = idx;
        }
        if r_data.shifted == last {
            r_data.shifted = idx;
        }
        if b.root_idx == last {
            b.root_idx = idx;
        }
    }

    fn bst_remove(b: &mut BTree<T>, key: T) -> RemovalData<T> {
        let idx = BTree::find(&b, &key);
        if DEBUG { println!("removing: {:#?}", key); }
        let mut r_data = RemovalData {
            parent: EMPTY,
            shifted: EMPTY,
            color: RED,
            val: key
        };

        if b.nodes[idx].right == EMPTY {
            if DEBUG { println!("right child is empty: {:#?}", b); }
            BTree::set_r_data(&b.nodes, idx, b.nodes[idx].left, &mut r_data);

            if r_data.shifted != EMPTY {
                b.nodes[r_data.shifted].color = b.nodes[idx].color;
            }
            BTree::shift_up(b, r_data.shifted, idx); // idx is now orph
        } else if b.nodes[idx].left == EMPTY {
            if DEBUG { println!("left child is empty: {:#?}", b); }
            BTree::set_r_data(&b.nodes, idx, b.nodes[idx].right, &mut r_data);

            if r_data.shifted != EMPTY {
                b.nodes[r_data.shifted].color = b.nodes[idx].color;
            }
            BTree::shift_up(b, r_data.shifted, idx); // idx is now orph
        } else {
            if DEBUG {
                println!("has 2 children");
                println!("tree: {:#?}", b);
            }
            // has two children, must find replacement
            let min = BTree::min_in_subtree(&b.nodes, b.nodes[idx].right);
            r_data.shifted = b.nodes[min].right;
            r_data.parent = BTree::min_shift(b, idx, min, r_data.shifted);
            r_data.color = b.nodes[min].color;
            b.nodes[min].color = b.nodes[idx].color;
            
            if DEBUG { println!("min: {:#?}", b.nodes[min].val); }
        };

        BTree::adjust_for_removal(b, idx, &mut r_data);

        // remove idx from the list and replace it with whatever node is at the end of the list
        if DEBUG { println!("before remove: {:#?}", b); }
        r_data.val = BTree::remove_node(&mut b.nodes, idx).val;
        if DEBUG { println!("bst_remove done: {:#?}", b); }
        if b.nodes.len() > 0 {
            debug_assert!(assert_is_bst(&b.nodes, b.root_idx));
            debug_assert!(assert_is_dlinked(&b.nodes, b.root_idx));
        }

        return r_data;
    }


    fn case_sib_is_red(b: &mut BTree<T>, idx: usize, sib: usize) {
        if b.nodes[idx].right == sib {
            BTree::left_rotate(b, idx);
        } else {
            BTree::right_rotate(b, idx);
        }
        b.nodes[idx].color = RED;
        b.nodes[sib].color = BLACK;
    }

    fn case_nephew_right_black_left_red(b: &mut BTree<T>, sib: usize) {
        let left = b.nodes[sib].left;
        b.nodes[left].color = BLACK;
        b.nodes[sib].color = RED;
        BTree::right_rotate(b, sib);
    }

    // note that the left nephew may also be red
    fn case_nephew_right_red(b: &mut BTree<T>, idx: usize, sib: usize) {
        let neph = if b.nodes[idx].right == sib {
            BTree::left_rotate(b, idx);
            b.nodes[sib].right
        } else {
            BTree::right_rotate(b, idx);
            b.nodes[sib].left
        };
        
        b.nodes[neph].color = BLACK;
        let holder = b.nodes[idx].color;
        b.nodes[idx].color = b.nodes[sib].color;
        b.nodes[sib].color = holder;
    }

    fn get_sib(nodes: &Vec<Node<T>>, idx: usize, shift: usize) -> usize {
        if nodes[idx].left == shift {
            nodes[idx].right
        } else {
            nodes[idx].left
        }
    }

    // takes parent of thing removed's black child
    fn balence_remove(b: &mut BTree<T>, mut idx: usize, mut shift: usize) {

        while idx != EMPTY {
            if DEBUG { println!("balence remove: b: {:#?}, idx: {}, shift: {}",
                    b, idx, shift); }
            let sib = BTree::get_sib(&b.nodes, idx, shift);
            // If sib is empty, then shift must also be empty,
            // thus the tree is already balenced.
            if sib == EMPTY {
                break;
            }

            if b.nodes[sib].color == RED {
                BTree::case_sib_is_red(b, idx, sib);
            } else {
                let right_nephew = b.nodes[sib].right;
                let left_nephew = b.nodes[sib].left;

                if BTree::is_black(&b.nodes, right_nephew)
                        && BTree::is_black(&b.nodes, left_nephew) {
                    if DEBUG { println!("both black"); }

                    b.nodes[sib].color = RED;
                    if b.nodes[idx].color == RED {
                        b.nodes[idx].color = BLACK;
                        break;
                    }
                    shift = idx;
                    idx = b.nodes[idx].parent;
                } else if BTree::is_black(&b.nodes, right_nephew) {
                    if DEBUG { println!("right neph black"); }
                    BTree::case_nephew_right_black_left_red(b, sib);
                } else {
                    if DEBUG { println!("right neph red"); }
                    BTree::case_nephew_right_red(b, idx, sib);
                    break;
                }
            }
        }
    }

    pub fn insert(&mut self, key: T) {
        // new elements are appended to the end of the list
        let new_idx = self.nodes.len();
        let mut n = Node {
            val: key,
            color: RED,
            parent: EMPTY,
            left: EMPTY,
            right: EMPTY
        };

        if new_idx > 0 {
            // this will return the parent of where val should go
            let idx = BTree::find_available_parent(&self, &n.val);
            let node = &mut self.nodes[idx];

            n.parent = idx;

            if n.val.lt(&node.val) {
                node.left = new_idx;
            } else {
                node.right = new_idx;
            }
        }

        self.nodes.push(n);
        BTree::balence_insert(self, new_idx);
        debug_assert!(assert_all(&self));
    }

    // key must be in tree
    pub fn remove(&mut self, key: T) -> T {
        let res = BTree::bst_remove(self, key);

        // was the node spliced out black
        if res.color == BLACK {
            if BTree::is_black(&self.nodes, res.shifted) {
                // replacement is black, must fix
                BTree::balence_remove(self, res.parent, res.shifted);
            } else { // replacement is red
                self.nodes[res.shifted].color = BLACK;
            }
        }
        
        debug_assert!(assert_all(self));

        return res.val;
    }

    pub fn size(&self) -> usize {
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

    fn from_file(name: &str) -> Result<BTree<char>> {
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

        Ok(b)
    }

    fn size(nodes: &Vec<Node<i32>>, idx: usize) -> usize {
        if idx == EMPTY {
            return 0;
        }

        let mut count = 1;

        let left_idx = nodes[idx].left;
        if left_idx != EMPTY {
            count += size(nodes, left_idx);
        }

        let right_idx = nodes[idx].right;
        if right_idx != EMPTY {
            count += size(nodes, right_idx);
        }

        return count;
    }

    #[test]
    fn test_ctor_empty() {
        let a = new_tree::<i32>();
        if DEBUG { println!("{:#?}", a); }
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
            if DEBUG { println!("{:#?}", b); }
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
            if DEBUG { println!("{:#?}", b); }
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
    fn test_big() {
        match from_file(".gitignore") {
            Ok(a) => if DEBUG { println!("{:#?}", a) },
            Err(e) => println!("{:#?}", e)
        }
    }

    #[test]
    fn test_even_bigger() {
        match from_file("src/lib.rs") {
            Ok(a) => if DEBUG { println!("{:#?}", a) },
            Err(e) => println!("{:#?}", e)
        }
    }

    #[test]
    fn test_bst_remove() {
        let mut b = new_tree::<i32>();
        let mut i = 0;
        while i < 20 {
            b.insert(i * 7 + (-i % 2) * 13);
            i += 1;
        }

        if DEBUG { println!("before bst_remove: {:#?}", b); }

        BTree::bst_remove(&mut b, 0);

        if DEBUG { println!("after bst_remove: {:#?}", b); }
        assert_is_dlinked(&b.nodes, b.root_idx);
        assert!(size(&b.nodes, b.root_idx) == 19);
        assert_is_bst(&b.nodes, b.root_idx);
    }

    #[test]
    fn test_remove_big() {
        match from_file(".gitignore") {
            Ok(mut a) => {
                while a.size() > 0 {
                    let v = a.remove(a.nodes[a.root_idx].val);
                    if DEBUG { println!("{:#?}", v); }
                }
            },
            Err(e) => println!("{:#?}", e)
        }
    }

    #[test]
    fn test_remove_3() {
        let mut b = new_tree::<i32>();
        let arr = [15, 5, 20, 17];
        let mut idx = 0;
        while idx < arr.len() {
            b.insert(arr[idx]);
            idx += 1;
        }

        while idx > 0 {
            idx -= 1;
            if DEBUG { println!("idx: {}", idx); }
            b.remove(arr[idx]);
        }
    }

    #[test]
    fn test_remove_7() {
        let mut b = new_tree::<i32>();
        let mut idx = 0;
        while idx < 7 {
            b.insert(idx);
            idx += 1;
        }

        while b.size() > 0 {
            let v = b.remove(b.nodes[b.root_idx].val);
            if DEBUG { println!("successfully removed: {:#?}", v); }
        }
    }

    #[test]
    fn test_remove_1_child() {
        let mut b = new_tree::<i32>();
        let arr = [15, 5, 20, 17];
        let mut idx = 0;
        while idx < arr.len() {
            b.insert(arr[idx]);
            idx += 1;
        }

        assert!(20 == b.remove(20));
    }

    #[test]
    fn test_remove_leaf() {
        let mut b = new_tree::<i32>();
        let arr = [15, 5, 20, 17];
        let mut idx = 0;
        while idx < arr.len() {
            b.insert(arr[idx]);
            idx += 1;
        }

        assert!(17 == b.remove(17));
    }

    #[test]
    fn test_remove_root() {
        let mut b = new_tree::<i32>();
        let arr = [15, 5, 20, 17];
        let mut idx = 0;
        while idx < arr.len() {
            b.insert(arr[idx]);
            idx += 1;
        }

        assert!(15 == b.remove(15));
    }

    #[test]
    fn test_remove_1_child_2() {
        let mut b = new_tree::<i32>();
        let arr = [15, 5, 20, 17];
        let mut idx = 0;
        while idx < arr.len() {
            b.insert(arr[idx]);
            idx += 1;
        }

        assert!(5 == b.remove(5));
    }
}
