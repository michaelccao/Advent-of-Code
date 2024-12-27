use crate::helper::read_data;
use core::num;
use std::collections::{HashMap, HashSet};
use std::vec::Vec;
use md5;

pub fn main() {
    let data: String = read_data("../Data/Day4.txt");

    let p1: u32 = find_number(&data, 5);
    let p2: u32 = find_number(&data, 6);

    println!("{p1}\n{p2}");

    
}

fn find_number(data: &String, num_zeros: usize) -> u32 {
    let mut num: u32 = 0;

    let mut hash: String = format!("{:x}", md5::compute(format!("{}{}", data, num)));

    let leading_zeros: String = vec!["0"; num_zeros].iter().cloned().collect();

    
    while !hash.starts_with(&leading_zeros) {
        num += 1;

        hash = format!("{:x}", md5::compute(format!("{}{}", data, num)));
    }

    num
}