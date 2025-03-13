use crate::helper::read_data;
use std::collections::{HashMap, HashSet, VecDeque};
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day12.txt");

    get_plants(&data);
}

fn get_plants(data: &String) {

    let lines: Vec<Vec<&str>> = data.lines().map(|line| line.trim().split_whitespace().collect()).collect();

    let plants: VecDeque<bool> = lines[0][2].chars().map(|c| c == '#').collect();

    println!("{:?}", plants);

    
}