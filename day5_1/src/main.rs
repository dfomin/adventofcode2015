use std::fs;

fn main() {
    let mut counter = 0;
    for s in fs::read_to_string("../inputs/day5.txt")
        .unwrap()
        .split("\n")
    {
        let mut vowels_count = 0;
        let mut prev_char = ' ';
        let mut has_double = false;
        let mut disallowed = false;
        for ch in s.chars() {
            match ch {
                'a' | 'e' | 'i' | 'o' | 'u' => vowels_count += 1,
                _ => (),
            }

            has_double |= prev_char == ch;

            match &format!("{}{}", prev_char, ch)[..] {
                "ab" | "cd" | "pq" | "xy" => {
                    disallowed = true;
                    break;
                }
                _ => (),
            }

            prev_char = ch;
        }

        if vowels_count >= 3 && has_double && !disallowed {
            counter += 1;
        }
    }

    println!("{}", counter);
}
