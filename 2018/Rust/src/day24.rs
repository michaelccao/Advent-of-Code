use crate::helper::read_data;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day24.txt");

    get_groups(&data);
}

#[derive(Debug)]
struct Group {
    units: u32,
    hp: u32,
    attack: u32,
    attack_type: String,
    initiative: u32,
    weaknesses: HashSet<String>,
    immunities: HashSet<String>,
}

fn get_groups(data: &String) {
    let mut immune_army: Vec<Group> = Vec::new();
    let mut infect_army: Vec<Group> = Vec::new();

    let mut target_army: &str = "";

    for line in data.lines() {
        if line.find("units").is_none() {
            target_army = line;
            continue;
        }
        let units: Vec<&str> = line.split(" hit points").next().unwrap().split_whitespace().collect();
        let num_units: u32 = units[0].parse().unwrap();
        let hit_points: u32 = units.last().unwrap().parse().unwrap();

        let attack: Vec<&str> = line.split("with an attack that does ").last().unwrap().split_whitespace().collect();
        let attack_power: u32 = attack[0].parse().unwrap();
        let attack_type: String = attack[1].to_string();
        let initiative: u32 = attack.last().unwrap().parse().unwrap();

        let mut weaknesses: HashSet<String> = HashSet::new();
        let mut immunities: HashSet<String> = HashSet::new();

        let immune: Vec<&str> = line.split("immune to ").collect();
        if immune.len() == 2 {
            let immune = immune[1];

            let terminator: usize = if let Some(t) = immune.find(";") {t} else {immune.find(")").unwrap()};

            let immune = immune[0..terminator].split(", ");

            for im in immune {
                immunities.insert(im.to_string());
            }
        }

        let weak: Vec<&str> = line.split("weak to ").collect();
        if weak.len() == 2 {
            let weak = weak[1];

            let terminator: usize = if let Some(t) = weak.find(";") {t} else {weak.find(")").unwrap()};

            let weak = weak[0..terminator].split(", ");

            for w in weak {
                weaknesses.insert(w.to_string());
            }
        }

        let group = Group {
            units: num_units,
            hp: hit_points,
            attack: attack_power,
            attack_type: attack_type,
            initiative: initiative,
            weaknesses: weaknesses,
            immunities: immunities
        };

        if target_army == "Immune System:" {
            immune_army.push(group);
        } else if target_army == "Infection:" {
            infect_army.push(group);
        }
        
    }

}

impl Group {
    fn predict_damage(&self, other: &Group) -> u32 {
        let effective_power = self.attack * self.units;

        if other.immunities.contains(&self.attack_type) {
            0
        } else if other.weaknesses.contains(&self.attack_type) {
            2*effective_power
        } else {
            effective_power
        }
    }
}