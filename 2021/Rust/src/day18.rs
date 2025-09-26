use crate::helper::read_data;
use std::collections::HashMap;

pub fn main() {
    let data: String = read_data("../Data/Day18.txt");
    let snailfish_numbers: Vec<Vec<(String, u32)>> = data
        .lines()
        .map(|line| create_tree_from_str(line, Vec::new(), &String::new()))
        .collect();

    let mut total: Vec<(String, u32)> = snailfish_numbers[0].clone();
    for sf2 in &snailfish_numbers[1..] {
        total = add_snailfish(&total, sf2);
        total = reduce(total);
    }

    let p1: u32 = magnitude(&total);

    println!("{p1}");

    let mut p2: u32 = 0;

    for i in 0..snailfish_numbers.len() {
        let sf1: &Vec<(String, u32)> = &snailfish_numbers[i];
        for j in i + 1..snailfish_numbers.len() {
            let sf2: &Vec<(String, u32)> = &snailfish_numbers[j];

            p2 = p2.max(magnitude(&reduce(add_snailfish(sf1, sf2))));
            p2 = p2.max(magnitude(&reduce(add_snailfish(sf2, sf1))));
        }
    }

    println!("{p2}");
}

fn create_tree_from_str(
    s: &str,
    mut tree: Vec<(String, u32)>,
    parent: &String,
) -> Vec<(String, u32)> {
    let s = &s[1..s.len() - 1];

    let mut split: usize = 0;
    let mut depth: i32 = 0;

    for (i, c) in s.chars().enumerate() {
        if c == '[' {
            depth += 1;
        } else if c == ']' {
            depth -= 1;
        } else if c == ',' && depth == 0 {
            split = i;
            break;
        }
    }

    let left: &str = &s[0..split];
    let right: &str = &s[split + 1..];

    let left_name: String = format!("{parent}L");
    let right_name: String = format!("{parent}R");

    if let Ok(num) = left.parse::<u32>() {
        tree.push((left_name, num));
    } else {
        tree = create_tree_from_str(left, tree, &left_name);
    }

    if let Ok(num) = right.parse::<u32>() {
        tree.push((right_name, num));
    } else {
        tree = create_tree_from_str(right, tree, &right_name);
    }

    tree.sort_unstable();

    tree
}

fn add_snailfish(sf1: &Vec<(String, u32)>, sf2: &Vec<(String, u32)>) -> Vec<(String, u32)> {
    let mut sf: Vec<(String, u32)> = Vec::new();

    for (branch, val) in sf1 {
        let new_branch: String = format!("L{branch}");
        sf.push((new_branch, *val));
    }

    for (branch, val) in sf2 {
        let new_branch: String = format!("R{branch}");
        sf.push((new_branch, *val));
    }

    sf.sort_unstable();

    sf
}

fn explode(mut sf: Vec<(String, u32)>, index: usize) -> Vec<(String, u32)> {
    let (left_name, left) = sf.remove(index);
    let (_, right) = sf.remove(index);

    let parent: String = left_name[0..left_name.len() - 1].to_string();

    if index > 0 {
        sf[index - 1].1 += left;
    }

    if index < sf.len() {
        sf[index].1 += right;
    }

    sf.push((parent, 0));

    sf.sort_unstable();

    sf
}

fn split(mut sf: Vec<(String, u32)>, index: usize) -> Vec<(String, u32)> {
    let (parent, val) = sf.remove(index);

    let left_name: String = format!("{parent}L");
    let right_name: String = format!("{parent}R");

    let left: u32 = val / 2;
    let right: u32 = val - left;

    sf.push((left_name, left));
    sf.push((right_name, right));

    sf.sort_unstable();

    sf
}

fn reduce(mut sf: Vec<(String, u32)>) -> Vec<(String, u32)> {
    let mut split_ind: Option<usize> = None;
    let mut explode_ind: Option<usize> = None;

    loop {
        for (i, (branch, val)) in sf.iter().enumerate() {
            if branch.len() > 4 && explode_ind.is_none() {
                explode_ind = Some(i);
            } else if *val >= 10 && split_ind.is_none() {
                split_ind = Some(i);
            }
        }

        if split_ind.is_none() && explode_ind.is_none() {
            break;
        } else if let Some(index) = explode_ind {
            sf = explode(sf, index);
        } else if let Some(index) = split_ind {
            sf = split(sf, index);
        }

        explode_ind = None;
        split_ind = None;
    }

    sf
}

fn magnitude(sf: &Vec<(String, u32)>) -> u32 {
    let tree: HashMap<String, u32> = sf.iter().cloned().collect();

    get_magnitude(&tree, &String::new())
}

fn get_magnitude(tree: &HashMap<String, u32>, branch: &String) -> u32 {
    if let Some(val) = tree.get(branch) {
        return *val;
    } else {
        let left_branch: String = format!("{branch}L");
        let right_branch: String = format!("{branch}R");
        return 3 * get_magnitude(tree, &left_branch) + 2 * get_magnitude(tree, &right_branch);
    }
}
