use crate::helper::read_data;
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day02.txt");

    let p1: u32 = calculate_checksum(&data);
    let p2: u32 = calculate_checksum2(&data);

    println!("{p1}\n{p2}");
}

fn calculate_checksum(data: &String) -> u32 {
    let mut total: u32 = 0;

    for line in data.lines() {
        let nums = line
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<u32>().unwrap());
        total += nums.clone().max().unwrap() - nums.min().unwrap();
    }

    total
}

fn calculate_checksum2(data: &String) -> u32 {
    let mut total: u32 = 0;

    for line in data.lines() {
        let mut nums: Vec<u32> = line
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect();
        nums.sort();

        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                if nums[j] % nums[i] == 0 {
                    total += nums[j] / nums[i];
                }
            }
        }
    }

    total
}
