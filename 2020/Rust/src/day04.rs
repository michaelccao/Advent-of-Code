use crate::helper::read_data;
use std::collections::{HashMap, HashSet};

pub fn main() {
    let data: String = read_data("../Data/Day04.txt");

    let passports: Vec<Vec<&str>> = data.split("\r\n\r\n").map(|pass| pass.split_whitespace().collect()).collect();

    let p1: usize = passports.iter().filter(|&passport| is_valid_passport(passport, false)).count();

    println!("{p1}");

    let p2: usize = passports.iter().filter(|&passport| is_valid_passport(passport, true)).count();

    println!("{p2}");
}

fn is_valid_passport(passport: &Vec<&str>, part2: bool) -> bool {

    let mut required: HashSet<&str> = HashSet::from([
        "byr",
        "iyr",
        "eyr",
        "hgt",
        "hcl",
        "ecl",
        "pid"
    ]);

    for &entry in passport {
        let mut entry = entry.split(":");
        let key: &str = entry.next().unwrap();

        if part2 {
            let value: &str = entry.next().unwrap();
            if is_valid_entry(key, value) {
                required.remove(key);
            }
        } else {
            required.remove(key);
        }

        
    }

    required.len() == 0
}

fn is_valid_entry(key: &str, value: &str) -> bool {
    match key {
        "byr" => {
            if let Ok(year) = value.parse::<u32>() {
                year >= 1920 && year <= 2002
            } else {
                false
            }
        },
        "iyr" => {
            if let Ok(year) = value.parse::<u32>() {
                year >= 2010 && year <= 2020
            } else {
                false
            }
        },
        "eyr" => {
            if let Ok(year) = value.parse::<u32>() {
                year >= 2020 && year <= 2030
            } else {
                false
            }
        },
        "hgt" => {
            let height: &str = &value[0..value.len()-2];
            let units: &str = &value[value.len()-2..];

            if units == "cm" {
                if let Ok(height) = height.parse::<u32>() {
                    height >= 150 && height <= 193
                } else {
                    false
                }
            } else if units == "in" {
                if let Ok(height) = height.parse::<u32>() {
                    height >= 59 && height <= 76
                } else {
                    false
                }
            } else {
                false
            }
        },
        "hcl" => {
            if value.len() == 7 {
                for (i, c) in value.chars().enumerate() {
                    if i == 0 {
                        if c != '#' {
                            return false
                        }
                    } else if !c.is_ascii_digit() && !c.is_lowercase() {
                        return false
                    }
                }

                true
            } else {
                false
            }
        },
        "ecl" => {
            let accepted: HashSet<&str> = HashSet::from([
                "amb",
                "blu",
                "brn",
                "gry",
                "grn",
                "hzl",
                "oth"
            ]);

            accepted.contains(value)
        },
        "pid" => {
            if value.len() == 9 {
                for c in value.chars() {
                    if !c.is_numeric() {
                        return false
                    }
                }
                true
            } else {
                false
            }
        }
        _ => {true}
    }
}