// Developed by Andre Paiva (C) 2024
// Rust Book-Based Personal Exercises

use std::collections::HashMap;
use std::io;
use itertools::Itertools;

fn main() {
    let mut employee_map: HashMap<String, Vec<String>> = HashMap::new();
    collect_inputs(&mut employee_map);
    order_vectors(&mut employee_map);
    print_teams(&employee_map);
}

fn collect_inputs(map: &mut HashMap<String, Vec<String>>) {
    'collect_loop: loop {
        let mut command = String::new();
        io::stdin().read_line(&mut command).expect("Invalid input.");
        let mut add: bool = false;
        let mut to: bool = false;
        let mut name_to_add = String::new();
        let mut sector_to_add = String::new();
        for word in command.split_whitespace() {
            match word {
                "Add" | "add" | "ADd" | "aDd" | "AdD" | "adD" | "ADD" | "aDD" => {
                    add = true;
                    to = false;
                }
                "to" | "To" | "tO" | "TO" => {
                    add = false;
                    to = true;
                }
                "end" | "End" | "eNd" | "ENd" | "enD" | "EnD" | "eND" | "END" => {
                    if !add && !to {
                        break 'collect_loop;
                    }
                }
                _ => {
                    if add {
                        if name_to_add.len() > 0 {
                            name_to_add.push(' ');
                        }
                        name_to_add.push_str(word);
                    } else if to {
                        if sector_to_add.len() > 0 {
                            sector_to_add.push(' ');
                        }
                        sector_to_add.push_str(word);
                    }
                }
            }
        }

        let vec_addr = map.entry(sector_to_add).or_insert(Vec::new());
        vec_addr.push(name_to_add);
    }
}

fn order_vectors(map: &mut HashMap<String, Vec<String>>) {
    for (_k, v) in map {
        v.sort_unstable();
    }
}

fn print_teams(map: &HashMap<String, Vec<String>>) {
    for key in map.keys().sorted() {
        println!("");
        println!("{key} team:");
        for name in map.get(key).unwrap() {
            println!("- {name}");
        }
    }
}
