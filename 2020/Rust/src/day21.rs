use crate::helper::read_data;
use std::collections::{HashMap, HashSet};

pub fn main() {
    let data: String = read_data("../Data/Day21.txt");

    let (_ingredients, allergens) = read_ingredients_list(&data);

    let p1: u32 = count_safe_ingredients(&data, &allergens.values().cloned().collect());

    println!("{p1}");

    let p2: String = sort_allergens(&allergens);

    println!("{p2}");
}

fn read_ingredients_list(data: &String) -> (HashSet<String>, HashMap<String, String>) {
    let mut all_ingredients: HashSet<String> = HashSet::new();
    let mut all_allergens: HashMap<String, HashSet<String>> = HashMap::new();

    for line in data.lines() {
        let mut line = line.split(" (contains ");
        let ingredients: HashSet<String> = line
            .next()
            .unwrap()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        let allergens = line.next().unwrap().trim_end_matches(")").split(", ");

        all_ingredients = all_ingredients.union(&ingredients).cloned().collect();

        for allergen in allergens {
            let allergen: String = allergen.to_string();

            if let Some(candidates) = all_allergens.get_mut(&allergen) {
                *candidates = candidates.intersection(&ingredients).cloned().collect();
            } else {
                all_allergens.insert(allergen, ingredients.clone());
            }
        }
    }

    let mut final_allergens: HashMap<String, String> = HashMap::new();

    while all_allergens.len() > 0 {
        let mut remove: Vec<(String, String)> = Vec::new();

        for (allergen, candidates) in &all_allergens {
            if candidates.len() == 1 {
                let candidate = candidates.iter().next().unwrap();
                remove.push((allergen.clone(), candidate.clone()));
            }
        }

        for (allergen, candidate) in remove {
            final_allergens.insert(allergen.clone(), candidate.clone());

            all_allergens.remove(&allergen);

            for a2 in all_allergens.clone().keys() {
                all_allergens.get_mut(a2).unwrap().remove(&candidate);
            }
        }
    }

    (all_ingredients, final_allergens)
}

fn count_safe_ingredients(data: &String, allergens: &HashSet<String>) -> u32 {
    let mut safe: u32 = 0;
    for line in data.lines() {
        let mut line = line.split(" (contains ");
        let ingredients = line
            .next()
            .unwrap()
            .split_whitespace()
            .map(|s| s.to_string());

        for ingredient in ingredients {
            if !allergens.contains(&ingredient) {
                safe += 1;
            }
        }
    }

    safe
}

fn sort_allergens(allergens: &HashMap<String, String>) -> String {
    let mut keys: Vec<&String> = allergens.keys().collect();
    keys.sort_unstable();

    let danger_list: Vec<String> = keys.iter().map(|&k| allergens[k].clone()).collect();

    format!("{}", danger_list.join(","))
}
