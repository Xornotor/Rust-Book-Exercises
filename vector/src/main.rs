// Developed by Andre Paiva (C) 2024
// Rust Book-Based Personal Exercises

enum Type {
    Int(i64),
    Float(f64),
    Str(String),
}

fn main(){
    let mut v: Vec<Type> = vec![Type::Int(957),
                                Type::Float(762.63),
                                Type::Str(String::from("Meuca"))];

    v.push(Type::Str(String::from("cete")));
    v.push(Type::Int(42));
    
    
    for item in &v {
        match item {
            Type::Int(val) => println!("Integer: {val}"),
            Type::Float(val) => println!("Floating point: {val}"),
            Type::Str(val) => println!("String: {val}"),
        }
    }
}