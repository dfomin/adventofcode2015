use std::fs;

#[derive(Clone, Debug)]
struct Group {
    sum: u64,
    len: usize,
    prod: u64,
}

fn main() {
    let weights = fs::read_to_string("../inputs/day24.txt")
        .unwrap()
        .trim()
        .lines()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let weights = weights.into_iter().rev().collect::<Vec<_>>();
    let sum: u64 = weights.iter().sum();
    let mut result = (weights.len(), u64::MAX);
    parts(
        vec![
            Group {
                sum: 0,
                len: 0,
                prod: 1,
            },
            Group {
                sum: 0,
                len: 0,
                prod: 1,
            },
            Group {
                sum: 0,
                len: 0,
                prod: 1,
            },
            Group {
                sum: 0,
                len: 0,
                prod: 1,
            },
        ],
        sum / 4,
        &mut result,
        &weights,
        0,
    );

    println!("{}", result.1);
}

fn parts(
    groups: Vec<Group>,
    target: u64,
    result: &mut (usize, u64),
    weights: &Vec<u64>,
    index: usize,
) {
    if index == weights.len() {
        let group = &groups[0];
        if group.len < (*result).0 {
            *result = (group.len, group.prod);
        } else if group.len == (*result).0 {
            (*result).1 = (*result).1.min(group.prod);
        }
    } else {
        if groups[0].len > (*result).0 {
            return;
        }
        for i in 0..4 {
            if groups[i].sum + weights[index] <= target {
                let mut groups = groups.clone();
                groups[i].sum += weights[index];
                groups[i].len += 1;
                if i == 0 {
                    groups[i].prod *= weights[index];
                }
                parts(groups, target, result, weights, index + 1);
            }
        }
    }
}
