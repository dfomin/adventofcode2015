use std::{collections::HashSet, fs};

fn main() {
    let contents = fs::read_to_string("../inputs/day3.txt").expect("Input file not found");
    let mut set = HashSet::from(["0_0".to_string()]);
    let mut x = 0;
    let mut y = 0;
    for ch in contents.trim().chars() {
        match ch {
            '>' => x += 1,
            '<' => x -= 1,
            '^' => y += 1,
            'v' => y -= 1,
            _ => panic!("Unexpected character"),
        }

        let code = format!("{}_{}", x, y);
        set.insert(code);
    }

    println!("{}", set.len());
}
