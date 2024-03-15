// Developed by Andre Paiva (C) 2024
// Rust Book-Based Personal Exercises

mod guesstype;

use std::io::stdin;
use std::cmp::Ordering;
use guesstype::Guess;
use rand::Rng;

fn main() {
    let secret_number: u8 = rand::thread_rng().gen_range(1..=100);

    println!("Guessing Game v2.0");
    println!("Enter your guess (1-100):");

    let mut user_input = String::new();

    loop {
        
        stdin().read_line(&mut user_input)
            .expect("Failed to read content from user");
        
        let user_value: i64 = user_input.trim().parse()
            .expect("Failed to convert user input to number");

        let user_guess: Guess = Guess::new(user_value);

        match user_guess.value().cmp(&secret_number) {
            Ordering::Greater => println!("Too big. Try a lower number."),
            Ordering::Less => println!("Too small. Try a higher number."),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

        user_input.clear();
    }
}
