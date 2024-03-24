enum List {
    Cons(i32, Box<List>),
    Nil,
}

impl List{
    fn print(&self) {
        print!("Lista:");
        let mut item = self;
        loop {
            if let List::Cons(val, list) = item {
                print!(" {}", val);
                item = list;
            } else {
                println!();
                break;
            }
        }
    }
}

use crate::List::*;

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    list.print();
}
