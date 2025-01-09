use crate::helper::read_data;
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day15.txt");

    let discs: Vec<(u32, u32)> = get_discs(&data);

    let mut discs2: Vec<(u32, u32)> = discs.clone();

    discs2.push((0, 11));

    let p1: u32 = find_start(&discs);
    let p2: u32 = find_start(&discs2);

    println!("{p1}\n{p2}");
}

fn get_discs(data: &String) -> Vec<(u32, u32)> {
    let mut discs: Vec<(u32, u32)> = Vec::new();

    for line in data.lines() {
        let line: Vec<&str> = line.split_whitespace().collect();

        let num_positions: u32 = line[3].parse().unwrap();

        let start_position: u32 = line[11][0..line[11].len() - 1].parse().unwrap();

        discs.push((start_position, num_positions));
    }

    discs
}

fn find_start(discs: &Vec<(u32, u32)>) -> u32 {
    let mut t: u32 = 0;

    let mut positions: Vec<u32> = discs
        .iter()
        .enumerate()
        .map(|(ind, (start, num))| (*start + t + (ind as u32 + 1)) % (*num))
        .collect();

    while positions.iter().sum::<u32>() != 0 {
        t += 1;
        positions = discs
            .iter()
            .enumerate()
            .map(|(ind, (start, num))| (*start + t + (ind as u32 + 1)) % (*num))
            .collect();
    }

    t
}
