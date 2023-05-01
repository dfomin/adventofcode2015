use md5::{Digest, Md5};
use std::fs;

fn main() {
    let contents = fs::read_to_string("../inputs/day4.txt")
        .expect("Input file not found")
        .trim()
        .to_string();
    let mut i = 0;
    loop {
        let mut s = contents.clone();
        s.push_str(&format!("{i}"));

        let mut hasher = Md5::new();
        hasher.update(s);
        let hash = hasher.finalize();
        if hash[0] == 0 && hash[1] == 0 && hash[2] == 0 {
            break;
        }

        i += 1;
    }

    println!("{i}");
}
