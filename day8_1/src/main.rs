use std::fs;

enum State {
    Ok,
    Back,
    X(u8),
}

fn main() {
    let mut counter = 0;
    let mut state = State::Ok;
    for s in fs::read_to_string("../inputs/day8.txt")
        .unwrap()
        .trim()
        .split("\n")
    {
        let chars = s.chars().collect::<Vec<_>>();
        counter += chars.len() + 2;
        for ch in chars {
            match state {
                State::Ok => {
                    if ch == '\\' {
                        state = State::Back;
                    } else {
                        counter -= 1;
                    }
                }
                State::Back => {
                    if ch == 'x' {
                        state = State::X(2);
                    } else {
                        counter -= 1;
                        state = State::Ok;
                    }
                }
                State::X(x) => {
                    if x == 1 {
                        counter -= 1;
                        state = State::Ok;
                    } else {
                        state = State::X(x - 1);
                    }
                }
            }
        }
    }

    println!("{}", counter);
}
