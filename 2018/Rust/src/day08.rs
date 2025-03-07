use crate::helper::read_data;
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day08.txt");
    let numbers: Vec<u32> = data
        .split_whitespace()
        .map(|num| num.parse::<u32>().unwrap())
        .collect();

    let p1: u32 = add_metadata(&numbers, 0).0;
    let p2: u32 = get_value(&numbers, 0).0;

    println!("{p1}\n{p2}");
}

fn add_metadata(numbers: &Vec<u32>, root: usize) -> (u32, usize) {
    let mut total: u32 = 0;

    let children: usize = numbers[root] as usize;
    let meta: usize = numbers[root + 1] as usize;

    let mut root: usize = root + 2;

    for _ in 0..children {
        let (child_total, pointer) = add_metadata(numbers, root);

        root = pointer;
        total += child_total;
    }

    for _ in 0..meta {
        total += numbers[root];
        root += 1;
    }

    (total, root)
}

fn get_value(numbers: &Vec<u32>, root: usize) -> (u32, usize) {
    let mut value: u32 = 0;

    let children: usize = numbers[root] as usize;
    let meta: usize = numbers[root + 1] as usize;

    let mut child_values: Vec<u32> = Vec::new();

    let mut root: usize = root + 2;

    for _ in 0..children {
        let (child_value, pointer) = get_value(numbers, root);

        child_values.push(child_value);

        root = pointer;
    }

    for _ in 0..meta {
        if children == 0 {
            value += numbers[root];
        } else {
            let ind: usize = numbers[root] as usize;
            if ind <= child_values.len() {
                value += child_values[ind - 1];
            }
        }

        root += 1;
    }

    (value, root)
}
