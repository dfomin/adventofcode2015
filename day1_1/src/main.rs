use std::fs;

fn main() {
    let contents = fs::read_to_string("../inputs/day1.txt").expect("Input file not found");
    let mut result = 0;
    for ch in contents.trim().chars() {
        match ch {
            '(' => result += 1,
            ')' => result -= 1,
            _ => panic!("Unexpected char"),
        }
    }

    println!("{}", result);
}
