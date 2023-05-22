use std::fs;

fn main() {
    let number = fs::read_to_string("../inputs/day20.txt")
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();
    let mut i = 0;
    loop {
        let mut result = 0;
        let mut counter = 0;
        for j in 1..=i.min(50) {
            if i % j == 0 {
                result += i / j * 11;
                counter += 1;
            }

            if counter >= 50 {
                break;
            }
        }

        if result >= number {
            println!("{}", i);
            return;
        }

        i += 1;
    }
}
