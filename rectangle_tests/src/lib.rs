// Developed by Andre Paiva (C) 2024
// Rust Book-Based Personal Exercises

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn can_hold(&self, other:&Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 30,
            height: 45,
        };
        let smaller = Rectangle {
            width: 20,
            height: 15,
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cant_hold_larger() {
        let smaller = Rectangle {
            width: 30,
            height: 45,
        };
        let larger = Rectangle {
            width: 50,
            height: 75,
        };
        assert!(!smaller.can_hold(&larger));
    }
}