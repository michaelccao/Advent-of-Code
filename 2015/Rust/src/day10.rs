use crate::helper::read_data;
use std::collections::{HashMap, HashSet};
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day10.txt");

    let mut n: Vec<u8> = data.chars().map(|c| c.to_digit(10).unwrap() as u8).collect();

    let p1: Vec<u8> = n_look_say(n, 40);
    let p2: Vec<u8> = n_look_say(p1.clone(), 10);

    println!("{}\n{}", p1.len(), p2.len());
}

fn look_say(n: Vec<u8>) -> Vec<u8> {
    let mut say: Vec<u8> = Vec::new();

    let mut pointer: usize = 0;

    let mut buffer: (u8, u8) = (0, 0);

    while pointer < n.len() {
        let num: u8 = n[pointer];

        if buffer.1 == 0 || buffer.0 == num {
            buffer.0 = num;
            buffer.1 += 1;
        } else {
            say.push(buffer.1);
            say.push(buffer.0);
            buffer = (num, 1);
        }

        pointer += 1;
    }

    if buffer.1 > 0 {
        say.push(buffer.1);
        say.push(buffer.0);
    }

    say
}

fn n_look_say(mut n: Vec<u8>, reps: u8) -> Vec<u8> {
    for _ in 0..reps {
        n = look_say(n);
    }

    n
}