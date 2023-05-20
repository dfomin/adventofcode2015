use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    let mut map = HashMap::new();
    let mut original = vec![];
    let mut reading_map = true;
    for line in fs::read_to_string("../inputs/day19.txt")
        .unwrap()
        .trim()
        .lines()
    {
        if line.is_empty() {
            reading_map = false;
            continue;
        }

        if reading_map {
            let parts = line.split(" => ").collect::<Vec<_>>();
            map.entry(parts[0].to_string())
                .or_insert(vec![])
                .push(parts[1].to_string());
        } else {
            original = line.chars().collect::<Vec<_>>();
        }
    }

    let mut molecules = HashSet::new();
    for i in 0..original.len() {
        if original[i].is_lowercase() {
            continue;
        }

        let mut j = i + 1;
        while j < original.len() && original[j].is_lowercase() {
            j += 1;
        }

        process(&mut molecules, &original, &map, i, j);
    }

    println!("{}", molecules.len());
}

fn process(
    molecules: &mut HashSet<String>,
    original: &Vec<char>,
    map: &HashMap<String, Vec<String>>,
    i: usize,
    j: usize,
) {
    let key = original[i..j].iter().collect::<String>();
    if let Some(values) = map.get(&key) {
        for value in values {
            let mut new_molecule = original[..i].iter().collect::<String>();
            new_molecule.push_str(value);
            new_molecule.push_str(&original[j..].iter().collect::<String>());
            molecules.insert(new_molecule);
        }
    }
}
