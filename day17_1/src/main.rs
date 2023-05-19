use std::fs;

fn main() {
    let sizes = fs::read_to_string("../inputs/day17.txt")
        .unwrap()
        .trim()
        .lines()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let mut result = 0;
    process(150, &mut result, &sizes, 0);

    println!("{}", result);
}

fn process(rest: i32, result: &mut i32, sizes: &Vec<i32>, index: usize) {
    if rest == 0 {
        *result += 1;
    }

    if index >= sizes.len() {
        return;
    }

    if rest > 0 {
        for i in index..sizes.len() {
            process(rest - sizes[i], result, sizes, i + 1);
        }
    }
}
