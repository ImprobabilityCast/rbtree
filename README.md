# A Red-Black BST written with Rust

Why on earth did I do this? To learn about red-black trees and gain experience with Rust.

At the time of writing, I'm still leery of Rust, but I now have a good understanding of red-black trees.

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
