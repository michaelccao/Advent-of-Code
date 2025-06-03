use crate::helper::read_data;
use std::{collections::{HashMap, VecDeque}, io::stdin};

pub fn main() {
    let data: String = read_data("../Data/Day25.txt");
    let instructions: HashMap<i64, i64> = data
        .split(",")
        .enumerate()
        .map(|(i, num)| (i as i64, num.parse::<i64>().unwrap()))
        .collect();

    let mut mud = Robot {
        instructions: instructions.clone(),
        pointer: 0,
        inputs: VecDeque::new(),
        state: RobotState::WaitingOnInput,
        relative_base: 0
    };

    // Just play the game and trial and error the items
    mud.run();


}

fn output_to_ascii(output: &Vec<i64>) -> String {
    output.iter().map(|o| *o as u8 as char).collect()
}

fn str_to_instructions(command: &str) -> VecDeque<i64> {
    command.chars().map(|c| c as i64).collect()
}

#[derive(Clone)]
struct Robot {
    instructions: HashMap<i64, i64>,
    pointer: i64,
    inputs: VecDeque<i64>,
    state: RobotState,
    relative_base: i64,
}

#[derive(PartialEq, Clone)]
enum RobotState {
    WaitingOnInput,
    Stopped,
}

impl Robot {
    fn run(&mut self) -> Vec<i64> {
        let mut outputs: Vec<i64> = Vec::new();

        loop {
            let instruction: i64 = self.instructions[&self.pointer];
            let opcode: i64 = instruction % 100;
            let mut modes: i64 = instruction / 100;

            match opcode {
                1 => {
                    let param1: i64 = self.get_parameter(self.pointer + 1, modes % 10);
                    modes /= 10;
                    let param2: i64 = self.get_parameter(self.pointer + 2, modes % 10);
                    modes /= 10;

                    let mut param3: i64 = *self.instructions.get(&(self.pointer + 3)).unwrap_or(&0);
                    if modes % 10 == 2 {
                        param3 += self.relative_base;
                    }

                    self.instructions.insert(param3, param1 + param2);
                    self.pointer += 4;
                }
                2 => {
                    let param1: i64 = self.get_parameter(self.pointer + 1, modes % 10);
                    modes /= 10;
                    let param2: i64 = self.get_parameter(self.pointer + 2, modes % 10);
                    modes /= 10;

                    let mut param3: i64 = *self.instructions.get(&(self.pointer + 3)).unwrap_or(&0);
                    if modes % 10 == 2 {
                        param3 += self.relative_base;
                    }

                    self.instructions.insert(param3, param1 * param2);
                    self.pointer += 4;
                }
                3 => {
                    let mut param1: i64 = *self.instructions.get(&(self.pointer + 1)).unwrap_or(&0);
                    if modes % 10 == 2 {
                        param1 += self.relative_base;
                    }

                    if self.inputs.len() > 0 {
                        self.instructions
                            .insert(param1, self.inputs.pop_front().unwrap());
                        self.pointer += 2;
                    } else {

                        println!("{}", output_to_ascii(&outputs));

                        let mut user_input: String = String::new();

                        let _ = stdin().read_line(&mut user_input);

                        outputs = Vec::new();

                        user_input = format!("{}\n", user_input.trim());

                        self.inputs = str_to_instructions(&user_input);

                        self.instructions.insert(param1, self.inputs.pop_front().unwrap());
                        self.pointer += 2;
                    }
                }
                4 => {
                    let param1: i64 = self.get_parameter(self.pointer + 1, modes % 10);
                    outputs.push(param1);
                    self.pointer += 2;
                }
                5 => {
                    let param1: i64 = self.get_parameter(self.pointer + 1, modes % 10);
                    modes /= 10;
                    let param2: i64 = self.get_parameter(self.pointer + 2, modes % 10);

                    if param1 != 0 {
                        self.pointer = param2;
                    } else {
                        self.pointer += 3;
                    }
                }
                6 => {
                    let param1: i64 = self.get_parameter(self.pointer + 1, modes % 10);
                    modes /= 10;
                    let param2: i64 = self.get_parameter(self.pointer + 2, modes % 10);

                    if param1 == 0 {
                        self.pointer = param2;
                    } else {
                        self.pointer += 3;
                    }
                }
                7 => {
                    let param1: i64 = self.get_parameter(self.pointer + 1, modes % 10);
                    modes /= 10;
                    let param2: i64 = self.get_parameter(self.pointer + 2, modes % 10);
                    modes /= 10;
                    let mut param3: i64 = *self.instructions.get(&(self.pointer + 3)).unwrap_or(&0);
                    if modes % 10 == 2 {
                        param3 += self.relative_base;
                    }

                    if param1 < param2 {
                        self.instructions.insert(param3, 1);
                    } else {
                        self.instructions.insert(param3, 0);
                    }
                    self.pointer += 4;
                }
                8 => {
                    let param1: i64 = self.get_parameter(self.pointer + 1, modes % 10);
                    modes /= 10;
                    let param2: i64 = self.get_parameter(self.pointer + 2, modes % 10);
                    modes /= 10;
                    let mut param3: i64 = *self.instructions.get(&(self.pointer + 3)).unwrap_or(&0);
                    if modes % 10 == 2 {
                        param3 += self.relative_base;
                    }

                    if param1 == param2 {
                        self.instructions.insert(param3, 1);
                    } else {
                        self.instructions.insert(param3, 0);
                    }
                    self.pointer += 4;
                }
                9 => {
                    let param1: i64 = self.get_parameter(self.pointer + 1, modes % 10);

                    self.relative_base += param1;
                    self.pointer += 2;
                }
                99 => {
                    self.state = RobotState::Stopped;
                    return outputs;
                }
                _ => continue,
            }
        }
    }

    fn get_parameter(&self, pointer: i64, mode: i64) -> i64 {
        match mode {
            0 => {
                let address = *self.instructions.get(&pointer).unwrap_or(&0);
                *self.instructions.get(&address).unwrap_or(&0)
            }
            1 => *self.instructions.get(&pointer).unwrap_or(&0),
            2 => {
                let address = *self.instructions.get(&pointer).unwrap_or(&0);
                *self
                    .instructions
                    .get(&(address + self.relative_base))
                    .unwrap_or(&0)
            }
            _ => *self.instructions.get(&pointer).unwrap_or(&0),
        }
    }
}
