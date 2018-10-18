
extern crate ring;

use ring::{
    digest
};


mod merkle_tree;
pub use merkle_tree::*;


/// A 32-byte hash
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Hash([u8; Hash::BYTES]);


/// Shorthand for creating a new hash of some bytes
pub fn hash(bytes: &[u8]) -> Hash {
    Hash::new(bytes)
}

impl Hash {
    pub const BYTES: usize = 32;

    /// Create a new hash by hashing some bytes
    pub fn new(bytes: &[u8]) -> Hash {
        let digest = digest::digest(&digest::SHA256, bytes);
        Hash::from_bytes(digest.as_ref())
    }

    /// Create a new hash from a raw byte-array slice
    pub fn from_bytes(bytes: &[u8]) -> Hash {
        let mut array: [u8; 32] = Default::default();

        array.copy_from_slice(&bytes[..32]);

        Hash(array)
    }


    /// Concatenate two hashes and return a hash of the result
    pub fn join(self, other: Hash) -> Hash {
        let mut sum = Vec::new();

        sum.extend_from_slice(&self.0);
        sum.extend_from_slice(&other.0);

        hash(&sum)
    }


    /// Return the hash as a vector
    pub fn into_vec(self) -> Vec<u8> {
        self.0.to_vec()
    }

    /// Return the hash as a byte-array slice
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// Return the hash as a mutable byte-array slice
    pub fn as_bytes_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}


impl Default for Hash {
    fn default() -> Self {
        Hash([0; 32])
    }
}