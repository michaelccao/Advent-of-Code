use crate::helper::read_data;
use std::collections::{HashMap, HashSet};

pub fn main() {
    let data: String = read_data("../Data/Day08.txt");

    let digits: Vec<Vec<Vec<char>>> = get_digits(&data);

    let p1: usize = digits
        .iter()
        .map(|display| {
            display[10..]
                .iter()
                .filter(|digit| {
                    digit.len() == 2 || digit.len() == 4 || digit.len() == 3 || digit.len() == 7
                })
                .count()
        })
        .sum();

    println!("{p1}");

    let p2: u32 = digits.iter().map(|display| decode(display)).sum();

    println!("{p2}");
}

fn get_digits(data: &String) -> Vec<Vec<Vec<char>>> {
    let mut digits: Vec<Vec<Vec<char>>> = Vec::new();

    for line in data.lines() {
        let line = line.split(" | ");

        let mut display: Vec<Vec<char>> = Vec::new();

        for seg in line {
            for digit in seg.split_whitespace() {
                let mut digit: Vec<char> = digit.chars().collect();
                digit.sort_unstable();

                display.push(digit);
            }
        }

        digits.push(display)
    }

    digits
}

fn decode(display: &Vec<Vec<char>>) -> u32 {
    let mut decoder: HashMap<Vec<char>, u32> = HashMap::new();
    let mut digits: Vec<HashSet<char>> = vec![HashSet::new(); 10];
    let mut six_segs: Vec<Vec<char>> = Vec::new();
    let mut five_segs: Vec<Vec<char>> = Vec::new();

    // 1, 4, 7, 8 - unique segments
    // 0, 6, 9 - 6 segments
    // 2, 3, 5 - 5 segments
    for digit in display[0..10].iter() {
        match digit.len() {
            2 => {
                decoder.insert(digit.clone(), 1);
                digits[1] = digit.iter().cloned().collect();
            }
            3 => {
                decoder.insert(digit.clone(), 7);
                digits[7] = digit.iter().cloned().collect();
            }
            4 => {
                decoder.insert(digit.clone(), 4);
                digits[4] = digit.iter().cloned().collect();
            }
            5 => {
                five_segs.push(digit.clone());
            }
            6 => {
                six_segs.push(digit.clone());
            }
            7 => {
                decoder.insert(digit.clone(), 8);
                digits[8] = digit.iter().cloned().collect();
            }
            _ => {}
        }
    }

    // 0 and 9 has two intersections with 1
    // but 6 only has one intersection with 1
    // 9 has 4 intersections with 4
    // but 0 only has 3
    for digit in six_segs {
        let digit_hash: HashSet<char> = digit.iter().cloned().collect();

        let one_intersection: usize = digit_hash.intersection(&digits[1]).count();
        if one_intersection == 1 {
            digits[6] = digit_hash.clone();
            decoder.insert(digit.clone(), 6);
            continue;
        }

        let four_intersection: usize = digit_hash.intersection(&digits[4]).count();
        if four_intersection == 4 {
            digits[9] = digit_hash.clone();
            decoder.insert(digit.clone(), 9);
        } else {
            digits[0] = digit_hash.clone();
            decoder.insert(digit.clone(), 0);
        }
    }

    // 5 has 5 intersections with 6, 2 and 3 only have 4
    // 3 has 2 intersections with 1, 2 only has 1
    for digit in five_segs {
        let digit_hash: HashSet<char> = digit.iter().cloned().collect();

        let six_intersection: usize = digit_hash.intersection(&digits[6]).count();
        if six_intersection == 5 {
            digits[5] = digit_hash.clone();
            decoder.insert(digit.clone(), 5);
            continue;
        }

        let one_intersection: usize = digit_hash.intersection(&digits[1]).count();
        if one_intersection == 2 {
            digits[3] = digit_hash.clone();
            decoder.insert(digit.clone(), 3);
        } else {
            digits[2] = digit_hash.clone();
            decoder.insert(digit.clone(), 2);
        }
    }

    let mut result: u32 = 0;

    for digit in display[10..].iter() {
        result *= 10;
        result += decoder[digit];
    }

    result
}
