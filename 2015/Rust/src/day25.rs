use crate::helper::read_data;
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day25.txt");
    let target: Vec<&str> = data.split(' ').collect();
    let row: u32 = target[target.len() - 3].trim_matches(',').parse().unwrap();
    let col: u32 = target[target.len() - 1].trim_matches('.').parse().unwrap();

    let n: u32 = coord_to_n(row, col);

    let p1: u64 = nth_code(20151125, n);

    println!("{p1}");
}

fn coord_to_n(row: u32, col: u32) -> u32 {
    let steps: u32 = row - 1;

    let mut n = col + steps;

    n = (n + 1) * n / 2 - steps;

    n
}

fn nth_code(mut start: u64, n: u32) -> u64 {
    for _ in 1..n {
        start = (start * 252533) % 33554393
    }

    start
}
