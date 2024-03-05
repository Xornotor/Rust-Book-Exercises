// Developed by Andre Paiva (C) 2024
// Rust Book-Based Personal Exercises

use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}