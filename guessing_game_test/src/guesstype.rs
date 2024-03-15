// Developed by Andre Paiva (C) 2024
// Rust Book-Based Personal Exercises

pub struct Guess {
    value: u8,
}

impl Guess {
    pub fn new(value: i64) -> Guess {
        if value < 1 || value > 100 {
            panic!("Value must be between 1 and 100.");
        }
        let u8_val = value as u8;
        Guess {value: u8_val}
    }

    pub fn value(&self) -> u8 {
        self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
}