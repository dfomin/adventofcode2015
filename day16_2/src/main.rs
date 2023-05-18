use std::{collections::HashMap, fs};

fn main() {
    let original = HashMap::from([
        ("children", Box::new(|x| x == 3) as Box<dyn Fn(i32) -> bool>),
        ("cats", Box::new(|x| x > 7)),
        ("samoyeds", Box::new(|x| x == 2)),
        ("pomeranians", Box::new(|x| x < 3)),
        ("akitas", Box::new(|x| x == 0)),
        ("vizslas", Box::new(|x| x == 0)),
        ("goldfish", Box::new(|x| x < 5)),
        ("trees", Box::new(|x| x > 3)),
        ("cars", Box::new(|x| x == 2)),
        ("perfumes", Box::new(|x| x == 1)),
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
                if !original[key](value) {
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
