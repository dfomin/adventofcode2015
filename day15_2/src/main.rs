use std::fs;

fn main() {
    let mut data = Vec::<Vec<i32>>::new();
    for s in fs::read_to_string("../inputs/day15.txt")
        .unwrap()
        .trim()
        .lines()
    {
        let parts = s.split(",").collect::<Vec<_>>();
        let capacity = parts[0].split(" ").collect::<Vec<_>>()[2]
            .parse::<i32>()
            .unwrap();
        let durability = parts[1].split(" ").collect::<Vec<_>>()[2]
            .parse::<i32>()
            .unwrap();
        let flavor = parts[2].split(" ").collect::<Vec<_>>()[2]
            .parse::<i32>()
            .unwrap();
        let texture = parts[3].split(" ").collect::<Vec<_>>()[2]
            .parse::<i32>()
            .unwrap();
        let calories = parts[4].split(" ").collect::<Vec<_>>()[2]
            .parse::<i32>()
            .unwrap();

        data.push(vec![capacity, durability, flavor, texture, calories]);
    }

    let mut max = 0;
    calc(&data, vec![0; data.len()], 100, 0, &mut max);

    println!("{}", max);
}

fn calc(data: &Vec<Vec<i32>>, cur: Vec<i32>, left: usize, index: usize, max: &mut i32) {
    if index == data.len() {
        let mut values = vec![0; data[0].len()];
        for i in 0..values.len() {
            for (j, d) in data.iter().enumerate() {
                values[i] += d[i] * cur[j];
            }
        }

        if values[values.len() - 1] != 500 {
            return;
        }

        let mut value = 1;
        for v in &values[..values.len() - 1] {
            value *= (*v).max(0);
        }

        *max = (*max).max(value);
    } else if index == data.len() - 1 {
        let mut cur = cur.clone();
        cur[index] += left as i32;
        calc(data, cur, 0, index + 1, max);
    } else {
        for i in 0..left {
            let mut cur = cur.clone();
            cur[index] += i as i32;
            calc(data, cur, left - i, index + 1, max);
        }
    }
}
