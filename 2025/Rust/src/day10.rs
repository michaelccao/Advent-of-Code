use crate::helper::read_data;
use std::collections::{HashMap, HashSet};

pub fn main() {
    let data: String = read_data("../Data/Day10.txt");

    let machines = get_machines(&data);

    let p1: usize = machines.iter().map(|machine| light_up(machine)).sum();

    println!("{p1}");

    // let p2: usize = machines.iter().map(|machine| set_joltage(machine)).sum();

    // println!("{p2}");

    test(&machines);
}

fn get_machines(data: &String) -> Vec<(Vec<bool>, Vec<Vec<usize>>, Vec<usize>)> {

    let mut machines: Vec<(Vec<bool>, Vec<Vec<usize>>, Vec<usize>)> = Vec::new();

    for line in data.lines() {
        let mut line = line.split_whitespace();

        let target_lights: Vec<bool> = line.next().unwrap().trim_matches(&['[', ']']).chars().map(|c| c == '#').collect();

        let mut buttons: Vec<Vec<usize>> = Vec::new();

        let mut joltage: Vec<usize> = Vec::new();

        for s in line {
            if s.starts_with("{") {
                joltage = s.trim_matches(&['{', '}']).split(",").map(|s| s.parse().unwrap()).collect();
            } else {
                let button: Vec<usize> = s.trim_matches(&['(', ')']).split(",").map(|s| s.parse().unwrap()).collect();
                buttons.push(button);
            }

            
        }

        machines.push((target_lights, buttons, joltage));

    }

    machines
}

fn light_up(machine: &(Vec<bool>, Vec<Vec<usize>>, Vec<usize>)) -> usize {

    let (target_lights, buttons, _joltage) = machine;

    let mut nodes: Vec<(usize, Vec<bool>)> = Vec::new();

    nodes.push((0, vec![false; target_lights.len()]));

    let mut visited: HashMap<Vec<bool>, usize> = HashMap::new();

    visited.insert(vec![false; target_lights.len()], 0);

    let mut least_pushes: usize = usize::MAX;

    while nodes.len() > 0 {

        let (pushes, lights) = nodes.pop().unwrap();

        if lights == *target_lights {
            least_pushes = least_pushes.min(pushes);
        }

        if pushes + 1 >= least_pushes {
            continue;
        }

        for button in buttons {
            let mut lights2 = lights.clone();

            for &light in button {
                lights2[light] = !lights2[light];
            }

            if !visited.contains_key(&lights2) || pushes + 1 < visited[&lights2] {
                visited.insert(lights2.clone(), pushes+1);
                nodes.push((pushes+1, lights2));
            }
        }

    }

    least_pushes
}

fn set_joltage(machine: &(Vec<bool>, Vec<Vec<usize>>, Vec<usize>)) -> usize {
    let (_target_lights, buttons, joltage) = machine;

    let mut nodes: Vec<(usize, Vec<usize>)> = Vec::new();

    nodes.push((0, vec![0; joltage.len()]));

    let mut visited: HashMap<Vec<usize>, usize> = HashMap::new();

    visited.insert(vec![0; joltage.len()], 0);

    let mut least_pushes: usize = usize::MAX;

    while nodes.len() > 0 {

        let (pushes, j) = nodes.pop().unwrap();

        if j == *joltage {
            least_pushes = least_pushes.min(pushes);
            println!("{least_pushes}");
        }

        let mut min_pushes: usize = 0;
        for i in 0..j.len() {
            min_pushes = min_pushes.max(joltage[i] - j[i]);
        }

        if pushes + min_pushes >= least_pushes {
            continue;
        }

        'b: for button in buttons {
            let mut j2 = j.clone();

            for &light in button {
                j2[light] += 1;

                if j2[light] > joltage[light] {
                    continue 'b
                }
            }

            if !visited.contains_key(&j2) || pushes + 1 < visited[&j2] {
                visited.insert(j2.clone(), pushes+1);
                nodes.push((pushes+1, j2));
            }
        }

    }

    least_pushes
}

fn test(machines: &Vec<(Vec<bool>, Vec<Vec<usize>>, Vec<usize>)>) {

    for i in 0..machines.len() {
        let (target_lights, buttons, target_joltage) = &machines[i];

        if buttons.len() == target_joltage.len() {
            println!("{i} exact");
        } else if buttons.len() < target_joltage.len() {
            println!("{i} less buttons");
        } else {
            println!("{i} more buttons");
        }
    }
}