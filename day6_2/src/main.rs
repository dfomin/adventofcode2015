use std::fs;

enum Operation {
    TurnOn,
    TurnOff,
    Toggle,
}

fn main() {
    let mut field = vec![vec![0; 1000]; 1000];
    for s in fs::read_to_string("../inputs/day6.txt")
        .unwrap()
        .trim()
        .split("\n")
    {
        let parts: Vec<&str> = s.split(" through ").collect();
        let start = parts[0]
            .split(" ")
            .collect::<Vec<_>>()
            .last()
            .unwrap()
            .split(",")
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let end = parts[1]
            .split(",")
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let operation = match &s[..7] {
            "turn on" => Operation::TurnOn,
            "turn of" => Operation::TurnOff,
            "toggle " => Operation::Toggle,
            _ => panic!("Unexpected input string"),
        };

        for i in start[0]..=end[0] {
            for j in start[1]..=end[1] {
                match operation {
                    Operation::TurnOn => field[i][j] += 1,
                    Operation::TurnOff => field[i][j] = (field[i][j] - 1).max(0),
                    Operation::Toggle => field[i][j] += 2,
                }
            }
        }
    }

    let mut counter = 0;
    for row in field.iter() {
        for lamp in row.iter() {
            counter += *lamp;
        }
    }

    println!("{counter}");
}
