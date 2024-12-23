use crate::helper::read_data;
use std::collections::{HashMap, HashSet};
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day23.txt");
    let connections: HashMap<&str, HashSet<&str>> = get_connections(&data);

    let p1: usize = find_3t_parties(&connections).len();

    let p2: String = connections
        .keys()
        .map(|pc: &&str| find_max_clique(pc, &connections))
        .max_by_key(|party: &String| party.len())
        .unwrap();

    println!("{p1}\n{p2}");
}

fn get_connections(data: &String) -> HashMap<&str, HashSet<&str>> {
    let mut connections: HashMap<&str, HashSet<&str>> = HashMap::new();

    for line in data.lines() {
        let connection: Vec<&str> = line.split('-').collect();

        let (p1, p2): (&str, &str) = (connection[0], connection[1]);

        if connections.contains_key(p1) {
            connections.get_mut(p1).unwrap().insert(p2);
        } else {
            connections.insert(p1, HashSet::from([p2]));
        }

        if connections.contains_key(p2) {
            connections.get_mut(p2).unwrap().insert(p1);
        } else {
            connections.insert(p2, HashSet::from([p1]));
        }
    }

    connections
}

fn find_3t_parties<'a>(
    connections: &'a HashMap<&'a str, HashSet<&'a str>>,
) -> HashSet<(&'a str, &'a str, &'a str)> {
    let mut parties: HashSet<(&str, &str, &str)> = HashSet::new();

    for p1 in connections.keys() {
        if p1.starts_with("t") {
            for p2 in &connections[p1] {
                for p3 in &connections[p2] {
                    if connections[p1].contains(p3) {
                        let mut party: Vec<&str> = vec![*p1, *p2, *p3];
                        party.sort();
                        parties.insert((party[0], party[1], party[2]));
                    }
                }
            }
        }
    }

    return parties;
}

fn find_max_clique(pc: &str, connections: &HashMap<&str, HashSet<&str>>) -> String {
    let mut clique: Vec<&str> = vec![pc];
    let mut candidates: HashSet<&str> = connections[pc].clone();

    while candidates.len() > 0 {
        let next_pc = candidates
            .iter()
            .max_by_key(|pc| {
                candidates
                    .intersection(&connections[*pc])
                    .collect::<HashSet<_>>()
                    .len()
            })
            .unwrap();
        clique.push(next_pc);

        candidates = candidates
            .intersection(&connections[next_pc])
            .cloned()
            .collect::<HashSet<_>>();
    }

    clique.sort();

    clique.join(",")
}
