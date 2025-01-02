use crate::helper::read_data;
use std::collections::{HashMap, HashSet, VecDeque};
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day19.txt");

    let (start, replacements): (&str, HashMap<&str, Vec<&str>>) = setup_machine(&data);

    let new_molecules: HashSet<String> = find_new_molecules(start, &replacements);

    let p1: usize = new_molecules.len();

    let reversions = get_reversion(&replacements);

    let p2: u32 = generate_molecule(String::from(start), String::from("e"), &reversions);

    println!("{p1}\n{p2}");
}

fn setup_machine(data: &String) -> (&str, HashMap<&str, Vec<&str>>) {
    let mut replacements: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut start: &str = "";

    for line in data.lines() {
        if line.trim() == "" {
            continue;
        }

        let replacement: Vec<&str> = line.split(" => ").collect();

        if replacement.len() == 2 {
            if let Some(choices) = replacements.get_mut(replacement[0]) {
                choices.push(replacement[1])
            } else {
                replacements.insert(replacement[0], vec![replacement[1]]);
            }
        } else {
            start = replacement[0];
        }
    }

    (start, replacements)
}

fn find_new_molecules(start: &str, replacements: &HashMap<&str, Vec<&str>>) -> HashSet<String> {
    let mut new_molecules: HashSet<String> = HashSet::new();

    for (key, swaps) in replacements {
        let key: &str = *key;

        let re: regex::Regex = regex::Regex::new(key).unwrap();

        let key_matches: regex::Matches<'_, '_> = re.find_iter(start);

        for m in key_matches {
            let swaps: Vec<&str> = swaps.clone();
            for swap in swaps {
                new_molecules.insert(format!(
                    "{}{}{}",
                    &start[0..m.start()],
                    swap,
                    &start[m.end()..]
                ));
            }
        }
    }

    new_molecules
}

fn get_reversion<'a>(
    replacements: &HashMap<&'a str, Vec<&'a str>>,
) -> HashMap<&'a str, Vec<&'a str>> {
    let mut reversions: HashMap<&str, Vec<&str>> = HashMap::new();

    for (key, values) in replacements {
        for value in values {
            if let Some(k) = reversions.get_mut(*value) {
                k.push(*key);
            } else {
                reversions.insert(*value, vec![*key]);
            }
        }
    }

    reversions
}

fn generate_molecule(start: String, end: String, replacements: &HashMap<&str, Vec<&str>>) -> u32 {
    let mut molecules: VecDeque<(String, u32)> = VecDeque::from([(start, 0)]);

    let mut fastest: HashMap<String, u32> = HashMap::new();

    while molecules.len() > 0 {
        let (molecule, steps) = molecules.pop_back().unwrap();

        if molecule == end {
            return steps;
        }

        fastest.insert(molecule.clone(), steps);

        let new_molecules = find_new_molecules(&molecule[0..], replacements);

        for new_molecule in new_molecules {
            if let Some(steps2) = fastest.get_mut(&new_molecule) {
                if steps + 1 < *steps2 {
                    molecules.push_back((new_molecule, steps + 1));
                }
            } else {
                molecules.push_back((new_molecule, steps + 1));
            }
        }
    }

    0
}
