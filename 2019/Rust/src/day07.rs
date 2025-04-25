use crate::helper::read_data;
use itertools::Itertools;
use std::collections::VecDeque;

pub fn main() {
    let data: String = read_data("../Data/Day07.txt");
    let instructions: Vec<i32> = data.split(",").map(|num| num.parse().unwrap()).collect();

    let p1: i32 = run_thrusters(&instructions);
    println!("{p1}");

    let p2: i32 = feedback_thrusters(&instructions);
    println!("{p2}");
}

struct Thruster {
    instructions: Vec<i32>,
    pointer: usize,
    inputs: VecDeque<i32>,
    state: ThrusterState,
}
#[derive(PartialEq)]
enum ThrusterState {
    WaitingOnInput,
    Stopped,
}

impl Thruster {
    fn run(&mut self) -> Vec<i32> {
        let mut outputs: Vec<i32> = Vec::new();

        while self.pointer < self.instructions.len() {
            let instruction: i32 = self.instructions[self.pointer];
            let opcode: i32 = instruction % 100;
            let mut modes: i32 = instruction / 100;

            match opcode {
                1 => {
                    let param1: i32 = if modes % 10 == 0 {
                        self.instructions[self.instructions[self.pointer + 1] as usize]
                    } else {
                        self.instructions[self.pointer + 1]
                    };
                    modes /= 10;
                    let param2: i32 = if modes % 10 == 0 {
                        self.instructions[self.instructions[self.pointer + 2] as usize]
                    } else {
                        self.instructions[self.pointer + 2]
                    };

                    let param3: i32 = self.instructions[self.pointer + 3];

                    self.instructions[param3 as usize] = param1 + param2;
                    self.pointer += 4;
                }
                2 => {
                    let param1: i32 = if modes % 10 == 0 {
                        self.instructions[self.instructions[self.pointer + 1] as usize]
                    } else {
                        self.instructions[self.pointer + 1]
                    };
                    modes /= 10;
                    let param2: i32 = if modes % 10 == 0 {
                        self.instructions[self.instructions[self.pointer + 2] as usize]
                    } else {
                        self.instructions[self.pointer + 2]
                    };

                    let param3: i32 = self.instructions[self.pointer + 3];

                    self.instructions[param3 as usize] = param1 * param2;
                    self.pointer += 4;
                }
                3 => {
                    let param1: i32 = self.instructions[self.pointer + 1];
                    if self.inputs.len() > 0 {
                        self.instructions[param1 as usize] = self.inputs.pop_back().unwrap();
                        self.pointer += 2;
                    } else {
                        self.state = ThrusterState::WaitingOnInput;
                        return outputs;
                    }
                }
                4 => {
                    let param1: i32 = if modes % 10 == 0 {
                        self.instructions[self.instructions[self.pointer + 1] as usize]
                    } else {
                        self.instructions[self.pointer + 1]
                    };
                    outputs.push(param1);
                    self.pointer += 2;
                }
                5 => {
                    let param1: i32 = if modes % 10 == 0 {
                        self.instructions[self.instructions[self.pointer + 1] as usize]
                    } else {
                        self.instructions[self.pointer + 1]
                    };
                    modes /= 10;
                    let param2: i32 = if modes % 10 == 0 {
                        self.instructions[self.instructions[self.pointer + 2] as usize]
                    } else {
                        self.instructions[self.pointer + 2]
                    };

                    if param1 != 0 {
                        self.pointer = param2 as usize;
                    } else {
                        self.pointer += 3;
                    }
                }
                6 => {
                    let param1: i32 = if modes % 10 == 0 {
                        self.instructions[self.instructions[self.pointer + 1] as usize]
                    } else {
                        self.instructions[self.pointer + 1]
                    };
                    modes /= 10;
                    let param2: i32 = if modes % 10 == 0 {
                        self.instructions[self.instructions[self.pointer + 2] as usize]
                    } else {
                        self.instructions[self.pointer + 2]
                    };

                    if param1 == 0 {
                        self.pointer = param2 as usize;
                    } else {
                        self.pointer += 3;
                    }
                }
                7 => {
                    let param1: i32 = if modes % 10 == 0 {
                        self.instructions[self.instructions[self.pointer + 1] as usize]
                    } else {
                        self.instructions[self.pointer + 1]
                    };
                    modes /= 10;
                    let param2: i32 = if modes % 10 == 0 {
                        self.instructions[self.instructions[self.pointer + 2] as usize]
                    } else {
                        self.instructions[self.pointer + 2]
                    };

                    let param3: i32 = self.instructions[self.pointer + 3];

                    if param1 < param2 {
                        self.instructions[param3 as usize] = 1;
                    } else {
                        self.instructions[param3 as usize] = 0;
                    }
                    self.pointer += 4;
                }
                8 => {
                    let param1: i32 = if modes % 10 == 0 {
                        self.instructions[self.instructions[self.pointer + 1] as usize]
                    } else {
                        self.instructions[self.pointer + 1]
                    };
                    modes /= 10;
                    let param2: i32 = if modes % 10 == 0 {
                        self.instructions[self.instructions[self.pointer + 2] as usize]
                    } else {
                        self.instructions[self.pointer + 2]
                    };

                    let param3: i32 = self.instructions[self.pointer + 3];

                    if param1 == param2 {
                        self.instructions[param3 as usize] = 1;
                    } else {
                        self.instructions[param3 as usize] = 0;
                    }
                    self.pointer += 4;
                }
                99 => {
                    self.state = ThrusterState::Stopped;
                    return outputs;
                }
                _ => continue,
            }
        }
        outputs
    }
}

fn run_thrusters(instructions: &Vec<i32>) -> i32 {
    let configurations = (0..5).permutations(5);

    let mut largest_output: i32 = 0;

    for config in configurations {
        let mut output: Vec<i32> = vec![0];

        for phase in config {
            let mut inputs: VecDeque<i32> = VecDeque::from([phase]);

            for out in output {
                inputs.push_front(out);
            }

            let mut thruster = Thruster {
                instructions: instructions.clone(),
                pointer: 0,
                inputs: inputs,
                state: ThrusterState::WaitingOnInput,
            };

            output = thruster.run();
        }

        largest_output = largest_output.max(*output.last().unwrap());
    }

    largest_output
}

fn feedback_thrusters(instructions: &Vec<i32>) -> i32 {
    let configurations = (5..10).permutations(5);
    let mut largest_output: i32 = 0;

    for config in configurations {
        let mut thrusters: Vec<Thruster> = Vec::new();

        for phase in config {
            let thruster = Thruster {
                instructions: instructions.clone(),
                pointer: 0,
                inputs: VecDeque::from([phase]),
                state: ThrusterState::WaitingOnInput,
            };

            thrusters.push(thruster);
        }

        let mut active_thruster: usize = 0;
        thrusters[0].inputs.push_front(0);

        let num_thrusters: usize = thrusters.len();

        while thrusters.last().unwrap().state == ThrusterState::WaitingOnInput {
            let output: Vec<i32> = thrusters[active_thruster].run();

            let next_thruster: &mut Thruster =
                &mut thrusters[(active_thruster + 1) % num_thrusters];

            if next_thruster.state == ThrusterState::WaitingOnInput {
                for o in output {
                    next_thruster.inputs.push_front(o);
                }
            } else {
                largest_output = largest_output.max(*output.last().unwrap());
            }

            active_thruster += 1;
            active_thruster %= num_thrusters;
        }
    }

    largest_output
}
