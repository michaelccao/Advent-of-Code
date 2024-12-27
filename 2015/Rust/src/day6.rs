use crate::helper::read_data;
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day6.txt");
    let instructions: Vec<Vec<&str>> = data.lines().map(|line| line.split(' ').collect::<Vec<_>>()).collect::<Vec<_>>();

    let p1: u32 = follow_instructions(&instructions);
    let p2: u32 = follow_instructions2(&instructions);

    println!("{p1}\n{p2}");
}

fn follow_instructions(instructions: &Vec<Vec<&str>>) -> u32 {
    let mut lights: Vec<Vec<u32>> = vec![vec![0; 1000]; 1000];

    for instruction in instructions {
        let command: &str = instruction[0];

        let start: Vec<&str> = instruction[instruction.len()-3].split(',').collect();
        let start_row: usize = start[0].parse().unwrap();
        let start_col: usize = start[1].parse().unwrap();

        let end: Vec<&str> = instruction[instruction.len()-1].split(',').collect();
        let end_row: usize = end[0].parse().unwrap();
        let end_col: usize = end[1].parse().unwrap();

        if command == "turn" {
            for i in start_row..end_row+1 {
                for j in start_col..end_col+1 {
                    lights[i][j] = if instruction[1] == "on" {1} else {0};
                }
            }
        } else {
            for i in start_row..end_row+1 {
                for j in start_col..end_col+1 {
                    lights[i][j] = 1 - lights[i][j];
                }
            }
        }

    }

    lights.iter().map(|line| line.iter().sum::<u32>()).sum()
}

fn follow_instructions2(instructions: &Vec<Vec<&str>>) -> u32 {
    let mut lights: Vec<Vec<u32>> = vec![vec![0; 1000]; 1000];

    for instruction in instructions {
        let command: &str = instruction[0];

        let start: Vec<&str> = instruction[instruction.len()-3].split(',').collect();
        let start_row: usize = start[0].parse().unwrap();
        let start_col: usize = start[1].parse().unwrap();

        let end: Vec<&str> = instruction[instruction.len()-1].split(',').collect();
        let end_row: usize = end[0].parse().unwrap();
        let end_col: usize = end[1].parse().unwrap();

        if command == "turn" {
            for i in start_row..end_row+1 {
                for j in start_col..end_col+1 {
                    if instruction[1] == "on" {
                        lights[i][j] += 1;
                    } else if lights[i][j] > 0 && instruction[1] == "off" {
                        lights[i][j] = lights[i][j] - 1;
                    }
                }
            }
        } else {
            for i in start_row..end_row+1 {
                for j in start_col..end_col+1 {
                    lights[i][j] += 2;
                }
            }
        }

    }

    lights.iter().map(|line| line.iter().sum::<u32>()).sum()
}