// Developed by Andre Paiva (C) 2024
// Rust Book-Based Personal Exercises

use std::collections::HashMap;

fn main() {
    let mut v = vec![80, 47, 95, 3, 22, 114, 57, 80, 114, 114, 95, 3, 5];
    v.sort_unstable();
    let median = v[v.len()/2];

    let mut instance_counter = HashMap::new();
    for item in &v {
        let instance_ptr = instance_counter.entry(item).or_insert(0);
        *instance_ptr += 1;
    }

    let mut mode = 0;

    {
        let mut max = 0;
        for (k, v) in &instance_counter {
            if max == 0 || *v > max {
                max = *v;
                mode = **k;
            }
        }
    }

    println!("{:?}", v);
    println!("Median: {}", median);
    println!("Mode: {}", mode);
}