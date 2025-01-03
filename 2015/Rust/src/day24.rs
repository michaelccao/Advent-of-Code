use crate::helper::read_data;
use std::collections::HashSet;
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day24.txt");
    let weights: Vec<u64> = data
        .lines()
        .map(|line| line.trim().parse::<u64>().unwrap())
        .collect();

    let target_weight: u64 = weights.iter().sum::<u64>() / 3;
    let target_weight2: u64 = weights.iter().sum::<u64>() / 4;

    let smallest_group: Option<Vec<u64>> = find_smallest_group2(target_weight, &weights, false);
    let smallest_group2: Option<Vec<u64>> = find_smallest_group2(target_weight2, &weights, true);

    let p1: u64 = smallest_group.unwrap().iter().product();
    let p2: u64 = smallest_group2.unwrap().iter().product();

    println!("{p1}\n{p2}");
}

fn find_smallest_group2(target_weight: u64, weights: &Vec<u64>, part2: bool) -> Option<Vec<u64>> {
    let mut smallest: Option<Vec<u64>> = None;

    let mut nodes: Vec<(u64, Vec<u64>, usize)> = vec![(0, Vec::new(), 0)];

    while nodes.len() > 0 {
        let (weight, group, pointer) = nodes.pop().unwrap();

        if weight > target_weight {
            continue;
        }

        if let Some(ref sgroup) = smallest {
            if sgroup.len() < group.len() {
                continue;
            }
        }

        if weight == target_weight {
            let weights2: Vec<u64> = HashSet::<u64>::from_iter(weights.clone())
                .difference(&HashSet::<u64>::from_iter(group.clone()))
                .cloned()
                .collect();

            if !can_sum(target_weight, &weights2, part2) {
                continue;
            }

            if let Some(ref sgroup) = smallest {
                if group.len() < sgroup.len()
                    || (group.len() == sgroup.len()
                        && group.iter().product::<u64>() < sgroup.iter().product::<u64>())
                {
                    smallest = Some(group);
                }
            } else {
                smallest = Some(group);
            }
            continue;
        }

        for i in pointer..weights.len() {
            let mut group2 = group.clone();
            group2.push(weights[i]);

            nodes.push((weight + weights[i], group2, i + 1));
        }
    }

    smallest
}

fn can_sum(target_weight: u64, weights: &Vec<u64>, part2: bool) -> bool {
    let mut nodes: Vec<(u64, Vec<u64>, usize)> = vec![(0, Vec::new(), 0)];

    while nodes.len() > 0 {
        let (weight, group, pointer) = nodes.pop().unwrap();

        if weight == target_weight {
            if !part2 {
                return true;
            } else {
                let weights2: Vec<u64> = HashSet::<u64>::from_iter(weights.clone())
                    .difference(&HashSet::<u64>::from_iter(group.clone()))
                    .cloned()
                    .collect();

                if can_sum(target_weight, &weights2, false) {
                    return true;
                }

                continue;
            }
        }

        for i in pointer..weights.len() {
            let mut group2 = group.clone();
            group2.push(weights[i]);
            nodes.push((weight + weights[i], group2, i + 1));
        }
    }

    false
}
