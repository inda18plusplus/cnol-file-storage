
extern crate reqwest;
use reqwest::{
    Client,

};
use std::io::Read;

type FileID = usize;


fn main() {
    let client = Client::new();

    let bytes = download_raw_file(&client, 0);
    println!("Text: {:?}", std::str::from_utf8(&bytes.unwrap()));

    let status = upload_raw_file(&client, 6, b"This is the end".to_vec());
    println!("Status: {:?}", status.map(|status| status.to_string()));

    let bytes = download_raw_file(&client, 6);
    println!("Text: {:?}", std::str::from_utf8(&bytes.unwrap()));
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

fn upload_raw_file(client: &Client, file: FileID, data: Vec<u8>) -> reqwest::Result<reqwest::StatusCode> {
    client.put(&file_uri(file))
        .body(data)
        .send()
        .map(|response| response.status())
}

fn file_uri(file: FileID) -> String {
    format!("http://localhost:8000/file/{id}", id = file)
}