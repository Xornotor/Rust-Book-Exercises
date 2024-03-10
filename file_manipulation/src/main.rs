// Developed by Andre Paiva (C) 2024
// Rust Book-Based Personal Exercises

use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn main() {
    let user = read_username_from_file().unwrap();
    println!("{user}");
}