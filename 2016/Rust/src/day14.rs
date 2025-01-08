use crate::helper::read_data;
use md5::compute;
use std::collections::{HashMap, HashSet};
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day14.txt");

    let p1 = get_keys(&data, 64, 1)[63].0;

    let p2 = get_keys(&data, 64, 2017)[63].0;

    println!("{p1}\n{p2}");
}

fn get_keys(salt: &String, num_keys: usize, n: u32) -> Vec<(u32, String)> {
    let mut keys: Vec<(u32, String)> = Vec::new();

    let mut triples: HashMap<char, Vec<(u32, String)>> = HashMap::new();

    let mut index: u32 = 0;

    while keys.len() < num_keys {
        let pre_hash = format!("{salt}{index}");

        let hash = super_hash(pre_hash, n);

        let (triple, quints) = parse_hash(&hash);

        if let Some(t) = triple {
            if let Some(trips) = triples.get_mut(&t) {
                trips.push((index, hash));
            } else {
                triples.insert(t, vec![(index, hash)]);
            }
        }

        for q in quints {
            if let Some(trips) = triples.get_mut(&q) {
                let mut remaining: Vec<(u32, String)> = Vec::new();

                for (ind, hash) in trips {
                    if index - *ind <= 1000 && index - *ind > 0 {
                        keys.push((*ind, hash.clone()));
                    } else {
                        remaining.push((*ind, hash.clone()));
                    }
                }

                triples.insert(q, remaining);
            }
        }

        index += 1;
    }

    keys.sort_by_key(|t| t.0);

    keys
}

fn parse_hash(hash: &String) -> (Option<char>, HashSet<char>) {
    let mut quints: HashSet<char> = HashSet::new();
    let mut buffer: (char, u32) = (' ', 0);
    let mut triple: Option<char> = None;

    for c in hash.chars() {
        if c == buffer.0 {
            buffer.1 += 1;
        } else {
            buffer.0 = c;
            buffer.1 = 1;
        }

        if triple.is_none() && buffer.1 == 3 {
            triple = Some(buffer.0);
        }

        if buffer.1 == 5 {
            quints.insert(buffer.0);
        }
    }

    (triple, quints)
}

fn super_hash(pre_hash: String, n: u32) -> String {
    let mut hash = pre_hash;

    for _ in 0..n {
        hash = format!("{:x}", compute(hash));
    }

    hash
}
