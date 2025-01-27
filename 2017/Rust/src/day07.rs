use crate::helper::read_data;
use std::collections::HashMap;
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day07.txt");

    let discs: HashMap<String, Disc> = get_discs(&data);

    let p1: String = find_root(&discs);

    let p2: u32 = correct_weight(&p1, &discs);

    println!("{p1}\n{p2}");
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct Disc {
    name: String,
    weight: u32,
    children: Option<Vec<String>>,
    parent: Option<String>,
}

fn get_discs(data: &String) -> HashMap<String, Disc> {
    let mut discs: HashMap<String, Disc> = HashMap::new();

    for line in data.lines() {
        let line: Vec<&str> = line.trim().split_whitespace().collect();

        let name: String = line[0].to_string();
        let weight: u32 = line[1].trim_matches(&['(', ')']).parse().unwrap();

        let mut children: Vec<String> = Vec::new();

        for i in 3..line.len() {
            let child: String = line[i].trim_matches(',').to_string();

            if let Some(disc) = discs.get_mut(&child) {
                disc.parent = Some(name.clone());
            } else {
                let child_disc = Disc {
                    name: child.clone(),
                    weight: 0,
                    children: None,
                    parent: Some(name.clone()),
                };

                discs.insert(child.clone(), child_disc);
            }

            children.push(child);
        }

        if let Some(disc) = discs.get_mut(&name) {
            disc.weight = weight;
            disc.children = Some(children);
        } else {
            let disc: Disc = Disc {
                name: name.clone(),
                weight: weight,
                children: Some(children),
                parent: None,
            };

            discs.insert(name, disc);
        }
    }

    discs
}

fn find_root(discs: &HashMap<String, Disc>) -> String {
    let mut disc: &Disc = discs.values().next().unwrap();

    while disc.parent.is_some() {
        disc = discs.get(&disc.parent.clone().unwrap()).unwrap();
    }

    disc.name.clone()
}

fn total_weight(disc: &Disc, discs: &HashMap<String, Disc>) -> u32 {
    if disc.children.is_none() {
        return disc.weight;
    }

    return disc.weight
        + disc
            .children
            .clone()
            .unwrap()
            .iter()
            .map(|disc_name| total_weight(&discs[disc_name], discs))
            .sum::<u32>();
}

fn is_balanced(disc: &Disc, discs: &HashMap<String, Disc>) -> bool {
    if disc.children.is_none() {
        return true;
    }

    let children: Vec<String> = disc.children.clone().unwrap();

    let child_weights: Vec<u32> = children
        .iter()
        .map(|child_name| total_weight(&discs[child_name], discs))
        .collect();

    child_weights
        .iter()
        .all(|&weight| weight == child_weights[0])
}

fn correct_weight(root: &String, discs: &HashMap<String, Disc>) -> u32 {
    let mut disc: &Disc = &discs[root];

    'outer: loop {
        for child_name in disc.children.clone().unwrap() {
            let child_disc: &Disc = &discs[&child_name];

            if !is_balanced(child_disc, discs) {
                disc = child_disc;
                continue 'outer;
            }
        }
        break;
    }

    let parent_disc: &Disc = disc;

    let children: Vec<String> = parent_disc.children.clone().unwrap();

    let child_weights: Vec<u32> = parent_disc
        .children
        .clone()
        .unwrap()
        .iter()
        .map(|child_name| total_weight(&discs[child_name], discs))
        .collect();

    let weight = child_weights[0];

    for i in 1..children.len() {
        if child_weights[i] != weight {
            if i != 1 || child_weights[2] == weight {
                let bad_child: &Disc = &discs[&children[i]];

                return bad_child.weight + weight - child_weights[i];
            } else {
                let bad_child: &Disc = &discs[&children[0]];

                return bad_child.weight + weight - child_weights[0];
            }
        }
    }

    0
}
