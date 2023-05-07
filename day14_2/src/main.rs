use std::fs;

fn main() {
    let mut data = Vec::<(i32, i32, i32, i32, i32)>::new();
    for s in fs::read_to_string("../inputs/day14.txt")
        .unwrap()
        .trim()
        .lines()
    {
        let parts = s.split(" can fly ").collect::<Vec<_>>();
        let parts = parts[1].split(" km/s for ").collect::<Vec<_>>();
        let speed = parts[0].parse::<i32>().unwrap();
        let parts = parts[1]
            .split(" seconds, but then must rest for ")
            .collect::<Vec<_>>();
        let time = parts[0].parse::<i32>().unwrap();
        let parts = parts[1].split(" ").collect::<Vec<_>>();
        let rest = parts[0].parse::<i32>().unwrap();

        data.push((speed, time, rest, 0, 0));
    }

    let full_time = 2503;
    for i in 0..full_time {
        for reindeer in data.iter_mut() {
            let (speed, time, rest, _, _) = reindeer;
            if i % (*time + *rest) < *time {
                reindeer.3 += *speed;
            }
        }

        let mut max = 0;
        for reindeer in data.iter() {
            max = max.max(reindeer.3);
        }

        for reindeer in data.iter_mut() {
            if reindeer.3 == max {
                reindeer.4 += 1;
            }
        }
    }

    let mut max = 0;
    for reindeer in data {
        max = max.max(reindeer.4);
    }

    println!("{}", max);
}
