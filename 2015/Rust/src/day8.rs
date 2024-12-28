use crate::helper::read_data;
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day8.txt");

    let p1: u32 = data.lines().map(|line| parse_str(line)).sum();
    let p2: u32 = data.lines().map(|line| encode_str(line)).sum();

    println!("{p1}\n{p2}");
}

fn parse_str(s: &str) -> u32 {
    let s: Vec<char> = s.chars().collect();

    let code_chars: u32 = s.len() as u32;

    let mut literal_chars: u32 = 0;

    let mut pointer: usize = 1;

    while pointer < (code_chars - 1) as usize {
        let c = s[pointer];

        literal_chars += 1;

        if c != '\\' {
            pointer += 1;
        } else {
            if s[pointer + 1] == 'x' {
                pointer += 4;
            } else {
                pointer += 2;
            }
        }
    }

    code_chars - literal_chars
}

fn encode_str(s: &str) -> u32 {
    let mut new_str: u32 = 2;

    for c in s.chars() {
        match c {
            '"' => new_str += 1,
            '\\' => new_str += 1,
            _ => {}
        };
    }

    return new_str;
}
