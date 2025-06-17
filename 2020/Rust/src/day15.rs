use crate::helper::read_data;
use std::collections::HashMap;

pub fn main() {
    let data: String = read_data("../Data/Day15.txt");
    let nums: Vec<usize> = data.split(",").map(|line| line.parse().unwrap()).collect();

    let p1: usize = memory_game(&nums, 2020);

    println!("{p1}");

    let p2: usize = memory_game(&nums, 30000000);

    println!("{p2}");
}

fn memory_game(nums: &Vec<usize>, n: usize) -> usize {

    let mut last_said: HashMap<usize, usize> = nums.iter().enumerate().map(|(i, &val)| (val, i)).collect();

    let mut next_num: usize = 0;

    for turn in nums.len()..n-1 {

        let last: usize = *last_said.get(&next_num).unwrap_or(&turn);

        last_said.insert(next_num, turn);

        next_num = turn - last;
    }

    next_num

}