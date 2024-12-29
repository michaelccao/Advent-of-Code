use crate::helper::read_data;
use regex::Regex;
use std::collections::HashSet;
use std::vec::Vec;

// This moron should've just used a JSON parsing crate

pub fn main() {
    let data: String = read_data("../Data/Day12.txt");

    let re: Regex = Regex::new(r"-?\d+").unwrap();

    let p1: i32 = re
        .find_iter(&data)
        .map(|m| m.as_str().parse::<i32>().unwrap())
        .sum();

    println!("{p1}");

    let red_objects = find_red_objects(&data);

    let red_total: i32 = red_objects
        .iter()
        .map(|object| {
            re.find_iter(*object)
                .map(|m| m.as_str().parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .sum();
    let p2: i32 = p1 - red_total;

    println!("{p2}");
}

fn find_red_objects(data: &String) -> Vec<&str> {
    let red_property_re: Regex = Regex::new(r#":"red""#).unwrap();
    let reds: Vec<usize> = red_property_re.find_iter(data).map(|m| m.start()).collect();

    let mut red_objects: HashSet<(usize, usize)> = HashSet::new();

    for pos in reds {
        let mut i: usize = pos;
        let mut j: usize = pos;

        let mut brackets: i32 = 0;

        loop {
            match &data[i..i + 1] {
                "{" => {
                    if brackets == 0 {
                        break;
                    } else {
                        brackets -= 1
                    }
                }
                "}" => brackets += 1,
                _ => {}
            }
            i -= 1;
        }

        loop {
            match &data[j..j + 1] {
                "}" => {
                    if brackets == 0 {
                        break;
                    } else {
                        brackets += 1
                    }
                }
                "{" => brackets -= 1,
                _ => {}
            }
            j += 1;
        }

        red_objects.insert((i, j + 1));
    }

    // Problem: Some red objects are sub-objects of
    // larger red objects
    // To avoid double counting, we remove any key
    // that has a wider key that encapsulates it

    let mut parent_red_objects: Vec<&str> = Vec::new();

    for k1 in &red_objects {
        let mut parent: bool = true;
        for k2 in &red_objects {
            if k1.0 >= k2.0 && k1.1 <= k2.1 && k1 != k2 {
                parent = false;
                break;
            }
        }
        if parent {
            parent_red_objects.push(&data[k1.0..k1.1]);
        }
    }

    parent_red_objects
}
