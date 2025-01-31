use crate::helper::read_data;
use std::collections::{HashMap, HashSet};
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day14.txt");

    println!("{:?}", knot_hash(&"flqrgnkx-1".to_string()));
}



fn knot_hash(key: &String) -> Vec<bool> {
    let lengths: Vec<usize> = key.chars().map(|c| c as usize).collect();
    let mut circle: Vec<u32> = (0..256).collect();

    let mut pointer: usize = 0;
    let circle_size: usize = circle.len();
    let mut skip: usize = 0;

    for _ in 0..64 {
        for &l in &lengths {
            for i in 0..l / 2 {
                let a: usize = (pointer + i) % circle_size;
                let b: usize = (pointer + l - i - 1) % circle_size;

                circle.swap(a, b);
            }

            pointer += l + skip;
            pointer %= circle_size;

            skip += 1;
        }
    }

    let mut knot_hash: Vec<bool> = Vec::new();

    for i in 0..circle_size / 16 {
        let mut dense_hash: u32 = 0;
        for j in 0..16 {
            dense_hash ^= circle[16 * i + j];
        }

        for bit in 0..8_u32 {
            knot_hash.push((dense_hash >> (8-bit-1)) & 1 == 1);
        }
    }

    knot_hash
}