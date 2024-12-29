use crate::helper::read_data;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day13.txt");

    let (people, happy): (HashSet<&str>, HashMap<(&str, &str), i32>) = get_happiness(&data);

    let p1: i32 = happiest_circle(&people, &happy);

    let mut people2: HashSet<&str> = people.clone();
    people2.insert("You");

    let p2: i32 = happiest_circle(&people2, &happy);

    println!("{p1}\n{p2}");
}

fn get_happiness(data: &String) -> (HashSet<&str>, HashMap<(&str, &str), i32>) {
    let mut happy: HashMap<(&str, &str), i32> = HashMap::new();
    let mut people: HashSet<&str> = HashSet::new();

    for line in data.lines() {
        let d_hap: Vec<&str> = line.split(' ').collect();
        let p1 = d_hap[0];
        let p2 = &d_hap[10][0..d_hap[10].len() - 1]; // Remove the period at the end
        let change: i32 =
            d_hap[3].parse::<i32>().unwrap() * if d_hap[2] == "gain" { 1 } else { -1 };

        happy.insert((p1, p2), change);
        people.insert(p1);
    }

    (people, happy)
}

fn happiest_circle(people: &HashSet<&str>, happy: &HashMap<(&str, &str), i32>) -> i32 {
    people
        .iter()
        .cloned()
        .permutations(people.len())
        .map(|circle| evaluate_circle(&circle, happy))
        .max()
        .unwrap()
}

fn evaluate_circle(circle: &Vec<&str>, happy: &HashMap<(&str, &str), i32>) -> i32 {
    let mut happiness: i32 = 0;
    for i in 0..circle.len() {
        let center: &str = circle[i];
        let left: &str = if i > 0 {
            circle[i - 1]
        } else {
            circle[circle.len() - 1]
        };
        let right: &str = if i < circle.len() - 1 {
            circle[i + 1]
        } else {
            circle[0]
        };

        happiness += match happy.get(&(center, right)) {
            Some(h) => *h,
            None => 0,
        };

        happiness += match happy.get(&(center, left)) {
            Some(h) => *h,
            None => 0,
        };
    }

    happiness
}
