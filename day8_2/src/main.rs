use std::fs;

fn main() {
    let mut counter = 0i32;
    for s in fs::read_to_string("../inputs/day8.txt")
        .unwrap()
        .trim()
        .split("\n")
    {
        let chars = s.chars().collect::<Vec<_>>();
        counter += 2;
        counter -= chars.len() as i32;
        for ch in chars {
            if ch == '\\' || ch == '\"' || ch == '\'' {
                counter += 1;
            }

            counter += 1;
        }
    }

    println!("{}", counter);
}
