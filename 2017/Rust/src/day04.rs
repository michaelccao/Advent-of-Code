use crate::helper::read_data;
use std::collections::HashSet;
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day04.txt");

    let p1: u32 = valid_passwords(&data, false);
    let p2: u32 = valid_passwords(&data, true);

    println!("{p1}\n{p2}");
}

fn valid_passwords(data: &String, part2: bool) -> u32 {
    let mut valid: u32 = 0;

    'password: for line in data.lines() {
        let line: &str = line.trim();

        let mut words: HashSet<String> = HashSet::new();

        for word in line.split_whitespace() {
            if part2 {
                let mut word: Vec<char> = word.chars().collect::<Vec<char>>();
                word.sort();

                let word: String = word.iter().collect();

                if !words.insert(word) {
                    continue 'password;
                }
            } else {
                if !words.insert(word.to_string()) {
                    continue 'password;
                }
            }
        }

        valid += 1;
    }

    valid
}
