// Developed by Andre Paiva (C) 2024
// Rust Book-Based Personal Exercises

use minigrep_improved::Config;
use std::env;
use std::process;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep_improved::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
