use crate::helper::read_data;
use std::collections::{HashMap, HashSet};

pub fn main() {
    let data: String = read_data("../Data/Day06.txt");

    let orbits: HashMap<&str, HashSet<&str>> = get_orbits(&data);

    let mut cache: HashMap<&str, u32> = HashMap::new();

    let p1: u32 = orbits
        .iter()
        .map(|(&k, _)| count_orbits(&orbits, k, &mut cache))
        .sum();

    println!("{p1}");

    let p2: u32 = shortest_path("YOU", "SAN", &orbits);

    println!("{p2}");
}

fn get_orbits(data: &String) -> HashMap<&str, HashSet<&str>> {
    let mut orbits: HashMap<&str, HashSet<&str>> = HashMap::new();

    for line in data.lines() {
        let line: Vec<&str> = line.split(")").collect();

        if let Some(orbs) = orbits.get_mut(line[0]) {
            orbs.insert(line[1]);
        } else {
            orbits.insert(line[0], HashSet::from([line[1]]));
        }
    }

    orbits
}

fn count_orbits<'a>(
    orbits: &HashMap<&'a str, HashSet<&'a str>>,
    body: &'a str,
    cache: &mut HashMap<&'a str, u32>,
) -> u32 {
    if !orbits.contains_key(body) {
        return 0;
    } else if cache.contains_key(body) {
        return cache[body];
    } else {
        let mut total: u32 = 0;
        for &orbitting in &orbits[body] {
            total += 1 + count_orbits(orbits, orbitting, cache);
        }

        cache.insert(body, total);

        return cache[body];
    }
}

fn shortest_path(start: &str, end: &str, orbits: &HashMap<&str, HashSet<&str>>) -> u32 {
    let mut connections = orbits.clone();

    for (&a, bodies) in orbits {
        for &body in bodies {
            if let Some(orbs) = connections.get_mut(body) {
                orbs.insert(a);
            } else {
                connections.insert(body, HashSet::from([a]));
            }
        }
    }

    let mut nodes: Vec<(&str, u32)> = vec![(start, 0)];
    let mut shortest: u32 = u32::MAX;
    let mut visited: HashMap<&str, u32> = HashMap::new();
    visited.insert(start, 0);

    while nodes.len() > 0 {
        let (body, steps) = nodes.pop().unwrap();

        if steps >= shortest {
            continue;
        } else if body == end {
            shortest = shortest.min(steps);
            continue;
        }

        for &orb in &connections[&body] {
            if !visited.contains_key(orb) || steps + 1 < visited[orb] {
                visited.insert(orb, steps + 1);
                nodes.push((orb, steps + 1));
            }
        }
    }

    shortest - 2
}
