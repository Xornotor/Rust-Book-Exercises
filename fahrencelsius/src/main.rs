// Developed by Andre Paiva (C) 2024
// Rust Book-Based Personal Exercises

fn f_to_c(f: f64) -> f64 {
    (f - 32.0) * (5.0/9.0)
}

fn c_to_f(c: f64) -> f64 {
    32.0 + (9.0/5.0) * c 
}

fn main() {
    println!("{}", c_to_f(27.0));
    println!("{}", f_to_c(104.0));
}
