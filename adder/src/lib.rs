// Developed by Andre Paiva (C) 2024
// Rust Book-Based Personal Exercises

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration(){
        assert_eq!(add(2, 5), 7);
    }

    #[test]
    fn another(){
        panic!("FAIL. DON'T QUESTION IT.");
    }
}
