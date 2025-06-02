use crate::helper::read_data;
use std::collections::{HashMap, VecDeque};

pub fn main() {
    let data: String = read_data("../Data/Day23.txt");
    let instructions: HashMap<i64, i64> = data
        .split(",")
        .enumerate()
        .map(|(i, num)| (i as i64, num.parse::<i64>().unwrap()))
        .collect();

    let p1: i64 = run_network(&instructions);

    println!("{p1}");

    let p2: i64 = run_network2(&instructions);

    println!("{p2}");
}

fn run_network(instructions: &HashMap<i64, i64>) -> i64 {
    let mut network: Vec<Robot> = Vec::new();

    for network_address in 0..50 {
        network.push(Robot {
            instructions: instructions.clone(),
            pointer: 0,
            inputs: VecDeque::from([network_address]),
            outputs: VecDeque::new(),
            relative_base: 0,
            no_input: 0,
        })
    }

    loop {
        for network_adress in 0..50 {
            let comp: &mut Robot = &mut network[network_adress];

            comp.run();

            if comp.outputs.len() == 3 {
                let target_pc: i64 = comp.outputs.pop_front().unwrap();

                let x: i64 = comp.outputs.pop_front().unwrap();
                let y: i64 = comp.outputs.pop_front().unwrap();

                if target_pc == 255 {
                    return y;
                }

                network[target_pc as usize].inputs.push_back(x);
                network[target_pc as usize].inputs.push_back(y);
            }
        }
    }
}

fn run_network2(instructions: &HashMap<i64, i64>) -> i64 {
    let mut network: Vec<Robot> = Vec::new();
    let mut nat: (i64, i64) = (0, 0);

    let mut last_nat_y: Option<i64> = None;

    for network_address in 0..50 {
        network.push(Robot {
            instructions: instructions.clone(),
            pointer: 0,
            inputs: VecDeque::from([network_address]),
            outputs: VecDeque::new(),
            relative_base: 0,
            no_input: 0,
        })
    }

    loop {
        let mut idling: u8 = 0;

        for network_adress in 0..50 {
            let comp: &mut Robot = &mut network[network_adress];

            comp.run();

            if comp.outputs.len() == 3 {
                let target_pc: i64 = comp.outputs.pop_front().unwrap();

                let x: i64 = comp.outputs.pop_front().unwrap();
                let y: i64 = comp.outputs.pop_front().unwrap();

                if target_pc == 255 {
                    nat = (x, y);
                } else {
                    network[target_pc as usize].inputs.push_back(x);
                    network[target_pc as usize].inputs.push_back(y);
                }
            } else if comp.no_input > 9 {
                idling += 1;
            }
        }

        if idling == 50 {
            network[0].inputs.push_back(nat.0);
            network[0].inputs.push_back(nat.1);

            if let Some(last_y) = last_nat_y {
                if last_y == nat.1 {
                    return last_y;
                }
            }

            last_nat_y = Some(nat.1);

            for network_address in 0..50 {
                network[network_address].no_input = 0;
            }
        }
    }
}

#[derive(Clone)]
struct Robot {
    instructions: HashMap<i64, i64>,
    pointer: i64,
    inputs: VecDeque<i64>,
    outputs: VecDeque<i64>,
    relative_base: i64,
    no_input: u32,
}

impl Robot {
    fn run(&mut self) {
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
                    self.no_input = 0;
                } else {
                    self.instructions.insert(param1, -1);
                    self.pointer += 2;
                    self.no_input += 1;
                }
            }
            4 => {
                let param1: i64 = self.get_parameter(self.pointer + 1, modes % 10);
                self.outputs.push_back(param1);
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
            99 => {}
            _ => {}
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
