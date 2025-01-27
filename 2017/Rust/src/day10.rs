use crate::helper::read_data;
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day10.txt");
    let lengths: Vec<usize> = data
        .split(',')
        .map(|num| num.parse::<usize>().unwrap())
        .collect();

    let mut lengths2: Vec<usize> = data.chars().map(|c| c as usize).collect();
    lengths2.append(&mut vec![17, 31, 73, 47, 23]);

    let p1: u32 = hash(&lengths);

    let p2: String = hash2(&lengths2);

    println!("{p1}\n{p2}");
}

fn hash(lengths: &Vec<usize>) -> u32 {
    let mut circle: Vec<u32> = (0..256).collect();

    let mut pointer: usize = 0;
    let circle_size: usize = circle.len();
    let mut skip: usize = 0;

    for &l in lengths {
        for i in 0..l / 2 {
            let a: usize = (pointer + i) % circle_size;
            let b: usize = (pointer + l - i - 1) % circle_size;

            circle.swap(a, b);
        }

        pointer += l + skip;
        pointer %= circle_size;

        skip += 1;
    }

    circle[0] * circle[1]
}

fn hash2(lengths: &Vec<usize>) -> String {
    let mut circle: Vec<u32> = (0..256).collect();

    let mut pointer: usize = 0;
    let circle_size: usize = circle.len();
    let mut skip: usize = 0;

    for _ in 0..64 {
        for &l in lengths {
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

    let mut knot_hash: String = String::new();

    for i in 0..circle_size / 16 {
        let mut dense_hash: u32 = 0;
        for j in 0..16 {
            dense_hash ^= circle[16 * i + j];
        }

        knot_hash = format!("{knot_hash}{dense_hash:02x}");
    }

    knot_hash
}
