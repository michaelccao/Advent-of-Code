use crate::helper::read_data;
use std::collections::{HashMap, HashSet};
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day19.txt");

    let (towels, designs): (Vec<&str>, Vec<&str>) = get_towels_and_designs(&data);

    let mut cache: HashMap<&str, u64> = HashMap::new();

    let mut p1: u64 = 0;
    let mut p2: u64 = 0;

    for design in designs {
        let poss = match_design(&towels, design, &mut cache);

        p1 += if poss > 0 {1} else {0};
        p2 += poss;
    }

    println!("{p1}\n{p2}");
}

fn get_towels_and_designs(data: &String) -> (Vec<&str>, Vec<&str>) {
    let lines: Vec<&str> = data
        .split('\n')
        .map(|line| line.trim())
        .collect::<Vec<&str>>();

    let towels: Vec<&str> = lines[0].split(", ").collect::<Vec<&str>>();

    let designs: Vec<&str> = lines[2..].to_vec();

    (towels, designs)
}

fn match_design<'a>(towels: &Vec<&str>, design: &'a str, cache: &mut HashMap<&'a str, u64>) -> u64 {

    if cache.contains_key(design) {
        return cache.get(design).unwrap().clone()
    }

    if design.len() == 0 {
        return 1
    }

    cache.insert(design, 0);

    for towel in towels {
        if design.starts_with(towel) {
            *cache.get_mut(design).unwrap() += match_design(towels, &design[towel.len()..], cache);
        }
    }

    *cache.get_mut(design).unwrap()
}
