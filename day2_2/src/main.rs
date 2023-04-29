use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn read_lines(filename: String) -> io::Lines<BufReader<File>> {
    // Open the file in read-only mode.
    let file = File::open(filename).unwrap();
    // Read the file line by line, and return an iterator of the lines of the file.
    return io::BufReader::new(file).lines();
}

fn main() {
    let mut result = 0;
    for line in read_lines("../inputs/day2_1.txt".to_string()) {
        let dims: Vec<i32> = line
            .unwrap()
            .split("x")
            .map(|x| x.parse::<i32>().unwrap())
            .into_iter()
            .collect();
        let mut ribbon = 0;
        let mut bow = 1;
        for dim in dims.iter() {
            ribbon += dim;
            bow *= dim;
        }

        result += 2 * (ribbon - dims.iter().max().unwrap()) + bow;
    }

    println!("{}", result);
}
