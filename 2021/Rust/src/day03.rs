use crate::helper::read_data;
use std::collections::{HashMap, HashSet};

pub fn main() {
    let data: String = read_data("../Data/Day03.txt");

    let numbers: Vec<Vec<i32>> = data.lines().map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as i32).collect()).collect();

    let p1: u32 = calculate_power(&numbers);

    println!("{p1}");

    let p2: i32 = calculate_life_support(&numbers);

    println!("{p2}");
}

fn calculate_power(data: &Vec<Vec<i32>>) -> u32 {
    let mut counts: Vec<i32> = vec![0;data[0].len()];

    for num in data {
        for (i, digit) in num.iter().enumerate() {
            counts[i] += digit
        }
    }

    let gamma: String = counts.iter().map(|&c| if c >= data.len() as i32 /2  {'1'} else {'0'}).collect();
    let epsilon: String = counts.iter().map(|&c| if c >= data.len() as i32 /2 {'0'} else {'1'}).collect();

    let gamma: u32 = u32::from_str_radix(&gamma, 2).unwrap();
    let epsilon: u32 = u32::from_str_radix(&epsilon, 2).unwrap();

    gamma*epsilon
}

fn reduce_candidates(numbers: &Vec<Vec<i32>>, place: usize, common: bool) -> Vec<Vec<i32>> {
    let mut zeros: Vec<Vec<i32>> = Vec::new();
    let mut ones: Vec<Vec<i32>> = Vec::new();

    for num in numbers {
        if num[place] == 1 {
            ones.push(num.clone());
        } else {
            zeros.push(num.clone());
        }
    }

    if common {
        if ones.len() >= zeros.len() {
            return ones
        } else {
            return zeros
        }
    } else {
        if zeros.len() <= ones.len() {
            return zeros
        } else {
            return ones
        }
    }
}

fn calculate_life_support(numbers: &Vec<Vec<i32>>) -> i32 {
    let mut oxygen: Vec<Vec<i32>> = reduce_candidates(numbers, 0, true);
    let mut co2: Vec<Vec<i32>> = reduce_candidates(numbers, 0, false);

    let mut place: usize = 1;

    while oxygen.len() > 1 {
        oxygen = reduce_candidates(&oxygen, place, true);
        place += 1;
    }

    place = 1;

    while co2.len() > 1 {
        co2 = reduce_candidates(&co2, place, false);
        place += 1;
    }

    let oxygen: i32 = oxygen[0].iter().fold(0, |acc, d| 2*acc + d);
    let co2: i32 = co2[0].iter().fold(0, |acc, d| 2*acc + d);

    oxygen*co2
}