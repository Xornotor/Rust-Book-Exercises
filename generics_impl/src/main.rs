// Developed by Andre Paiva (C) 2024
// Rust Book-Based Personal Exercises

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn get_x(&self) -> &T {
        &self.x
    }

    fn get_y(&self) -> &T {
        &self.y
    }
}

impl Point<f64> {
    fn dist_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let integer = Point {x: 5, y: 10};
    let float = Point {x: 1.0, y: 4.0};

    println!("{} {} {} {}",
        integer.get_x(),
        integer.get_y(),
        float.get_x(),
        float.get_y());

    println!("{}", float.dist_from_origin());
    //println!("{}", integer.dist_from_origin());
}