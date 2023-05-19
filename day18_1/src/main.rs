use std::fs;

const SIZE: usize = 100;
const STEPS: usize = 100;

fn main() {
    let mut field = vec![vec![vec![0; SIZE + 2]; SIZE + 2]; 2];
    for (i, line) in fs::read_to_string("../inputs/day18.txt")
        .unwrap()
        .trim()
        .lines()
        .enumerate()
    {
        for (j, ch) in line.chars().enumerate() {
            field[0][i + 1][j + 1] = if ch == '#' { 1 } else { 0 };
        }
    }

    for step in 0..STEPS {
        for i in 1..SIZE + 1 {
            for j in 1..SIZE + 1 {
                field[(step + 1) % 2][i][j] = new_value(&field[step % 2], i, j);
            }
        }
    }

    println!("{}", sum(&field[STEPS % 2]));
}

fn new_value(field: &Vec<Vec<i32>>, i: usize, j: usize) -> i32 {
    let mut result = -field[i][j];
    for shift_y in 0..3 {
        for shift_x in 0..3 {
            result += field[i + shift_y - 1][j + shift_x - 1];
        }
    }

    if field[i][j] == 1 {
        return if result == 2 || result == 3 { 1 } else { 0 };
    } else {
        return if result == 3 { 1 } else { 0 };
    }
}

fn sum(field: &Vec<Vec<i32>>) -> i32 {
    let mut sum = 0;
    for i in 1..SIZE + 1 {
        for j in 1..SIZE + 1 {
            sum += field[i][j];
        }
    }

    return sum;
}
