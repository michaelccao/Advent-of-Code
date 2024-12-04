use std::vec::Vec;
use regex::Regex;
use std::iter::zip;
use counter::Counter;
use crate::helper::read_data;

pub fn main() {
    
    let data = read_data("../Data/Day1.txt");

    let (left, right) = get_columns(data);
    
    let p1 = part1(&left, &right);
    
    let p2 = part2(&left, &right);

    println!("{p1}");
    println!("{p2}");

}

fn get_columns(data: String) -> (Vec<i32>, Vec<i32>) {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    let re = Regex::new(r"(\d+)\s*(\d+)").unwrap();

    for line in data.split("\n") {
        let nums = re.captures(line).unwrap();

        let a = &nums[1].parse::<i32>().unwrap();
        let b = &nums[2].parse::<i32>().unwrap();

        left.push(*a);
        right.push(*b);
    }

    return (left, right)
}

fn part1(left: &Vec<i32>, right: &Vec<i32>) -> i32 {
    let mut left = left.clone();
    let mut right = right.clone();

    left.sort();
    right.sort();

    let mut total:i32 = 0;

    for (a,b) in zip(left.into_iter(), right.into_iter()) {
        total += (a-b).abs();
    }

    total
}

fn part2(left: &Vec<i32>, right: &Vec<i32>) -> i32 {

    let right_counts = right.clone().into_iter().collect::<Counter<i32>>();

    let mut total = 0;

    for num in left {
        let count = *right_counts.get(&num).unwrap_or(&0) as i32;

        total += num * count;
    }

    total
}