use crate::helper::read_data;
use std::collections::HashMap;
use std::iter::zip;
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day02.txt");
    let boxes: Vec<&str> = data.lines().map(|line| line.trim()).collect();

    let p1: u32 = twos_and_threes(&boxes);

    let p2: String = find_match(&boxes);

    println!("{p1}\n{p2}");
}

fn twos_and_threes(boxes: &Vec<&str>) -> u32 {
    let mut twos: u32 = 0;
    let mut threes: u32 = 0;

    for &b in boxes {
        let mut letter_counter: HashMap<char, u32> = HashMap::new();

        for c in b.chars() {
            if let Some(count) = letter_counter.get_mut(&c) {
                *count += 1;
            } else {
                letter_counter.insert(c, 1);
            }
        }

        let counts: Vec<u32> = letter_counter.values().cloned().collect();

        if counts.contains(&2) {
            twos += 1;
        }

        if counts.contains(&3) {
            threes += 1;
        }
    }

    twos * threes
}

fn find_match(boxes: &Vec<&str>) -> String {
    for i in 0..boxes.len() {
        for j in i + 1..boxes.len() {
            let mut mismatch: u32 = 0;

            let mut matching: String = String::new();

            for (c1, c2) in zip(boxes[i].chars(), boxes[j].chars()) {
                if c1 != c2 {
                    mismatch += 1;
                } else {
                    matching.push(c1);
                }

                if mismatch == 2 {
                    break;
                }
            }

            if mismatch == 1 {
                return matching;
            }
        }
    }

    String::new()
}
