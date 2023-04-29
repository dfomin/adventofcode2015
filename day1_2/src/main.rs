use std::fs;

fn main() {
    let contents = fs::read_to_string("../inputs/day1.txt").expect("Input file not found");
    let mut result = 0;
    let mut position: Option<i32> = None;
    for (i, ch) in contents.trim().chars().enumerate() {
        match ch {
            '(' => result += 1,
            ')' => result -= 1,
            _ => panic!("Unexpected char"),
        }

        if result == -1 {
            position = Some(i as i32 + 1);
            break;
        }
    }

    println!("{:?}", position);
}
