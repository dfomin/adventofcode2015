use std::fs;

fn main() {
    let sizes = fs::read_to_string("../inputs/day17.txt")
        .unwrap()
        .trim()
        .lines()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let mut result = 0;
    let mut min_amount = sizes.len() as i32;
    process(150, &mut result, &mut min_amount, &sizes, 0, 0);

    println!("{}", result);
}

fn process(
    rest: i32,
    result: &mut i32,
    min_amount: &mut i32,
    sizes: &Vec<i32>,
    index: usize,
    cur_number: i32,
) {
    if rest == 0 {
        if cur_number < *min_amount {
            *min_amount = cur_number;
            *result = 1;
        } else if cur_number == *min_amount {
            *result += 1;
        }

        return;
    }

    if index >= sizes.len() {
        return;
    }

    if rest > 0 {
        for i in index..sizes.len() {
            process(
                rest - sizes[i],
                result,
                min_amount,
                sizes,
                i + 1,
                cur_number + 1,
            );
        }
    }
}
