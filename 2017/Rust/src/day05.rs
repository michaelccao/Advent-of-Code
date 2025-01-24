use crate::helper::read_data;
use std::collections::{HashMap, HashSet};
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day05.txt");
    let jumps: Vec<i32> = data
        .lines()
        .map(|line| line.trim().parse::<i32>().unwrap())
        .collect();

    let p1: u32 = follow_steps(&jumps, false);
    let p2: u32 = follow_steps(&jumps, true);

    println!("{p1}\n{p2}");
}

fn follow_steps(jumps: &Vec<i32>, part2: bool) -> u32 {
    let mut jumps: Vec<i32> = jumps.clone();
    let mut pointer: i32 = 0;
    let mut steps: u32 = 0;

    while pointer >= 0 && pointer < jumps.len() as i32 {
        let jump = jumps[pointer as usize];
        if part2 && jump >= 3 {
            jumps[pointer as usize] -= 1;
        } else {
            jumps[pointer as usize] += 1;
        }

        pointer += jump;
        steps += 1;
    }

    steps
}
