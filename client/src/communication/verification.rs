
use reqwest::{
    Client,
};

use std::{
    self,
    fs::File,
    io::{
        self,
        Read,
        Write
    },
    mem
};

use file_hash::{
    Hash,
    hash,
    MerkleTree
};

use super::{
    FileID,
    download,
};


#[derive(Debug)]
pub enum Error {
    /// The client's root hash could not be loaded
    ClientHashNotFound(io::Error),

    /// The client's hash is invalid
    ClientHashInvalid(io::Error),

    /// Could not save the client's hash
    ClientHashNoWrite(io::Error),

    /// The server's root hash could not be downloaded
    ServerHashNotFound(super::Error),

    /// The server's hash dependencies could not be downloaded
    ServerHashDependenciesNotFound(super::Error),

    /// Got an unexpected amount of hash dependencies from the server
    InvalidHashDependencyCount(usize),

    /// The server's and client's root hash did not match
    HashOutOfDate {
        client: Hash,
        server: Hash
    },

    /// The file has been modified by a third party
    TamperedFiles,
}


// Shorthand for Results originating in this module
type Result<T> = std::result::Result<T, Error>;

// Path to the client's root hash
const ROOT_HASH_PATH: &'static str = "root_hash";


/// Verify that a file has not been modified. Returns `Ok` if that's the case, `Err` otherwise
pub fn verify_file(client: &Client, file: FileID, data: &[u8]) -> Result<()> {
    let client_root_hash = get_client_root_hash(client)?;

    verify_root_hashes(client, client_root_hash.clone())?;

    let dependencies = get_file_dependencies(client, file)?;
    let file_hash = hash(data);

    let root_hash = MerkleTree::reconstruct_root_hash(dependencies, file as usize, file_hash);

    if root_hash == client_root_hash {
        Ok(())
    } else {
        println!("client: {:#?}", client_root_hash);
        println!("reconstructed: {:#?}", root_hash);

        Err(Error::TamperedFiles)
    }
}


/// Update the client's root hash
pub fn update_root_hash(client: &Client, client_root_hash: Hash) -> Result<()> {
    verify_root_hashes(client, client_root_hash.clone())?;

    save_client_root_hash(client_root_hash)
}


/// Compute a new root hash based on a files location and it's expected data
pub fn compute_new_root_hash(client: &Client, file: FileID, data: &[u8]) -> Result<Hash> {
    let client_root_hash = get_client_root_hash(client)?; 
    verify_root_hashes(client, client_root_hash)?;

    let dependencies = get_file_dependencies(client, file)?;

    let file_hash = hash(data);
    let root_hash = MerkleTree::reconstruct_root_hash(dependencies, file as usize, file_hash);

    Ok(root_hash)
}


/// Verify that this client's and the server's root hashes match
fn verify_root_hashes(client: &Client, client_root_hash: Hash) -> Result<()> {
    let server_root_hash = get_server_root_hash(client)?;

    if client_root_hash != server_root_hash {
        Err(Error::HashOutOfDate{client: client_root_hash, server: server_root_hash})
    } else {
        Ok(())
    }
}


/// Return the hashes required to reconstruct the root hash from a specific file
fn get_file_dependencies(client: &Client, file: FileID) -> Result<Vec<Hash>> {
    match download(client, &verify_file_uri(file)) {
        Ok(bytes) => {
            let hash_count = bytes.len() / Hash::BYTES;

            if hash_count != mem::size_of::<FileID>() * 8 {
                return Err(Error::InvalidHashDependencyCount(hash_count));
            }

            let mut hashes = Vec::new();
            hashes.reserve(hash_count);

            for i in 0..hash_count {
                let low = i * Hash::BYTES;
                let high = low + Hash::BYTES;

                hashes.push(Hash::from_bytes(&bytes[low..high]))
            }

            Ok(hashes)
        }

        Err(e) => Err(Error::ServerHashDependenciesNotFound(e)),
    }
}


/// Attempts to download the server's root hash
fn get_server_root_hash(client: &Client) -> Result<Hash> {
    match download(client, &root_hash_uri()) {
        Ok(bytes) => Ok(Hash::from_bytes(&bytes)),
        Err(e) => Err(Error::ServerHashNotFound(e)),
    }
}

/// Attempts to load the client's root hash.
/// If the client does not have a root hash 
/// a new one be downloaded from the server.
fn get_client_root_hash(client: &Client) -> Result<Hash> {
    match File::open(ROOT_HASH_PATH) {
        Ok(mut file) => {
            let mut hash = Hash::default();

            match file.read_exact(hash.as_bytes_mut()) {
                Ok(_) => Ok(hash),
                Err(e) => Err(Error::ClientHashInvalid(e))
            }
        },

        Err(ref e) if e.kind() == io::ErrorKind::NotFound => {
            initialize_client_hash(client)
        },

        Err(e) => Err(Error::ClientHashNotFound(e))
    }
}

/// Downloads the root hash of the server and saves it to the client
fn initialize_client_hash(client: &Client) -> Result<Hash> {
    let hash = get_server_root_hash(client)?;
    save_client_root_hash(hash.clone())?;
    Ok(hash)
}


/// Attempts to save the client's root hash
fn save_client_root_hash(hash: Hash) -> Result<()> {
    match File::create(ROOT_HASH_PATH) {
        Ok(mut file) => {
            match file.write(hash.as_bytes()) {
                Ok(_) => Ok(()),
                Err(e) => Err(Error::ClientHashNoWrite(e))
            }
        },

        Err(e) => Err(Error::ClientHashNotFound(e)),
    }
}

/// Get the URI to the verification hashes for a file on the server
fn verify_file_uri(file: FileID) -> String {
    format!("http://localhost:8000/file/verify/{id}", id = file)
}

/// Get the URI to the top hash on the server
fn root_hash_uri() -> String {
    "http://localhost:8000/file/verify/root".to_owned()
}
