# A Red-Black BST written with Rust

Why on earth did I do this? To learn about red-black trees and gain experience with Rust.

The algorithm for balancing the tree after an insert is directly based on material presented by Dr. Wenger in his [Data Structures and Algorithms](http://web.cse.ohio-state.edu/~wenger.4/cse2331/index.html) class.
The algorithm for balancing the tree after a remove was directly derived from: http://web.cse.ohio-state.edu/~lai.1/6331/0.Red-Black%20Trees.pdf.

## Available functionality

### insert(&mut self, key: T)
Takes O(log(n)).
Adds an element to the tree. Duplicates are permitted.

### remove(&mut self, key: T) -> T
Takes O(log(n)).
Removes an element from the tree and returns it. The element specified to be removed must already be in the tree. If there are duplicates, no guareentees are made about which duplicate gets removed.

### size(&self) -> usize
Takes O(1).
Returns the number of elements in the tree.
