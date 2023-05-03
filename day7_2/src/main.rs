use std::{collections::HashMap, fs};

#[derive(Debug, Clone)]
enum Operation {
    Assign(String),
    And(String, String),
    Or(String, String),
    Not(String),
    Lshift(String, u16),
    Rshift(String, u16),
}

impl Operation {
    fn eval(&self, map: &mut HashMap<String, Operation>) -> u16 {
        match &self {
            Operation::Assign(key) => self.get_value(key.clone(), map),
            Operation::Not(key) => !self.get_value(key.clone(), map),
            Operation::And(key1, key2) => {
                self.get_value(key1.clone(), map) & self.get_value(key2.clone(), map)
            }
            Operation::Or(key1, key2) => {
                self.get_value(key1.clone(), map) | self.get_value(key2.clone(), map)
            }
            Operation::Lshift(key, shift) => self.get_value(key.clone(), map) << shift,
            Operation::Rshift(key, shift) => self.get_value(key.clone(), map) >> shift,
        }
    }

    fn get_value(&self, key: String, map: &mut HashMap<String, Operation>) -> u16 {
        match key.parse::<u16>() {
            Ok(value) => value,
            Err(_) => {
                let op = map.get(&key).unwrap().clone();
                let value = op.eval(map);
                map.insert(key, Operation::Assign(value.to_string()));
                value
            }
        }
    }
}

fn main() {
    let mut map: HashMap<String, Operation> = HashMap::new();
    let binding = fs::read_to_string("../inputs/day7.txt").unwrap();
    for s in binding.trim().split("\n") {
        let parts = s.split(" -> ").collect::<Vec<_>>();
        let name = parts[1];
        let operation: Operation;

        if let Some(_) = parts[0].find("NOT ") {
            operation = Operation::Not(parts[0][4..].to_string());
        } else if let Some(_) = parts[0].find(" AND ") {
            let components = parts[0].split(" AND ").collect::<Vec<&str>>();
            operation = Operation::And(components[0].to_string(), components[1].to_string());
        } else if let Some(_) = parts[0].find(" OR ") {
            let components = parts[0].split(" OR ").collect::<Vec<&str>>();
            operation = Operation::Or(components[0].to_string(), components[1].to_string());
        } else if let Some(_) = parts[0].find("LSHIFT") {
            let components = parts[0].split(" LSHIFT ").collect::<Vec<&str>>();
            operation = Operation::Lshift(
                components[0].to_string(),
                components[1].parse::<u16>().unwrap(),
            )
        } else if let Some(_) = parts[0].find("RSHIFT") {
            let components = parts[0].split(" RSHIFT ").collect::<Vec<&str>>();
            operation = Operation::Rshift(
                components[0].to_string(),
                components[1].parse::<u16>().unwrap(),
            )
        } else {
            operation = Operation::Assign(parts[0].to_string());
        }

        map.insert(name.to_string(), operation);
    }

    map.insert("b".to_string(), Operation::Assign("3176".to_string()));

    let op = map.get("a").unwrap().clone();
    println!("{}", op.eval(&mut map));
}
