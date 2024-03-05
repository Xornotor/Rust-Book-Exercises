// Developed by Andre Paiva (C) 2024
// Rust Book-Based Personal Exercises

use std::io::stdin;

fn main() {
    let index = catch_input();
    println!("{}", fibonacci(index));
}

fn catch_input() -> u64 {
    let mut input = String::new();

    println!("Fibonacci -> Enter the position: ");

    loop{
        match stdin().read_line(&mut input) {
            Ok(_) => {
                match input.trim().parse() {
                    Ok(val) => return val,
                    Err(_) => {
                        println!("Invalid read. Try again: ");
                        input.clear();
                        continue;
                    }
                };
            },
            Err(_) => {
                print!("Invalid read. Try again: ");
                input.clear();
                continue;
            }
        }
    }
}

fn fibonacci(index: u64) -> u64 {
    match index {
        0 => 0,
        1 => 1,
        _ => {
            let mut minus1 = 0;
            let mut minus2 = 1;
            let mut val = 1;
            for _ in 2..=index {
                val = minus1 + minus2;
                minus1 = minus2;
                minus2 = val;
            }
            val
        }
    }
}