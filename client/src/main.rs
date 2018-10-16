extern crate reqwest;
extern crate ring;

use reqwest::{
    Client,
    StatusCode
};


use std::io::Read;

mod secret;
use secret::Secret;

type FileID = u32;


fn main() {
    let message = b"Super secret message";
    let password = b"abc";
    let verification = serialize_file_id(3);

    use std::str::from_utf8 as s;
    println!("Encrypting '{}' with '{}'", s(message).unwrap(), s(password).unwrap());

    let secret = Secret::new(password, message, &verification);

    let client = Client::new();
    upload_secret(&client, 4, secret).unwrap();


    let secret = download_secret(&client, 4).unwrap();

    let data = secret.reveal(password, &verification).expect("Could not reveal secret");
    println!("data: {:?}", s(&data))
}


fn serialize_file_id(id: FileID) -> [u8; 4] {
    use std::mem::transmute;
    unsafe { transmute(id.to_be()) }
}



fn download_secret(client: &Client, file: FileID) -> reqwest::Result<Secret> {
    download_raw_file(client, file)
        .map(|bytes| Secret::from_bytes(&bytes)
            .expect("Received invalid data"))
}

fn upload_secret(client: &Client, file: FileID, secret: Secret) -> reqwest::Result<StatusCode> {
    upload_raw_file(client, file, secret.as_bytes())
}

fn download_raw_file(client: &Client, file: FileID) -> reqwest::Result<Vec<u8>> {
    client.get(&file_uri(file))
        .send()
        .map(|response| response
            .bytes()
            .map(Result::unwrap)
            .collect()
        )
}

fn upload_raw_file(client: &Client, file: FileID, data: Vec<u8>) -> reqwest::Result<StatusCode> {
    client.put(&file_uri(file))
        .body(data)
        .send()
        .map(|response| response.status())
}

fn file_uri(file: FileID) -> String {
    format!("http://localhost:8000/file/{id}", id = file)
}