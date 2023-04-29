use std::{collections::HashSet, fs};

fn main() {
    let contents = fs::read_to_string("../inputs/day3.txt").expect("Input file not found");
    let mut set = HashSet::from(["0_0".to_string()]);
    let mut x = vec![0, 0];
    let mut y = vec![0, 0];
    for (i, ch) in contents.trim().chars().enumerate() {
        match ch {
            '>' => x[i % 2] += 1,
            '<' => x[i % 2] -= 1,
            '^' => y[i % 2] += 1,
            'v' => y[i % 2] -= 1,
            _ => panic!("Unexpected character"),
        }

        let code = format!("{}_{}", x[i % 2], y[i % 2]);
        set.insert(code);
    }

    println!("{}", set.len());
}
