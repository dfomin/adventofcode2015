use std::fs;

enum Instruction {
    Half(usize),
    Triple(usize),
    Increment(usize),
    Jump(i32),
    JumpIfEven(usize, i32),
    JumpIfOne(usize, i32),
}

fn main() {
    let mut instructions = vec![];
    for line in fs::read_to_string("../inputs/day23.txt")
        .unwrap()
        .trim()
        .lines()
    {
        let parts = line.split(" ").collect::<Vec<_>>();
        instructions.push(match parts[0] {
            "hlf" => Instruction::Half(parse_register(parts[1])),
            "tpl" => Instruction::Triple(parse_register(parts[1])),
            "inc" => Instruction::Increment(parse_register(parts[1])),
            "jmp" => Instruction::Jump(parts[1].parse().unwrap()),
            "jie" => Instruction::JumpIfEven(parse_register(parts[1]), parts[2].parse().unwrap()),
            "jio" => Instruction::JumpIfOne(parse_register(parts[1]), parts[2].parse().unwrap()),
            _ => panic!("Unexpected command"),
        });
    }

    let mut index = 0;
    let mut registers = vec![0, 0];
    while index < instructions.len() {
        let command = &instructions[index];
        match command {
            Instruction::Half(r) => {
                registers[*r] /= 2;
                index += 1;
            }
            Instruction::Triple(r) => {
                registers[*r] *= 3;
                index += 1;
            }
            Instruction::Increment(r) => {
                registers[*r] += 1;
                index += 1;
            }
            Instruction::Jump(i) => {
                index = (index as i32 + i) as usize;
            }
            Instruction::JumpIfEven(r, i) => {
                if registers[*r] % 2 == 0 {
                    index = (index as i32 + i) as usize;
                } else {
                    index += 1;
                }
            }
            Instruction::JumpIfOne(r, i) => {
                if registers[*r] == 1 {
                    index = (index as i32 + i) as usize;
                } else {
                    index += 1;
                }
            }
        }
    }

    println!("{}", registers[1]);
}

fn parse_register(s: &str) -> usize {
    if s == "a" || s == "a," {
        0
    } else {
        1
    }
}
