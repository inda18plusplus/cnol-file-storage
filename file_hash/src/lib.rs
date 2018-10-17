
extern crate ring;

use ring::{
    digest
};


mod merkle_tree;
pub use merkle_tree::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Hash(Vec<u8>);

pub fn hash(bytes: &[u8]) -> Hash {
    Hash(digest::digest(&digest::SHA256, bytes).as_ref().to_vec())
}


impl Hash {
    pub fn join(self, other: Hash) -> Hash {
        let mut a = self.0;
        let b = other.0;

        a.extend_from_slice(&b);

        hash(&a)
    }
}