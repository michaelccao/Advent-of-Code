use crate::helper::read_data;
use regex::Regex;
use std::collections::HashMap;
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day16.txt");

    let sue: HashMap<&str, u32> = HashMap::from([
        ("children", 3),
        ("cats", 7),
        ("samoyeds", 2),
        ("pomeranians", 3),
        ("akitas", 0),
        ("vizslas", 0),
        ("goldfish", 5),
        ("trees", 3),
        ("cars", 2),
        ("perfumes", 1),
    ]);

    let candidates: Vec<HashMap<&str, u32>> = get_candidates(&data);

    let mut p1: usize = candidates
        .iter()
        .enumerate()
        .map(|(sue_num, candidate)| {
            if is_match(candidate, &sue) {
                sue_num
            } else {
                0
            }
        })
        .sum();
    p1 += 1; // We start counting Sues from 1

    let mut p2: usize = candidates
        .iter()
        .enumerate()
        .map(|(sue_num, candidate)| {
            if is_match2(candidate, &sue) {
                sue_num
            } else {
                0
            }
        })
        .sum();
    p2 += 1; // We start counting Sues from 1

    println!("{p1}\n{p2}");
}

fn get_candidates(data: &String) -> Vec<HashMap<&str, u32>> {
    let mut candidates: Vec<HashMap<&str, u32>> = Vec::new();

    let re = Regex::new(r"(?<key>[a-z]*): (?<value>\d+)").unwrap();

    for line in data.lines() {
        let mut candidate: HashMap<&str, u32> = HashMap::new();

        for cap in re.captures_iter(line) {
            let key = cap.name("key").unwrap().as_str();
            let value: u32 = cap.name("value").unwrap().as_str().parse().unwrap();
            candidate.insert(key, value);
        }
        candidates.push(candidate);
    }

    candidates
}

fn is_match(candidate: &HashMap<&str, u32>, criteria: &HashMap<&str, u32>) -> bool {
    for (key, value) in candidate {
        if criteria[*key] != *value {
            return false;
        }
    }

    true
}

fn is_match2(candidate: &HashMap<&str, u32>, criteria: &HashMap<&str, u32>) -> bool {
    for (key, value) in candidate {
        match *key {
            "cats" | "trees" => {
                if criteria[*key] >= *value {
                    return false;
                }
            }
            "pomeranians" | "goldfish" => {
                if criteria[*key] <= *value {
                    return false;
                }
            }
            _ => {
                if criteria[*key] != *value {
                    return false;
                }
            }
        }
    }

    true
}
