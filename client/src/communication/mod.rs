
mod secret;
mod verification;

use reqwest::{
    self,
    Client,
    StatusCode,
};

use std::io::Read;

use self::secret::Secret;
use self::verification::{
    verify_file,
    compute_new_root_hash,
    update_root_hash
};


#[derive(Debug)]
pub enum Error {
    Reqwest(reqwest::Error),
    Http(StatusCode)
}

type FileID = u16;


/// Encrypt a file and upload it to the server
pub fn upload_file(message: &[u8], file: FileID, password: &[u8], client: &Client) {
    let verification = serialize_file_id(file);

    let secret = Secret::new(password, message, &verification);

    let root_hash = compute_new_root_hash(client, file, &secret.as_bytes())
        .expect("Failed to compute new root hash");

    upload(client, &file_uri(file), secret.as_bytes())
        .expect("Failed to upload file");

    update_root_hash(client, root_hash)
        .expect("Failed to update root hash");

    verify_file(client, file, &secret.as_bytes())
        .expect("File not stored correctly")
}


/// Download, verify and decrypt a file from the server
pub fn download_file(file: FileID, password: &[u8], client: &Client) -> Vec<u8> {
    let verification = serialize_file_id(file);

    let bytes = download(client, &file_uri(file))
        .expect("Failed to download file");

    verify_file(client, file, &bytes)
        .expect("Failed to verify authenticity of file");

    let secret = Secret::from_bytes(&bytes)
        .expect("Failed to interpret file");

    secret.reveal(password, &verification)
        .expect("Failed to decrypt file")
}


/// Convert a file into a byte array
fn serialize_file_id(id: FileID) -> [u8; 2] {
    use std::mem::transmute;
    unsafe { transmute(id.to_be()) }
}


/// Upload some bytes to the server
fn upload(client: &Client, uri: &str, data: Vec<u8>) -> reqwest::Result<StatusCode> {
    client.put(uri)
        .body(data)
        .send()
        .map(|response| response.status())
}

/// Download some bytes from the server
fn download(client: &Client, uri: &str) -> Result<Vec<u8>, Error> {
    client.get(uri)
        .send()
        .map_err(|error| Error::Reqwest(error))
        .and_then(|response| match response.status() {
            StatusCode::OK => Ok(response),
            code => Err(Error::Http(code))
        })
        .map(|response| response
                .bytes()
                .map(Result::unwrap)
                .collect()
        )
}


/// Get the URI to a file on the server
fn file_uri(file: FileID) -> String {
    format!("http://localhost:8000/file/{id}", id = file)
}

