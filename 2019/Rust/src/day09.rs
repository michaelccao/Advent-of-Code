use crate::helper::read_data;
use std::collections::{HashMap, VecDeque};

pub fn main() {
    let data: String = read_data("../Data/Day09.txt");
    let instructions: Vec<i64> = data.split(",").map(|num| num.parse().unwrap()).collect();

    let p1: Vec<i64> = get_boost_keycode(&instructions);

    println!("{p1:?}");

    let p2: Vec<i64> = get_coordinates(&instructions);

    println!("{p2:?}");

}

fn get_boost_keycode(instructions: &Vec<i64>) -> Vec<i64> {
    let mut instructions_map: HashMap<i64, i64> = HashMap::new();

    for i in 0..instructions.len() {
        instructions_map.insert(i as i64, instructions[i]);
    }

    let mut thruster = Thruster {
        instructions: instructions_map,
        pointer: 0,
        inputs: VecDeque::from([1]),
        state: ThrusterState::WaitingOnInput,
        relative_base: 0,
    };

    let outputs: Vec<i64> = thruster.run();

    outputs
}

fn get_coordinates(instructions: &Vec<i64>) -> Vec<i64> {
    let mut instructions_map: HashMap<i64, i64> = HashMap::new();

    for i in 0..instructions.len() {
        instructions_map.insert(i as i64, instructions[i]);
    }

    let mut thruster = Thruster {
        instructions: instructions_map,
        pointer: 0,
        inputs: VecDeque::from([2]),
        state: ThrusterState::WaitingOnInput,
        relative_base: 0,
    };

    let outputs: Vec<i64> = thruster.run();

    outputs
}

struct Thruster {
    instructions: HashMap<i64, i64>,
    pointer: i64,
    inputs: VecDeque<i64>,
    state: ThrusterState,
    relative_base: i64,
}
#[derive(PartialEq)]
enum ThrusterState {
    WaitingOnInput,
    Stopped,
}

impl Thruster {
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
                            .insert(param1, self.inputs.pop_back().unwrap());
                        self.pointer += 2;
                    } else {
                        self.state = ThrusterState::WaitingOnInput;
                        return outputs;
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
                    self.state = ThrusterState::Stopped;
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
