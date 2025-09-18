use crate::helper::read_data;
use std::collections::HashMap;

pub fn main() {
    let data: String = read_data("../Data/Day14.txt");

    let (polymer, reactions, first, last) = parse_template(&data);

    let p1: usize = score(&mutate_n(&polymer, &reactions, 10), first, last);

    println!("{p1}");

    let p2: usize = score(&mutate_n(&polymer, &reactions, 40), first, last);

    println!("{p2}");
}

fn parse_template(data: &String) -> (HashMap<String, usize>, HashMap<String, char>, char, char) {
    let mut lines = data.lines();

    let mut reactions: HashMap<String, char> = HashMap::new();

    let polymer: Vec<char> = lines.next().unwrap().chars().collect();

    let first: char = polymer[0];
    let last: char = *polymer.last().unwrap();

    let mut pairs: HashMap<String, usize> = HashMap::new();

    for i in 0..polymer.len() - 1 {
        let pair = format!("{}{}", polymer[i], polymer[i + 1]);
        if let Some(count) = pairs.get_mut(&pair) {
            *count += 1;
        } else {
            pairs.insert(pair, 1);
        }
    }

    lines.next();

    for line in lines {
        let mut line = line.split(" -> ");
        let pair: String = line.next().unwrap().to_string();
        let c: char = line.next().unwrap().chars().next().unwrap();

        reactions.insert(pair, c);
    }

    (pairs, reactions, first, last)
}

fn mutate(
    polymer: &HashMap<String, usize>,
    reactions: &HashMap<String, char>,
) -> HashMap<String, usize> {
    let mut polymer2: HashMap<String, usize> = polymer.clone();

    for (pair, count) in polymer {
        *polymer2.get_mut(pair).unwrap() -= count;

        let middle: char = reactions[pair];
        let mut pair = pair.chars();
        let start: char = pair.next().unwrap();
        let end: char = pair.next().unwrap();

        let out1: String = format!("{}{}", start, middle);
        let out2: String = format!("{}{}", middle, end);

        if let Some(count1) = polymer2.get_mut(&out1) {
            *count1 += count;
        } else {
            polymer2.insert(out1, *count);
        }

        if let Some(count2) = polymer2.get_mut(&out2) {
            *count2 += count;
        } else {
            polymer2.insert(out2, *count);
        }
    }

    polymer2
}

fn mutate_n(
    polymer: &HashMap<String, usize>,
    reactions: &HashMap<String, char>,
    n: usize,
) -> HashMap<String, usize> {
    let mut polymer2 = polymer.clone();

    for _ in 0..n {
        polymer2 = mutate(&polymer2, reactions);
    }

    polymer2
}

fn score(polymer: &HashMap<String, usize>, first: char, last: char) -> usize {
    let mut char_count: HashMap<char, usize> = HashMap::new();

    for (pair, count) in polymer {
        for c in pair.chars() {
            if let Some(count2) = char_count.get_mut(&c) {
                *count2 += count;
            } else {
                char_count.insert(c, *count);
            }
        }
    }

    *char_count.get_mut(&first).unwrap() += 1;
    *char_count.get_mut(&last).unwrap() += 1;

    let max_count: usize = *char_count.values().max().unwrap() / 2;
    let min_count: usize = *char_count.values().min().unwrap() / 2;

    max_count - min_count
}
