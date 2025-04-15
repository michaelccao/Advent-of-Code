use crate::helper::read_data;
use std::collections::HashSet;
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day24.txt");

    let groups = get_groups(&data);

    let p1: u32 = fight(&groups, 0).1;

    println!("{p1}");

    let p2: u32 = minimum_immune_boost(&groups);

    println!("{p2}");
}

#[derive(Debug, Clone)]
struct Group {
    units: u32,
    hp: u32,
    attack: u32,
    attack_type: String,
    initiative: u32,
    weaknesses: HashSet<String>,
    immunities: HashSet<String>,
    army: String,
}

fn get_groups(data: &String) -> Vec<Group> {
    let mut groups: Vec<Group> = Vec::new();

    let mut target_army: &str = "";

    for line in data.lines() {
        if line.find("units").is_none() {
            target_army = line.split(":").next().unwrap();
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
            immunities: immunities,
            army: target_army.to_string(),
        };

        groups.push(group);
    }

    groups
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

fn fight(groups: &Vec<Group>, immune_boost: u32) -> (bool, u32) {
    let mut groups = groups.clone();

    let mut immune_system: u32 = 0;
    let mut infection: u32 = 0;

    for i in 0..groups.len() {
        let mut group = groups[i].clone();
        if group.army == "Immune System" {
            group.attack += immune_boost;
            groups[i] = group;
            immune_system += 1;
        } else if group.army == "Infection" {
            infection += 1;
        }
    }

    while immune_system > 0 && infection > 0 {
        // Target Selection Phase
        groups.sort_by_key(|g| (g.units*g.attack, g.initiative));
        groups.reverse();

        let mut targets: Vec<Option<usize>> = vec![None; groups.len()];
        let mut targetted: Vec<bool> = vec![false; groups.len()];

        for (i, attacker) in groups.iter().enumerate() {
            if attacker.units == 0 {
                continue;
            }
            let mut best_opponent: (u32, u32, u32, usize) = (0, 0, 0, i);

            for (j, defender) in groups.iter().enumerate() {
                if attacker.army == defender.army || targetted[j] || defender.units == 0 {
                    continue;
                }
                let opponent: (u32, u32, u32, usize) = (attacker.predict_damage(defender), defender.units*defender.attack, defender.initiative, j);

                if opponent > best_opponent {
                    best_opponent = opponent
                }
            }

            if best_opponent.0 > 0 {
                targets[i] = Some(best_opponent.3);
                targetted[best_opponent.3] = true;
            }
        }    

        let mut attack_order: Vec<usize> = (0..groups.len()).collect();

        attack_order.sort_by_key(|&i| groups[i].initiative);
        attack_order.reverse();

        let mut total_unit_damage: u32 = 0;

        for i in attack_order {
            let attacker = groups[i].clone();
            if attacker.units == 0 {
                continue;
            }

            if let Some(j) = targets[i] {
                let mut defender = groups[j].clone();

                let units_lost = attacker.predict_damage(&defender) / defender.hp;

                if units_lost < defender.units {
                    defender.units -= units_lost;
                    total_unit_damage += units_lost;
                } else {
                    total_unit_damage += defender.units;
                    defender.units = 0;
                    

                    if defender.army == "Immune System" {
                        immune_system -= 1;
                    } else if defender.army == "Infection" {
                        infection -= 1;
                    }
                }

                groups[j] = defender;
            }
        }

        if total_unit_damage == 0 {
            return (false, 0);
        }
    }

    (infection == 0, groups.iter().map(|g| g.units).sum())
}

fn minimum_immune_boost(groups: &Vec<Group>) -> u32 {

    let mut lb: u32 = 0;
    let mut ub: u32 = 10000;

    while lb < ub {
        let boost: u32 = (lb + ub)/2;

        if fight(groups, boost).0 {
            ub = boost;
        } else {
            lb = boost+1;
        }
    }

    fight(groups, lb).1

}