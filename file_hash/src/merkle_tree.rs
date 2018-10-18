
use std;
use super::{
    Hash,
    hash
};


/// A perfectly binary hash tree
#[derive(Debug)]
pub struct MerkleTree {
    root: Node,
    depth: u8
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


#[derive(Debug)]
pub enum Error {
    NodeNotPresent
}

pub type Result<T> = std::result::Result<T, Error>;


impl MerkleTree {
    pub fn new(depth: u8) -> MerkleTree {
        if depth > 32 {
            panic!(format!("Attempted to construct too deep MerkelTree ({})", depth));
        }

        MerkleTree {
            root: Node::empty_with_depth(depth),
            depth
        }
    }


    /// Returns the root hash
    pub fn root(&self) -> Hash {
        self.root.hash()
    }


    /// Returns a hash from the tree, if present
    pub fn get(&self, node: usize) -> Result<Hash> {
        if self.index_in_bounds(node) {
            self.root.get(node)
        } else {
            Err(Error::NodeNotPresent)
        }
    }


    /// Inserts a new hash into the tree, returning the old hash, if present
    pub fn insert(&mut self, node: usize, hash: Hash) -> Result<Option<Hash>> {
        if self.index_in_bounds(node) {
            self.root.insert(node, hash)
        } else {
            Err(Error::NodeNotPresent)
        }
    }


    /// Returns a sequence of hashes required to construct to root hash
    /// from a leaf node.
    ///
    /// The sequence is built from the bottom up. That is, the first hash
    /// in the sequence is the sibling of the leaf node. After combining
    /// these hashes you will get the sibling of the next hash in the
    /// sequence. This continues until the root node is reached.
    pub fn dependencies(&self, node: usize) -> Result<Vec<Hash>> {
        if self.index_in_bounds(node) {
            self.root.dependencies(node)
        } else {
            Err(Error::NodeNotPresent)
        }
    }


    /// Reconstructs tho root hash based on all required sibling hashes and the
    /// location of a file in a MerkleTree.
    pub fn reconstruct_root_hash(dependencies: Vec<Hash>, mut node: usize, node_hash: Hash) ->
    Hash {
        let mask = 1 << (dependencies.len() - 1);

        let mut result = node_hash;

        for hash in dependencies {
            // node index begins with 0 => node is on the left
            // node index begins with 1 => node is on the right
            let (left, right) = if node & mask == 0 {
                (result, hash)
            } else {
                (hash, result)
            };

            result = left.join(right);
            node = node << 1;
        }

        result
    }


    fn index_in_bounds(&self, node: usize) -> bool {
        node < 1usize << self.depth
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
    pub fn get(&self, node: usize) -> Result<Hash> {
        match self {
            &Leaf { .. } if node == 0 => {
                Ok(self.hash())
            },

            &Branch { ref left, ref right, .. } => {
                let next_node = node >> 1;

                if node & 1 == 1 {
                    right.get(next_node)
                } else {
                    left.get(next_node)
                }
            }

            // Attempted to search deeper into leaf or landed on empty node
            _ => Err(Error::NodeNotPresent)
        }
    }


    /// Inserts a new hash into the tree, updating all dependencies
    pub fn insert(&mut self, node: usize, hash: Hash) -> Result<Option<Hash>> {
        match self {
            &mut Empty if node == 0 => {
                *self = Leaf { hash };
                Ok(None)
            }

            &mut Leaf { hash: ref mut current } if node == 0 => {
                use std::mem::replace;
                Ok(Some(replace(current, hash)))
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
            _ => Err(Error::NodeNotPresent)
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


    /// Returns all hashes required to construct a node's
    /// hash from one of it's children.
    pub fn dependencies(&self, node: usize) -> Result<Vec<Hash>> {
        match self {
            &Empty | &Leaf { .. } if node == 0 => {
                Ok(Vec::new())
            },

            &Branch { ref left, ref right, .. } => {
                let next_node = node >> 1;

                if node & 1 == 1 {
                    right.dependencies(next_node)
                        .map(|mut hashes| {
                            hashes.push(left.hash());
                            hashes
                        })
                } else {
                    left.dependencies(next_node)
                        .map(|mut hashes| {
                            hashes.push(right.hash());
                            hashes
                        })
                }
            }

            // Attempted to search deeper into leaf
            _ => Err(Error::NodeNotPresent)
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn update_tree() {
        let mut tree = MerkleTree::new(1);
        println!("tree: {:x?}", tree);
        println!("root: {:x?}", tree.root());

        let hash = hash(&[1, 2, 3]);
        tree.insert(0, hash.clone()).unwrap();

        println!("tree: {:x?}", tree);
        println!("root: {:x?}", tree.root());

        assert_eq!(hash.join(Empty.hash()), tree.root());
    }


    #[test]
    fn dependencies() {
        let mut tree = MerkleTree::new(5);
        println!("tree: {:#x?}", tree);
        println!("dependencies: {:#?}", tree.dependencies(18).unwrap().len());
    }
}

