use std::{collections::HashMap, fs};

fn main() {
    let original = HashMap::from([
        ("children", 3),
        ("cats", 7),
        ("samoyeds", 2),
        ("pomeranians", 3),
        ("akitas", 0),
        ("vizslas", 0),
        ("goldfish", 5),
        ("trees", 3),
        ("cars", 2),
        ("perfumes", 1),
    ]);

    for (i, line) in fs::read_to_string("../inputs/day16.txt")
        .unwrap()
        .trim()
        .lines()
        .enumerate()
    {
        let mut found = true;
        for value in line.split(", ") {
            let parts = value.split(": ").collect::<Vec<_>>();
            let key = parts[parts.len() - 2];
            let value = parts[parts.len() - 1].parse::<i32>().unwrap();
            if original.contains_key(key) {
                if original[key] != value {
                    found = false;
                    break;
                }
            }
        }

        if found {
            println!("{}", i + 1);
        }
    }
}
