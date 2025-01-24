use crate::helper::read_data;
use std::collections::HashMap;
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day06.txt");
    let banks: Vec<u32> = data
        .split_whitespace()
        .map(|num| num.parse::<u32>().unwrap())
        .collect();

    let p1: u32 = balance(&banks, false);
    let p2: u32 = balance(&banks, true);

    println!("{p1}\n{p2}");
}

fn argmax(v: &Vec<u32>) -> usize {
    let mut largest: u32 = v[0];
    let mut largest_ind: usize = 0;

    for i in 1..v.len() {
        if v[i] > largest {
            largest = v[i];
            largest_ind = i;
        }
    }

    largest_ind
}

fn redistribute(mut v: Vec<u32>, i: usize) -> Vec<u32> {
    let mut buffer: u32 = v[i];
    v[i] = 0;

    if v[i] > v.len() as u32 {
        for j in 0..v.len() {
            v[j] += buffer / v.len() as u32;
        }
    }

    buffer %= v.len() as u32;

    let mut j: usize = (i + 1) % v.len();

    while buffer > 0 {
        v[j] += 1;
        buffer -= 1;

        j = (j + 1) % v.len();
    }

    v
}

fn balance(banks: &Vec<u32>, part2: bool) -> u32 {
    let mut banks: Vec<u32> = banks.clone();
    let mut redis: u32 = 0;

    let mut states: HashMap<Vec<u32>, u32> = HashMap::new();

    states.insert(banks.clone(), 0);

    loop {
        let largest_bank: usize = argmax(&banks);

        banks = redistribute(banks, largest_bank);

        redis += 1;

        if let Some(prev) = states.insert(banks.clone(), redis) {
            if part2 {
                return redis - prev;
            } else {
                return redis;
            }
        }
    }
}
