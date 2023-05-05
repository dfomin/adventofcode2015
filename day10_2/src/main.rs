use std::fs;

fn main() {
    let binding = fs::read_to_string("../inputs/day10.txt").unwrap();
    let mut s = binding.trim().to_string();
    for _ in 0..50 {
        let mut prev_ch = ' ';
        let mut counter = 0;
        let mut next_s = String::new();
        for ch in s.chars() {
            if ch == prev_ch {
                counter += 1;
            } else {
                if prev_ch != ' ' {
                    next_s.push_str(&counter.to_string());
                    next_s.push(prev_ch);
                }
                counter = 1;
            }

            prev_ch = ch;
        }

        next_s.push_str(&counter.to_string());
        next_s.push(prev_ch);

        s = next_s;
    }

    println!("{}", s.len());
}
