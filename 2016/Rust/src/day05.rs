use crate::helper::read_data;
use md5::compute;
use std::collections::HashMap;
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day05.txt");

    let p1: String = get_password(&data);

    let p2: String = get_password2(&data);

    println!("{p1}\n{p2}");
}

fn next_interesting_hash(s: &String, mut start: u32) -> (String, u32) {
    let mut hash: String = format!("{:x}", compute(format!("{s}{start}")));

    while !hash.starts_with("00000") {
        start += 1;

        hash = format!("{:x}", compute(format!("{s}{start}")));
    }

    (hash, start)
}

fn get_password(s: &String) -> String {
    let mut pass: String = String::new();
    let mut start: u32 = 0;

    while pass.len() != 8 {
        let (hash, next) = next_interesting_hash(s, start);

        start = next + 1;
        pass.push(hash.chars().collect::<Vec<char>>()[5]);
    }

    pass
}

fn get_password2(s: &String) -> String {
    let mut pass: HashMap<u32, char> = HashMap::new();
    let mut start: u32 = 0;

    while pass.len() != 8 {
        let (hash, next) = next_interesting_hash(s, start);

        start = next + 1;

        let hash: Vec<char> = hash.chars().collect();

        if let Some(ind) = hash[5].to_digit(10) {
            if ind < 8 {
                if !pass.contains_key(&ind) {
                    pass.insert(ind, hash[6]);
                }
            }
        }
    }

    (0..8_u32).map(|k| pass[&k]).collect::<String>()
}
