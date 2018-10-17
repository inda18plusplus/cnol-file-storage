extern crate reqwest;
extern crate ring;

extern crate file_hash;

use reqwest::{
    Client,
    StatusCode
};


use std::io::Read;

mod secret;
use secret::Secret;

type FileID = u16;


fn main() {
    let client = Client::new();

    let password = b"abc";

    use std::str::from_utf8 as s;

    {
        let message = b"Super secret message";
        println!("Uploading message: {:?}", s(message));
        upload_file(message, 0, password, &client);
    }

    {
        let file = download_file(0, password, &client);
        println!("Downloaded message: {:?}", s(&file));
    }
}



fn upload_file(message: &[u8], file: FileID, password: &[u8], client: &Client) {
    let verification = serialize_file_id(file);

    let secret = Secret::new(password, message, &verification);

    upload(client, &file_uri(file), secret.as_bytes())
        .expect("Failed to upload file");
}


fn download_file(file: FileID, password: &[u8], client: &Client) -> Vec<u8> {
    let verification = serialize_file_id(file);

    let bytes = download(client, &file_uri(file))
        .expect("Failed to download file");

    let secret = Secret::from_bytes(&bytes)
        .expect("Failed to interpret file");

    secret.reveal(password, &verification)
        .expect("Failed to decrypt file")
}


fn serialize_file_id(id: FileID) -> [u8; 2] {
    use std::mem::transmute;
    unsafe { transmute(id.to_be()) }
}

fn upload(client: &Client, uri: &str, data: Vec<u8>) -> reqwest::Result<StatusCode> {
    client.put(uri)
        .body(data)
        .send()
        .map(|response| response.status())
}

fn download(client: &Client, uri: &str) -> reqwest::Result<Vec<u8>> {
    client.get(uri)
        .send()
        .map(|response| response
            .bytes()
            .map(Result::unwrap)
            .collect()
        )
}


fn file_uri(file: FileID) -> String {
    format!("http://localhost:8000/file/{id}", id = file)
}

fn verify_file_uri(file: FileID) -> String {
    format!("http://localhost:8000/file/verify/{id}", id = file)
}

fn root_hash_uri() -> String {
    "http://localhost:8000/file/verify/root".to_owned()
}
