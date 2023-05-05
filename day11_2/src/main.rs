use std::fs;

fn main() {
    let mut pass: Vec<u8> = vec![];
    for ch in fs::read_to_string("../inputs/day11.txt")
        .unwrap()
        .trim()
        .chars()
    {
        pass.push(ch as u8 - 'a' as u8);
    }

    let mut counter = 0;
    loop {
        step(&mut pass);
        if check(&pass) {
            counter += 1;
        }

        if counter == 2 {
            break;
        }
    }

    println!(
        "{}",
        pass.into_iter()
            .map(|x| char::from_u32(x as u32 + 'a' as u32).unwrap())
            .into_iter()
            .collect::<String>()
    );
}

fn step(pass: &mut Vec<u8>) {
    for i in (0..pass.len()).rev() {
        pass[i] += 1;
        if pass[i] == 26 {
            pass[i] = 0;
        } else {
            break;
        }
    }
}

fn check(pass: &Vec<u8>) -> bool {
    check_straight(pass) && check_no_letters(pass) && check_pairs(pass)
}

fn check_straight(pass: &Vec<u8>) -> bool {
    for i in 2..pass.len() {
        if pass[i] == pass[i - 1] + 1 && pass[i] == pass[i - 2] + 2 {
            return true;
        }
    }

    return false;
}

fn check_no_letters(pass: &Vec<u8>) -> bool {
    for ch in pass {
        if *ch == 'i' as u8 || *ch == 'o' as u8 || *ch == 'l' as u8 {
            return false;
        }
    }

    return true;
}

fn check_pairs(pass: &Vec<u8>) -> bool {
    let mut i = 1;
    let mut pairs = 0;
    while i < pass.len() {
        if pass[i - 1] == pass[i] {
            pairs += 1;
            i += 1;
        }

        i += 1;
    }

    return pairs >= 2;
}
