use std::fs;

fn main() {
    let j: serde_json::Value =
        serde_json::from_str(&fs::read_to_string("../inputs/day12.txt").unwrap()).unwrap();
    let mut result = 0;
    process_json(&j, &mut result);
    println!("{}", result);
}

fn process_json(j: &serde_json::Value, result: &mut i64) {
    if j.is_array() {
        let j = j.as_array().unwrap();
        for child in j {
            process_json(child, result);
        }
    } else if j.is_object() {
        let j = j.as_object().unwrap();
        let mut red = false;
        for value in j.values() {
            if value.is_string() && value.as_str().unwrap() == "red" {
                red = true;
                break;
            }
        }
        if !red {
            for value in j.values() {
                process_json(value, result);
            }
        }
    } else if j.is_number() {
        *result += j.as_i64().unwrap();
    }
}
