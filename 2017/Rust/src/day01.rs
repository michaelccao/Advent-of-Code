use crate::helper::read_data;
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day01.txt");

    let p1: u32 = solve_captcha(&data, false);
    let p2: u32 = solve_captcha(&data, true);

    println!("{p1}\n{p2}");
}

fn solve_captcha(data: &String, part2: bool) -> u32 {
    let mut total: u32 = 0;

    let digits: Vec<u32> = data.chars().map(|c| c.to_digit(10).unwrap()).collect();

    let offset: usize = if part2 { digits.len() / 2 } else { 1 };

    for i in 0..digits.len() {
        if digits[i] == digits[(i + offset) % digits.len()] {
            total += digits[i];
        }
    }

    total
}
