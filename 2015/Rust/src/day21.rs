use crate::helper::read_data;
use std::collections::{HashMap, HashSet};
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day21.txt");
    let boss: Vec<u32> = data
        .lines()
        .map(|line| {
            line.trim().split(": ").collect::<Vec<&str>>()[1]
                .parse::<u32>()
                .unwrap()
        })
        .collect();

    let weapons: Vec<(u32, u32, u32)> =
        vec![(8, 4, 0), (10, 5, 0), (25, 6, 0), (40, 7, 0), (74, 8, 0)];

    let armors: Vec<(u32, u32, u32)> = vec![
        (0, 0, 0), // Option to not buy armor
        (13, 0, 1),
        (31, 0, 2),
        (53, 0, 3),
        (75, 0, 4),
        (102, 0, 5),
    ];

    let rings: Vec<(u32, u32, u32)> = vec![
        (0, 0, 0), // Option to have one or no rings
        (0, 0, 0),
        (25, 1, 0),
        (50, 2, 0),
        (100, 3, 0),
        (20, 0, 1),
        (40, 0, 2),
        (80, 0, 3),
    ];

    let equipment: Vec<&Vec<(u32, u32, u32)>> = vec![&weapons, &armors, &rings];

    let good_builds: HashMap<(u32, u32), u32> = get_builds(&equipment, false);
    let bad_builds: HashMap<(u32, u32), u32> = get_builds(&equipment, true);

    let p1 = good_builds
        .iter()
        .filter(|(build, _)| is_good_build(build, &boss))
        .map(|(_, cost)| *cost)
        .min()
        .unwrap();
    let p2 = bad_builds
        .iter()
        .filter(|(build, _)| !is_good_build(build, &boss))
        .map(|(_, cost)| *cost)
        .max()
        .unwrap();

    println!("{p1}\n{p2}");
}

fn get_builds(equipment: &Vec<&Vec<(u32, u32, u32)>>, part2: bool) -> HashMap<(u32, u32), u32> {
    let mut nodes: Vec<(usize, u32, u32, u32)> = vec![(0, 0, 0, 0)];

    let mut builds: HashMap<(u32, u32), u32> = HashMap::new();

    while nodes.len() > 0 {
        let (pointer, gold, attack, defense): (usize, u32, u32, u32) = nodes.pop().unwrap();

        if pointer == equipment.len() {
            if let Some(cost) = builds.get_mut(&(attack, defense)) {
                if !part2 && gold < *cost {
                    *cost = gold;
                } else if part2 && gold > *cost {
                    *cost = gold;
                }
            } else {
                builds.insert((attack, defense), gold);
            }
            continue;
        }

        let gear: &Vec<(u32, u32, u32)> = equipment[pointer];

        for i in 0..gear.len() - 1 {
            let (add_gold, add_atk, add_def): (u32, u32, u32) = gear[i];

            if pointer == equipment.len() - 1 {
                for j in i + 1..gear.len() {
                    let (add_gold2, add_atk2, add_def2): (u32, u32, u32) = gear[j];

                    nodes.push((
                        pointer + 1,
                        gold + add_gold + add_gold2,
                        attack + add_atk + add_atk2,
                        defense + add_def + add_def2,
                    ));
                }
            } else {
                nodes.push((
                    pointer + 1,
                    gold + add_gold,
                    attack + add_atk,
                    defense + add_def,
                ));
            }
        }
    }

    builds
}

fn is_good_build(build: &(u32, u32), boss: &Vec<u32>) -> bool {
    let boss_hp = boss[0];
    let mut boss_atk = boss[1];
    let boss_def = boss[2];

    let (mut atk, def) = *build;

    if atk <= boss_def {
        atk = boss_def + 1;
    }

    let boss_kill = boss_hp / (atk - boss_def)
        + if boss_hp % (atk - boss_def) == 0 {
            0
        } else {
            1
        };

    if def >= boss_atk {
        boss_atk = def + 1;
    }

    let we_die = 100 / (boss_atk - def) + if 100 % (boss_atk - def) == 0 { 0 } else { 1 };

    if boss_kill <= we_die {
        return true;
    } else {
        return false;
    }
}
