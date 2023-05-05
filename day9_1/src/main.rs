use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    let mut location_counter = 0usize;
    let mut locations = HashMap::<String, usize>::new();
    let mut records = Vec::<(String, String, i32)>::new();
    for s in fs::read_to_string("../inputs/day9.txt")
        .unwrap()
        .trim()
        .lines()
    {
        let parts = s.split(" = ").collect::<Vec<_>>();
        let distance = parts[1].parse::<i32>().unwrap();
        let route = parts[0].split(" to ").collect::<Vec<_>>();
        records.push((route[0].to_string(), route[1].to_string(), distance));
        for location in route {
            if !locations.contains_key(location) {
                locations.insert(location.to_string(), location_counter);
                location_counter += 1;
            }
        }
    }

    let mut matrix = vec![vec![0; locations.len()]; locations.len()];
    for record in records {
        let i = locations[&record.0];
        let j = locations[&record.1];
        let distance = record.2;
        matrix[i][j] = distance;
        matrix[j][i] = distance;
    }

    let mut record = i32::MAX;
    let to_visit = (0..locations.len()).collect::<HashSet<_>>();
    for i in to_visit.iter() {
        let mut to_visit = to_visit.clone();
        to_visit.remove(&i);
        find_optimal(&matrix, to_visit, &mut record, 0, *i);
    }

    println!("{}", record);
}

fn find_optimal(
    matrix: &Vec<Vec<i32>>,
    to_visit: HashSet<usize>,
    record: &mut i32,
    current: i32,
    last: usize,
) {
    if to_visit.is_empty() {
        *record = (*record).min(current);
        return;
    }

    let mut possible_next = to_visit.clone();

    while !possible_next.is_empty() {
        let mut min_distance = i32::MAX;
        let mut min_index = 0usize;
        for i in possible_next.iter() {
            if matrix[last][*i] < min_distance {
                min_distance = matrix[last][*i];
                min_index = *i;
            }
        }

        let mut to_visit = to_visit.clone();
        to_visit.remove(&min_index);
        possible_next.remove(&min_index);

        if current + min_distance >= *record {
            return;
        }

        find_optimal(&matrix, to_visit, record, current + min_distance, min_index);
    }
}
