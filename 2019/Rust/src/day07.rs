use crate::helper::read_data;
use std::collections::{HashMap, HashSet, VecDeque};
use itertools::Itertools;

pub fn main() {
    let data: String = read_data("../Data/Day07.txt");
    let instructions: Vec<i32> = data.split(",").map(|num| num.parse().unwrap()).collect();

    let p1 = run_thrusters(&instructions);
    println!("{p1}");
}


fn run_thrusters(instructions: &Vec<i32>) -> i32 {

    let configurations = (0..5).permutations(5);

    let mut largest_output: i32 = 0;

    for config in configurations {
        let mut output: Vec<i32> = vec![0];

        for phase in config {
            let mut input: VecDeque<i32> = VecDeque::from([phase]);

            for out in output {
                input.push_front(out);
            }
            
            output = execute_instructions(instructions, input);
        }
        
        largest_output = largest_output.max(*output.last().unwrap());
    }

    largest_output
}

fn execute_instructions(instructions: &Vec<i32>, mut input: VecDeque<i32>) -> Vec<i32> {
    let mut instructions: Vec<i32> = instructions.clone();

    let mut pointer: usize = 0;

    let mut outputs: Vec<i32> = Vec::new();

    while pointer < instructions.len() {
        let instruction: i32 = instructions[pointer];
        let opcode: i32 = instruction % 100;
        let mut modes: i32 = instruction / 100;

        match opcode {
            1 => {
                let param1: i32 = if modes % 10 == 0 {
                    instructions[instructions[pointer + 1] as usize]
                } else {
                    instructions[pointer + 1]
                };
                modes /= 10;
                let param2: i32 = if modes % 10 == 0 {
                    instructions[instructions[pointer + 2] as usize]
                } else {
                    instructions[pointer + 2]
                };

                let param3: i32 = instructions[pointer + 3];

                instructions[param3 as usize] = param1 + param2;
                pointer += 4;
            }
            2 => {
                let param1: i32 = if modes % 10 == 0 {
                    instructions[instructions[pointer + 1] as usize]
                } else {
                    instructions[pointer + 1]
                };
                modes /= 10;
                let param2: i32 = if modes % 10 == 0 {
                    instructions[instructions[pointer + 2] as usize]
                } else {
                    instructions[pointer + 2]
                };

                let param3: i32 = instructions[pointer + 3];

                instructions[param3 as usize] = param1 * param2;
                pointer += 4;
            }
            3 => {
                let param1: i32 = instructions[pointer + 1];
                instructions[param1 as usize] = input.pop_back().unwrap();
                pointer += 2;
            }
            4 => {
                let param1: i32 = if modes % 10 == 0 {
                    instructions[instructions[pointer + 1] as usize]
                } else {
                    instructions[pointer + 1]
                };
                outputs.push(param1);
                pointer += 2;
            }
            5 => {
                let param1: i32 = if modes % 10 == 0 {
                    instructions[instructions[pointer + 1] as usize]
                } else {
                    instructions[pointer + 1]
                };
                modes /= 10;
                let param2: i32 = if modes % 10 == 0 {
                    instructions[instructions[pointer + 2] as usize]
                } else {
                    instructions[pointer + 2]
                };

                if param1 != 0 {
                    pointer = param2 as usize;
                } else {
                    pointer += 3;
                }
            }
            6 => {
                let param1: i32 = if modes % 10 == 0 {
                    instructions[instructions[pointer + 1] as usize]
                } else {
                    instructions[pointer + 1]
                };
                modes /= 10;
                let param2: i32 = if modes % 10 == 0 {
                    instructions[instructions[pointer + 2] as usize]
                } else {
                    instructions[pointer + 2]
                };

                if param1 == 0 {
                    pointer = param2 as usize;
                } else {
                    pointer += 3;
                }
            }
            7 => {
                let param1: i32 = if modes % 10 == 0 {
                    instructions[instructions[pointer + 1] as usize]
                } else {
                    instructions[pointer + 1]
                };
                modes /= 10;
                let param2: i32 = if modes % 10 == 0 {
                    instructions[instructions[pointer + 2] as usize]
                } else {
                    instructions[pointer + 2]
                };

                let param3: i32 = instructions[pointer + 3];

                if param1 < param2 {
                    instructions[param3 as usize] = 1;
                } else {
                    instructions[param3 as usize] = 0;
                }
                pointer += 4;
            }
            8 => {
                let param1: i32 = if modes % 10 == 0 {
                    instructions[instructions[pointer + 1] as usize]
                } else {
                    instructions[pointer + 1]
                };
                modes /= 10;
                let param2: i32 = if modes % 10 == 0 {
                    instructions[instructions[pointer + 2] as usize]
                } else {
                    instructions[pointer + 2]
                };

                let param3: i32 = instructions[pointer + 3];

                if param1 == param2 {
                    instructions[param3 as usize] = 1;
                } else {
                    instructions[param3 as usize] = 0;
                }
                pointer += 4;
            }
            99 => break,
            _ => continue,
        }
    }

    outputs
}