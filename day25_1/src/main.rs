use std::fs;

fn main() {
    let binding = fs::read_to_string("../inputs/day25.txt").unwrap();
    let parts = binding.trim().split(" ").collect::<Vec<_>>();
    let row = parts[parts.len() - 3];
    let row = row[..row.len() - 1].parse::<usize>().unwrap() - 1;
    let col = parts[parts.len() - 1];
    let col = col[..col.len() - 1].parse::<usize>().unwrap() - 1;

    let mut i = 0;
    let mut j = 0;
    let mut result = 20151125u64;
    while i != row || j != col {
        if i == 0 {
            i = j + 1;
            j = 0;
        } else {
            i -= 1;
            j += 1;
        }

        result *= 252533;
        result %= 33554393;
    }

    println!("{}", result);
}
