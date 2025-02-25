use crate::helper::read_data;
use std::collections::HashMap;
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day24.txt");

    let parts: HashMap<u32, HashMap<(u32, u32), bool>> = get_parts(&data);

    let p1: u32 = strongest_bridge(&parts);
    let p2: u32 = strongest_longest_bridge(&parts).1;

    println!("{p1}\n{p2}");
}

fn get_parts(data: &String) -> HashMap<u32, HashMap<(u32, u32), bool>> {
    let mut parts: HashMap<u32, HashMap<(u32, u32), bool>> = HashMap::new();

    for line in data.lines() {
        let mut line = line.trim().split('/');
        let pin1: u32 = line.next().unwrap().parse().unwrap();
        let pin2: u32 = line.next().unwrap().parse().unwrap();

        if let Some(subparts) = parts.get_mut(&pin1) {
            subparts.insert((pin1, pin2), false);
        } else {
            parts.insert(pin1, HashMap::from([((pin1, pin2), false)]));
        }

        if let Some(subparts) = parts.get_mut(&pin2) {
            subparts.insert((pin2, pin1), false);
        } else {
            parts.insert(pin2, HashMap::from([((pin2, pin1), false)]));
        }
    }

    parts
}

fn strongest_bridge(parts: &HashMap<u32, HashMap<(u32, u32), bool>>) -> u32 {
    let mut nodes: Vec<(u32, u32, HashMap<u32, HashMap<(u32, u32), bool>>)> =
        vec![(0, 0, parts.clone())];

    let mut strongest: u32 = 0;

    while nodes.len() > 0 {
        let (dangling, strength, parts) = nodes.pop().unwrap();

        if strength > strongest {
            strongest = strength;
        }

        if let Some(next_parts) = parts.get(&dangling) {
            for (&(pin1, pin2), &used) in next_parts {
                if used {
                    continue;
                };

                let mut parts2: HashMap<u32, HashMap<(u32, u32), bool>> = parts.clone();

                parts2.get_mut(&pin1).unwrap().insert((pin1, pin2), true);
                parts2.get_mut(&pin2).unwrap().insert((pin2, pin1), true);

                let strength2: u32 = strength + pin1 + pin2;

                nodes.push((pin2, strength2, parts2));
            }
        }
    }

    strongest
}

fn strongest_longest_bridge(parts: &HashMap<u32, HashMap<(u32, u32), bool>>) -> (u32, u32) {
    let mut nodes: Vec<(u32, u32, u32, HashMap<u32, HashMap<(u32, u32), bool>>)> =
        vec![(0, 0, 0, parts.clone())];

    let mut longest: (u32, u32) = (0, 0);

    while nodes.len() > 0 {
        let (dangling, strength, length, parts) = nodes.pop().unwrap();

        if (length, strength) > longest {
            longest = (length, strength);
        }

        if let Some(next_parts) = parts.get(&dangling) {
            for (&(pin1, pin2), &used) in next_parts {
                if used {
                    continue;
                };

                let mut parts2: HashMap<u32, HashMap<(u32, u32), bool>> = parts.clone();

                parts2.get_mut(&pin1).unwrap().insert((pin1, pin2), true);
                parts2.get_mut(&pin2).unwrap().insert((pin2, pin1), true);

                let strength2: u32 = strength + pin1 + pin2;

                nodes.push((pin2, strength2, length + 1, parts2));
            }
        }
    }

    longest
}
