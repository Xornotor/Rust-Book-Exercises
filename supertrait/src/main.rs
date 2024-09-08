use std::fmt::{self, Display, Formatter};

trait OutlinePrint: Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len+4));
        println!("*{}*", " ".repeat(len+2));
        println!("* {output} *");
        println!("*{}*", " ".repeat(len+2));
        println!("{}", "*".repeat(len+4));
    }
}

struct Point {
    x: f64,
    y: f64
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result{
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Point {}

fn main() {
    let p = Point { x: 1.0, y: 1.0 };
    p.outline_print();
}