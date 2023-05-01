use std::{collections::HashSet, fs};

fn main() {
    let mut counter = 0;
    for s in fs::read_to_string("../inputs/day5.txt")
        .unwrap()
        .split("\n")
    {
        let mut set = HashSet::new();
        let mut has_double = false;
        let mut has_substr = false;
        let chars: Vec<char> = s.chars().collect();
        for i in 0..chars.len() {
            if i >= 2 {
                if chars[i] == chars[i - 2] {
                    has_double = true;
                }

                if set.contains(&format!("{}{}", chars[i - 1], chars[i])) {
                    has_substr = true;
                }

                set.insert(format!("{}{}", chars[i - 2], chars[i - 1]));
            }
        }

        if has_double && has_substr {
            counter += 1;
        }
    }

    println!("{}", counter);
}
