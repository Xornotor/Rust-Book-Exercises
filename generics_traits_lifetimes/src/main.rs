// Developed by Andre Paiva (C) 2024
// Rust Book-Based Personal Exercises

use std::fmt::Display;

fn longest_with_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let x = "Test";
    let y = "Testing";
    let ann = String::from("Birb");

    let longest = longest_with_announcement(&x, &y, ann);
    println!("{}", longest);
}
