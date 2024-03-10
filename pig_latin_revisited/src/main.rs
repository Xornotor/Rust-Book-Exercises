// Developed by Andre Paiva (C) 2024
// Rust Book-Based Personal Exercises

use std::io;

fn main() {
    println!("Pig_Latin_Translator_v1.0");
    println!("Type a sentence to be translated:");

    let mut sentence = String::new();
    io::stdin()
        .read_line(&mut sentence)
        .expect("Invalid sentence.");

    let mut pig_sentence = String::new();

    for word in sentence.split_whitespace() {
        match word.chars().next() {
            Some(ch) => match ch {
                'A' | 'a' | 'E' | 'e' | 'I' | 'i' | 'O' | 'o' | 'U' | 'u' => {
                    pig_sentence.push_str(word);
                    pig_sentence.push_str("-hay");
                }
                _ => {
                    let mut mod_word = String::from(word);
                    let removed_char = mod_word.remove(0);
                    pig_sentence.push_str(&mod_word);
                    pig_sentence.push('-');
                    pig_sentence.push(removed_char);
                    pig_sentence.push_str("ay");
                }
            },
            None => panic!("Error: char not found (?)"),
        }
        pig_sentence.push(' ');
    }

    println!("{pig_sentence}");
}
