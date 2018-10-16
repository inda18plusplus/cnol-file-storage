
use std::collections::HashMap;

type FileID = usize;

pub struct FileCache {
    files: HashMap<FileID, Vec<u8>>
}

impl FileCache {
    pub fn new() -> FileCache {
        FileCache {
            files: HashMap::new(),
        }
    }


    pub fn insert(&mut self, id: FileID, file: Vec<u8>) -> Option<Vec<u8>> {
        self.files.insert(id, file)
    }


    pub fn get(&self, id: FileID) -> Option<Vec<u8>> {
        self.files.get(&id).cloned()
    }
}
