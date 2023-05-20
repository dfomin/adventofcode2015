use std::fs;

fn main() {
    let mut original = vec![];
    let mut reading_map = true;
    for line in fs::read_to_string("../inputs/day19.txt")
        .unwrap()
        .trim()
        .lines()
    {
        if line.is_empty() {
            reading_map = false;
            continue;
        }

        if !reading_map {
            original = line.chars().collect();
        }
    }

    let mut all = 0;
    let mut rn = 0;
    let mut ar = 0;
    let mut y = 0;
    for i in 0..original.len() {
        if original[i].is_uppercase() {
            all += 1;
        }

        if original[i] == 'R' && original[i + 1] == 'n' {
            rn += 1;
        }

        if original[i] == 'A' && original[i + 1] == 'r' {
            ar += 1;
        }

        if original[i] == 'Y' {
            y += 1;
        }
    }

    println!("{}", all - rn - ar - 2 * y - 1);
}
