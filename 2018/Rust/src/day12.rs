use crate::helper::read_data;
use std::collections::{HashMap, VecDeque};
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day12.txt");

    let (plants, rules) = get_plants(&data);

    let p1: i64 = n_grow_plants(&plants, &rules, 20);
    let p2: i64 = n_grow_plants(&plants, &rules, 50000000000);

    println!("{p1}\n{p2}");
}

fn get_plants(data: &String) -> (VecDeque<bool>, HashMap<VecDeque<bool>, bool>) {
    let lines: Vec<Vec<&str>> = data
        .lines()
        .map(|line| line.trim().split_whitespace().collect())
        .collect();

    let plants: VecDeque<bool> = lines[0][2].chars().map(|c| c == '#').collect();

    let mut rules: HashMap<VecDeque<bool>, bool> = HashMap::new();

    for i in 2..lines.len() {
        let pattern: VecDeque<bool> = lines[i][0].chars().map(|c| c == '#').collect();
        let res: bool = lines[i][2] == "#";

        rules.insert(pattern, res);
    }

    // Notably [False;5] => False is a rule
    // This means if a pot is outside the current boundary
    // it can only grow a plant if it's at most 2 away from a plant
    // This means boundary only expands if there's a plant within the
    // first two or last two pots

    (plants, rules)
}

fn grow_plants(
    plants: &VecDeque<bool>,
    rules: &HashMap<VecDeque<bool>, bool>,
) -> (VecDeque<bool>, i64) {
    let mut plants2: VecDeque<bool> = VecDeque::new();
    let mut offset: i64 = 0;

    let mut buffer: VecDeque<bool> = VecDeque::from([false; 5]);

    for i in 0..plants.len() {
        buffer.pop_front();
        buffer.push_back(plants[i]);

        let new_plant: bool = *rules.get(&buffer).unwrap_or(&false);

        match (i, new_plant, offset) {
            (0, true, 0) => {
                plants2.push_back(new_plant);
                offset = -2;
            }
            (0, false, 0) => {}
            (1, true, 0) => {
                plants2.push_back(new_plant);
                offset = -1;
            }
            (1, true, 2) => {
                plants2.push_back(new_plant);
            }
            (1, false, 0) => {}
            (1, false, 2) => {
                plants2.push_back(new_plant);
            }
            _ => {
                plants2.push_back(new_plant);
            }
        }
    }

    // Last two pots + two more pots past right boundary
    for _ in 0..4 {
        buffer.pop_front();
        buffer.push_back(false);

        let new_plant: bool = *rules.get(&buffer).unwrap_or(&false);

        plants2.push_back(new_plant)
    }

    // Trim falses
    while !plants2[0] {
        plants2.pop_front();
        offset += 1;
    }

    while !plants2[plants2.len() - 1] {
        plants2.pop_back();
    }

    (plants2, offset)
}

fn n_grow_plants(plants: &VecDeque<bool>, rules: &HashMap<VecDeque<bool>, bool>, n: i64) -> i64 {
    let mut plants: VecDeque<bool> = plants.clone();
    let mut offset: i64 = 0;

    let mut plant_cycles: HashMap<VecDeque<bool>, usize> = HashMap::new();
    let mut plant_diary: Vec<(VecDeque<bool>, i64)> = Vec::new();

    for i in 0..n {
        let (plants2, d_offset) = grow_plants(&plants, rules);

        plants = plants2;
        offset += d_offset;

        plant_diary.push((plants.clone(), offset));

        // We noticed at some point the relevant number of pots
        // Remained fixed at 138 while increasing the offset only by 1
        // println!("{:?}, {}", plants.len(), offset);

        if let Some(prev_day) = plant_cycles.insert(plants.clone(), i as usize) {
            let cycle_length: i64 = i - prev_day as i64;

            let target_day: usize = prev_day + ((n - i) % cycle_length) as usize;

            plants = plant_diary[target_day].0.clone();
            offset += n - 1 - i;

            return plants
                .iter()
                .enumerate()
                .map(|(i, &plant)| if plant { i as i64 + offset } else { 0 })
                .sum::<i64>();
        }
    }

    plants
        .iter()
        .enumerate()
        .map(|(i, &plant)| if plant { i as i64 + offset } else { 0 })
        .sum::<i64>()
}
