use crate::helper::read_data;
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day08.txt");

    let instructions: Vec<(String, usize, usize)> = get_instructions(&data);

    let screen: Vec<Vec<bool>> = follow_instructions(&instructions);

    let p1: u32 = screen
        .iter()
        .map(|row| row.iter().map(|l| *l as u32).sum::<u32>())
        .sum::<u32>();
    let p2: String = screen
        .iter()
        .map(|row| {
            row.iter()
                .map(|l| if *l { '#' } else { ' ' })
                .collect::<String>()
        })
        .collect::<Vec<String>>()
        .join("\n");
    println!("{p1}\n{p2}");
}

fn get_instructions(data: &String) -> Vec<(String, usize, usize)> {
    let mut instructions: Vec<(String, usize, usize)> = Vec::new();

    for line in data.lines() {
        let line: Vec<&str> = line.trim().split_whitespace().collect();

        match line[0] {
            "rect" => {
                let instruction: String = line[0].to_string();
                let mut nums = line[1].split('x');
                let a: usize = nums.next().unwrap().parse().unwrap();
                let b: usize = nums.next().unwrap().parse().unwrap();

                instructions.push((instruction, a, b));
            }
            "rotate" => {
                let instruction: String = line[0..2].join(" ");
                let a: usize = line[2].split('=').last().unwrap().parse().unwrap();
                let b: usize = line[4].parse().unwrap();

                instructions.push((instruction, a, b));
            }
            _ => {}
        }
    }

    instructions
}

fn follow_instructions(instructions: &Vec<(String, usize, usize)>) -> Vec<Vec<bool>> {
    let mut lights: Vec<Vec<bool>> = vec![vec![false; 50]; 6];
    let rows: usize = lights.len();
    let cols: usize = lights[0].len();

    for (instruction, a, b) in instructions {
        match instruction.as_str() {
            "rect" => {
                for i in 0..*b {
                    for j in 0..*a {
                        lights[i][j] = true;
                    }
                }
            }
            "rotate row" => {
                let mut lights2 = lights.clone();

                for j in 0..cols {
                    lights2[*a][(j + *b) % cols] = lights[*a][j];
                }

                lights = lights2;
            }
            "rotate column" => {
                let mut lights2 = lights.clone();

                for i in 0..rows {
                    lights2[(i + *b) % rows][*a] = lights[i][*a];
                }

                lights = lights2;
            }
            _ => {}
        }
    }

    lights
}
