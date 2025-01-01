use crate::helper::read_data;
use std::collections::{HashMap, HashSet};
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day17.txt");

    let mut containers: Vec<u32> = data
        .lines()
        .map(|container| container.parse().unwrap())
        .collect();
    containers.sort();

    let mut cache: HashMap<u32, HashSet<Vec<bool>>> = HashMap::new();
    let mut visited: HashSet<Vec<bool>> = HashSet::new();

    let all_empty: Vec<bool> = vec![false; containers.len()];

    cache.insert(0, HashSet::from([all_empty.clone()]));

    (cache, visited) = fill_containers(150, &containers, all_empty, cache, visited);

    let p1: usize = cache[&150].len();

    let min_containers: usize = cache[&150]
        .iter()
        .map(|used| used.iter().map(|x| *x as usize).sum::<usize>())
        .min()
        .unwrap();
    let p2: usize = cache[&150]
        .iter()
        .map(|used| (used.iter().map(|x| *x as usize).sum::<usize>() == min_containers) as usize)
        .sum();

    println!("{p1}\n{p2}");
}

fn fill_containers(
    target: u32,
    containers: &Vec<u32>,
    used: Vec<bool>,
    mut cache: HashMap<u32, HashSet<Vec<bool>>>,
    mut visited: HashSet<Vec<bool>>,
) -> (HashMap<u32, HashSet<Vec<bool>>>, HashSet<Vec<bool>>) {
    if visited.contains(&used) {
        return (cache, visited);
    }

    visited.insert(used.clone());

    let filled: u32 = (0..containers.len())
        .map(|i| if used[i] { containers[i] } else { 0 })
        .sum();

    if filled >= target {
        return (cache, visited);
    }

    for i in 0..containers.len() {
        if !used[i] {
            let container = containers[i];
            let mut used2 = used.clone();
            used2[i] = true;

            if let Some(combos) = cache.get_mut(&(filled + containers[i])) {
                combos.insert(used2.clone());
            } else {
                cache.insert(filled + container, HashSet::from([used2.clone()]));
            }

            (cache, visited) = fill_containers(target, containers, used2, cache, visited);
        }
    }

    (cache, visited)
}
