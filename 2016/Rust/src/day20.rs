use crate::helper::read_data;
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day20.txt");

    let ranges: Vec<(u32, u32)> = get_ranges(&data);

    let p1: u32 = find_lowest_allowed(&ranges);
    let p2: u32 = find_allowed(&ranges);

    println!("{p1}\n{p2}");
}

fn get_ranges(data: &String) -> Vec<(u32, u32)> {
    let mut ranges: Vec<(u32, u32)> = Vec::new();

    for line in data.lines() {
        let mut line = line.trim().split('-');

        let start: u32 = line.next().unwrap().parse().unwrap();
        let end: u32 = line.next().unwrap().parse().unwrap();

        ranges.push((start, end));
    }

    ranges.sort();

    ranges
}

fn find_lowest_allowed(ranges: &Vec<(u32, u32)>) -> u32 {
    let mut forbidden: u32 = 0;

    for (start, end) in ranges {
        let start: u32 = *start;
        let end: u32 = *end;

        if start > forbidden+1 {
            break;
        }

        if end > forbidden {
            forbidden = end;
        }
    }

    forbidden+1
}

fn find_allowed(ranges: &Vec<(u32, u32)>) -> u32 {
    let mut forbidden: u32 = 0;
    let mut allowed: u32 = 0;

    for (start, end) in ranges {
        let start: u32 = *start;
        let end: u32 = *end;

        if start > forbidden+1 {
            allowed += start - forbidden - 1;
        }

        if end > forbidden {
            forbidden = end;
            if forbidden == u32::MAX {
                break;
            }
        }
    }

    if forbidden < u32::MAX - 1 {
        allowed += u32::MAX - forbidden - 1;
    }

    allowed
}