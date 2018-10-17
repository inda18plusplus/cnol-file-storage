
use file_hash::MerkleTree;

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
            hashes: MerkleTree::new(size_of::<FileID>() as u8)
        }
    }


    pub fn insert(&mut self, id: FileID, file: Vec<u8>) -> Option<Vec<u8>> {
        self.files.insert(id, file)
    }


    pub fn get(&self, id: FileID) -> Option<Vec<u8>> {
        self.files.get(&id).cloned()
    }


    pub fn root_hash(&self) -> Vec<u8> {
        Vec::new()
    }
}
