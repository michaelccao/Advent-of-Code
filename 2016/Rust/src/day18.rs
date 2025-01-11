use crate::helper::read_data;
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day18.txt");

    let first_row: Vec<bool> = data
        .chars()
        .map(|c| if c == '.' { true } else { false })
        .collect();

    let p1: u32 = safe_spots(&first_row, 40);
    let p2: u32 = safe_spots(&first_row, 400000);

    println!("{p1}\n{p2}");
}

fn safe_spots(first_row: &Vec<bool>, rows: u32) -> u32 {
    let mut safe: u32 = first_row.iter().map(|b| if *b { 1 } else { 0 }).sum();

    let mut prev_row: Vec<bool> = first_row.clone();

    for _ in 1..rows {
        let mut current_row: Vec<bool> = Vec::new();

        for i in 0..prev_row.len() {
            let left: bool = if i == 0 { true } else { prev_row[i - 1] };
            let center: bool = prev_row[i];
            let right: bool = if i == prev_row.len() - 1 {
                true
            } else {
                prev_row[i + 1]
            };

            match (left, center, right) {
                (false, false, true)
                | (true, false, false)
                | (false, true, true)
                | (true, true, false) => current_row.push(false),
                _ => {
                    current_row.push(true);
                    safe += 1
                }
            }
        }

        prev_row = current_row;
    }

    safe
}
