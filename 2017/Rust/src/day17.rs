use crate::helper::read_data;
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day17.txt");
    let steps: usize = data.parse().unwrap();

    let p1: u32 = spinlock(steps, 2017);
    let p2: u32 = angryspinlock(steps as u32, 50000000);

    println!("{p1}\n{p2}");
}

fn spinlock(steps: usize, n: u32) -> u32 {
    let mut buffer: Vec<u32> = vec![0];

    let mut pointer: usize = 0;

    for i in 1..n + 1 {
        pointer += steps;
        pointer %= buffer.len();

        buffer.insert(pointer + 1, i);

        pointer += 1;
    }

    if pointer < buffer.len() - 1 {
        return buffer[pointer + 1];
    } else {
        return buffer[0];
    }
}

fn angryspinlock(steps: u32, n: u32) -> u32 {
    let mut position: u32 = 0;
    let mut nums: u32 = 1;

    let mut pos1: u32 = 0;

    for i in 1..n + 1 {
        position += steps;
        position %= nums;

        if position == 0 {
            pos1 = i;
        }

        position += 1;
        nums += 1;
    }

    pos1
}
