use std::fs;

fn main() {
    let number = fs::read_to_string("../inputs/day20.txt")
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap()
        / 10;
    let mut i = 1;
    loop {
        let mut result = 0;
        for j in 1..=(i as f64).sqrt() as i32 + 1 {
            if i % j == 0 {
                result += j;
                result += i / j;
            }
        }

        if result >= number {
            println!("{}", i);
            return;
        }

        i += 1;
    }
}
