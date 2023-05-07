use std::fs;

fn main() {
    let mut max = 0;
    let full_time = 2503;
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

        let cycle = time + rest;
        let distance = (full_time / cycle) * time * speed + speed * (time.min(full_time % cycle));
        max = max.max(distance);
    }

    println!("{}", max);
}
