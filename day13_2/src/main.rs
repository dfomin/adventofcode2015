use itertools::Itertools;
use std::{collections::HashMap, fs};

fn main() {
    let mut records = Vec::<(String, String, i32)>::new();
    let mut persons = HashMap::<String, usize>::new();
    let mut counter = 0;
    for s in fs::read_to_string("../inputs/day13.txt")
        .unwrap()
        .trim()
        .lines()
    {
        let parts = s
            .split(" happiness units by sitting next to ")
            .collect::<Vec<_>>();
        let second = parts[1][..parts[1].len() - 1].to_string();
        let parts = parts[0].split(" would ").collect::<Vec<_>>();
        let first = parts[0].to_string();
        let parts = parts[1].split(" ").collect::<Vec<_>>();
        let mut happiness = parts[1].parse::<i32>().unwrap();
        if parts[0] == "lose" {
            happiness = -happiness;
        }

        records.push((first.clone(), second.clone(), happiness));
        for person in vec![first, second] {
            if !persons.contains_key(&person) {
                persons.insert(person, counter);
                counter += 1;
            }
        }
    }

    persons.insert("ME".to_string(), counter);
    let mut matrix = vec![vec![0; persons.len()]; persons.len()];
    for record in records {
        let first = persons[&record.0];
        let second = persons[&record.1];
        matrix[first][second] = record.2;
    }

    let mut record = i32::MIN;
    for perm in (0..persons.len()).permutations(persons.len()) {
        let mut current = 0;
        for i in 0..perm.len() {
            current += matrix[perm[i]][perm[(i + 1) % perm.len()]];
            current += matrix[perm[(i + 1) % perm.len()]][perm[i]];
        }

        record = record.max(current);
    }

    println!("{}", record);
}
