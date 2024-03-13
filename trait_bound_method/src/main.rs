// Developed by Andre Paiva (C) 2024
// Rust Book-Based Personal Exercises

use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest number is x = {}", self.x);
        } else {
            println!("The largest number is y = {}", self.y);
        }
    }
}

fn main() {
    let num_pair = Pair::new(80, 60);
    num_pair.cmp_display();

    //cmp_display doesn't work with option_pair
    
    //let option_pair = Pair::new(Some(30), Some(60));
    //option_pair.cmp_display();
}