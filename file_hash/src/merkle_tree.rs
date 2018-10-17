
use super::{
    Hash,
    hash
};


/// A perfectly binary hash tree
#[derive(Debug)]
pub struct MerkleTree {
    root: Node
}


#[derive(Debug)]
enum Node {
    Branch {
        hash: Hash,
        left: Box<Node>,
        right: Box<Node>
    },

    Leaf {
        hash: Hash
    },

    Empty
}
use self::Node::*;


impl MerkleTree {
    pub fn new(depth: u8) -> MerkleTree {
        MerkleTree {
            root: Node::empty_with_depth(depth)
        }
    }


    /// Returns the root hash
    pub fn root(&self) -> Hash {
        self.root.hash()
    }


    /// Returns a hash from the tree, if present
    pub fn get(&self, node: usize) -> Option<Hash> {
        self.root.get(node)
    }


    /// Inserts a new hash into the tree, returning the old hash, if present
    pub fn insert(&mut self, node: usize, hash: Hash) -> Option<Hash> {
        self.root.insert(node, hash)
    }
}


impl Node {
    pub fn empty_with_depth(depth: u8) -> Node {
        match depth {
            0 => {
                Empty
            }

            _ => {
                let left = Box::new(Node::empty_with_depth(depth - 1));
                let right = Box::new(Node::empty_with_depth(depth - 1));

                Branch {
                    hash: left.hash().join(right.hash()),

                    left,
                    right,
                }
            }
        }
    }


    /// Searches the tree for a node and returns it's hash
    pub fn get(&self, node: usize) -> Option<Hash> {
        match self {
            &Empty | &Leaf { .. } if node == 0 => {
                Some(self.hash())
            },

            &Branch { ref left, ref right, .. } => {
                let next_node = node >> 1;

                if node & 1 == 1 {
                    right.get(next_node)
                } else {
                    left.get(next_node)
                }
            }

            // Attempted to search deeper into leaf
            _ => None
        }
    }


    /// Inserts a new hash into the tree, updating all dependencies
    pub fn insert(&mut self, node: usize, hash: Hash) -> Option<Hash> {
        match self {
            &mut Empty if node == 0 => {
                *self = Leaf { hash };
                None
            }

            &mut Leaf { hash: ref mut current } if node == 0 => {
                use std::mem::replace;
                Some(replace(current, hash))
            }

            &mut Branch { ref mut left, ref mut right, hash: ref mut current } => {
                let next_node = node >> 1;

                // Update children hashes
                let old = if node & 1 == 1 {
                    right.insert(next_node, hash)
                } else {
                    left.insert(next_node, hash)
                };

                // Update hash
                *current = left.hash().join(right.hash());

                old
            }

            // Attempted to search deeper into leaf
            _ => None
        }
    }


    /// Returns the hash of a node
    pub fn hash(&self) -> Hash {
        match self {
            &Empty => hash(b"Hello, world!"),
            &Leaf { ref hash, .. } => {
                hash.clone()
            }
            &Branch { ref hash, .. } => {
                hash.clone()
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn update_tree() {
        let mut tree = MerkleTree::new(1);
        println!("tree: {:#x?}", tree);
        println!("root: {:#x?}", tree.root());

        let hash = hash(&[1, 2, 3]);
        tree.insert(0, hash.clone());

        println!("tree: {:#x?}", tree);
        println!("root: {:#x?}", tree.root());

        assert_eq!(hash.join(Empty.hash()), tree.root());
    }
}

