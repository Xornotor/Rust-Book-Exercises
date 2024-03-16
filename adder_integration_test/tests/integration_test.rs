// Developed by Andre Paiva (C) 2024
// Rust Book-Based Personal Exercises

use adder_integration_test;

#[test]
fn it_adds_two() {
    assert_eq!(4, adder_integration_test::add_two(2));
}