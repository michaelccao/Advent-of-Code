use crate::helper::read_data;
use std::cmp::max;
use std::collections::HashSet;

pub fn main() {
    let data: String = read_data("../Data/Day08.txt");
    let instructions: Vec<&str> = data.lines().collect();

    let p1: i32 = run_instructions(&instructions, 0, 0, HashSet::new(), false, false);

    println!("{p1}");

    let p2: i32 = run_instructions(&instructions, 0, 0, HashSet::new(), true, true);

    println!("{p2}");
}

fn run_instructions(
    instructions: &Vec<&str>,
    mut acc: i32,
    mut pointer: i32,
    mut visited: HashSet<usize>,
    can_alter: bool,
    part2: bool,
) -> i32 {
    while pointer >= 0 && pointer < instructions.len() as i32 {
        let mut instruction = instructions[pointer as usize].split_whitespace();

        let command = instruction.next().unwrap();
        let param: i32 = instruction.next().unwrap().parse().unwrap();

        if !visited.insert(pointer as usize) {
            if part2 {
                return -1;
            } else {
                return acc;
            }
        }

        match command {
            "acc" => {
                acc += param;
                pointer += 1;
            }
            "jmp" => {
                if can_alter {
                    return max(
                        run_instructions(
                            instructions,
                            acc,
                            pointer + 1,
                            visited.clone(),
                            false,
                            part2,
                        ),
                        run_instructions(
                            instructions,
                            acc,
                            pointer + param,
                            visited.clone(),
                            true,
                            part2,
                        ),
                    );
                } else {
                    pointer += param;
                }
            }
            "nop" => {
                if can_alter {
                    return max(
                        run_instructions(
                            instructions,
                            acc,
                            pointer + 1,
                            visited.clone(),
                            true,
                            part2,
                        ),
                        run_instructions(
                            instructions,
                            acc,
                            pointer + param,
                            visited.clone(),
                            false,
                            part2,
                        ),
                    );
                } else {
                    pointer += 1;
                }
            }
            _ => pointer += 1,
        }
    }

    acc
}
