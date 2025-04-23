use crate::helper::read_data;

pub fn main() {
    let data: String = read_data("../Data/Day05.txt");
    let instructions: Vec<i32> = data.split(",").map(|num| num.parse().unwrap()).collect();

    // Part 1
    execute_instructions(&instructions, 1);

    // Part 2
    execute_instructions(&instructions, 5);
}

fn execute_instructions(instructions: &Vec<i32>, input: i32) {
    let mut instructions: Vec<i32> = instructions.clone();

    let mut pointer: usize = 0;

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
                instructions[param1 as usize] = input;
                pointer += 2;
            }
            4 => {
                let param1: i32 = if modes % 10 == 0 {
                    instructions[instructions[pointer + 1] as usize]
                } else {
                    instructions[pointer + 1]
                };
                println!("{}", param1);
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
}
