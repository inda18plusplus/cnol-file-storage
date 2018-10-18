
use file_hash::{
    MerkleTree,
    hash
};

use std::collections::HashMap;

pub type FileID = u16;

pub struct FileCache {
    files: HashMap<FileID, Vec<u8>>,
    hashes: MerkleTree
}

impl FileCache {
    pub fn new() -> FileCache {
        use std::mem::size_of;

        FileCache {
            files: HashMap::new(),
            hashes: MerkleTree::new(size_of::<FileID>() as u8 * 8)
        }
    }


    pub fn insert(&mut self, file: FileID, data: Vec<u8>) -> Option<Vec<u8>> {
        let hash = hash(&data);
        self.hashes.insert(file as usize, hash).unwrap();

        self.files.insert(file, data)
    }


    pub fn get(&self, id: FileID) -> Option<Vec<u8>> {
        self.files.get(&id).cloned()
    }


    pub fn root_hash(&self) -> Vec<u8> {
        self.hashes.root().into_vec()
    }

    /// Return a list of 32 byte hashes
    pub fn hash_dependencies(&self, file: FileID) -> Option<Vec<u8>> {
        match self.hashes.dependencies(file as usize) {
            Ok(dependencies) => {
                let deps = dependencies.into_iter()
                    .map(|hash| hash.into_vec())
                    .flatten()
                    .collect();

                Some(deps)
            }

            Err(_) => None
        }
    }
}
