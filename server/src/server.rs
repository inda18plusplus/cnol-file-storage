#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

extern crate file_hash;

use rocket::{
    State,
    http::Status,
    response::status::Custom as CustomStatus
};

use std::sync::{
    Arc,
    RwLock
};
use std::collections::HashMap;

mod file_cache;
use file_cache::{
    FileCache,
    FileID
};


type Files = Arc<RwLock<FileCache>>;


#[get("/<file>")]
fn get_file(files: State<Files>, file: FileID) -> Option<Vec<u8>> {
    files.read().unwrap()
        .get(file)
}

#[put("/<file>", data="<data>")]
fn upload_file(files: State<Files>, file: FileID, data: Vec<u8>) -> CustomStatus<()> {
    match files.write().unwrap()
        .insert(file, data) {
        Some(_) => CustomStatus(Status::Ok, ()),
        None => CustomStatus(Status::Created, ())
    }
}

#[get("/")]
fn get_root_hash(files: State<Files>) -> Vec<u8> {
    files.read().unwrap()
        .root_hash()
}


fn main() {
    let files: Files = Arc::new(RwLock::new(FileCache::new()));

    rocket::ignite()
        .manage(files)
        .mount("/file", routes![get_file, upload_file, get_root_hash])
        .launch();
}


