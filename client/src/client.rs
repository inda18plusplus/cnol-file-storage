extern crate reqwest;
extern crate ring;

extern crate file_hash;

mod communication;

use communication::{
    upload_file,
    download_file
};


fn main() {
    let client = reqwest::Client::new();

    let password = b"abc";

    use std::str::from_utf8 as s;

    {
        let message = b"Super secret message";
        println!("Uploading message: {:?}", s(message));
        upload_file(message, 1342, password, &client);
    }

    {
        let file = download_file(1342, password, &client);
        println!("Downloaded message: {:?}", s(&file));
    }
}




