use crate::helper::read_data;
use std::collections::HashSet;
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day01.txt");

    let p1: i32 = data
        .lines()
        .map(|line| line.trim().parse::<i32>().unwrap())
        .sum();

    let p2: i32 = first_repeat(&data);

    println!("{p1}\n{p2}");
}

fn first_repeat(data: &String) -> i32 {
    let mut frequencies: HashSet<i32> = HashSet::new();

    let changes: Vec<i32> = data
        .lines()
        .map(|line| line.trim().parse::<i32>().unwrap())
        .collect();

    let mut current_freq: i32 = 0;

    let mut pointer: usize = 0;

    while !frequencies.contains(&current_freq) {
        frequencies.insert(current_freq);

        current_freq += changes[pointer];

        pointer = (pointer + 1) % changes.len();
    }

    current_freq
}
