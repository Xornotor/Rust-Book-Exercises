// Developed by Andre Paiva (C) 2024
// Rust Book-Based Personal Exercises

use std::io::stdin;

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    println!("Enter the coin:");
    let mut coin_string = String::new();
    let coin: Coin;
    loop{
        stdin().read_line(&mut coin_string).unwrap();
        coin_string.pop();
        match coin_string.as_str() {
            "Penny" => {
                coin = Coin::Penny;
                break;
            },
            "Nickel" => {
                coin = Coin::Nickel;
                break;
            },
            "Dime" => {
                coin = Coin::Dime;
                break;
            },
            "Quarter" => {
                coin = Coin::Quarter;
                break;
            },
            _ => println!("Invalid coin {}. Enter the coin:", coin_string)
        }
        coin_string.clear();
    }
    println!("The value of the coin is {}", value_in_cents(coin));
}