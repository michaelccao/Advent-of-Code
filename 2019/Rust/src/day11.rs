use crate::helper::read_data;
use std::collections::{HashMap, VecDeque};

pub fn main() {
    let data: String = read_data("../Data/Day11.txt");
    let instructions: HashMap<i64, i64> = data
        .split(",")
        .enumerate()
        .map(|(i, num)| (i as i64, num.parse::<i64>().unwrap()))
        .collect();

    let p1: usize = paint(&instructions, 0).len();

    println!("{p1}");

    let p2: String = tiles_to_string(&paint(&instructions, 1));

    println!("{p2}");
}

fn tiles_to_string(tiles: &HashMap<(i32, i32), i64>) -> String {
    let mut x_min: i32 = i32::MAX;
    let mut x_max: i32 = i32::MIN;
    let mut y_min: i32 = i32::MAX;
    let mut y_max: i32 = i32::MIN;

    for (x, y) in tiles.keys().cloned() {
        x_min = x_min.min(x);
        x_max = x_max.max(x);

        y_min = y_min.min(y);
        y_max = y_max.max(y);
    }

    let width: usize = (x_max - x_min + 1) as usize;
    let height: usize = (y_max - y_min + 1) as usize;

    let mut tiles_arr: Vec<Vec<char>> = vec![vec![' '; width]; height];

    for (&(x, y), &c) in tiles {
        if c == 1 {
            tiles_arr[(y - y_min) as usize][(x - x_min) as usize] = '#';
        }
    }

    let mut output_str: String = String::new();

    for row in tiles_arr {
        for c in row {
            output_str.push(c);
        }
        output_str.push('\n')
    }

    output_str
}

fn paint(instructions: &HashMap<i64, i64>, start_tile: i64) -> HashMap<(i32, i32), i64> {
    let mut robot: Robot = Robot {
        instructions: instructions.clone(),
        pointer: 0,
        inputs: VecDeque::new(),
        state: RobotState::WaitingOnInput,
        relative_base: 0,
    };

    let mut tiles: HashMap<(i32, i32), i64> = HashMap::from([((0, 0), start_tile)]);
    let mut x: i32 = 0;
    let mut y: i32 = 0;

    let moves: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

    let mut heading: usize = 0;

    while robot.state != RobotState::Stopped {
        robot.inputs.push_front(*tiles.get(&(x, y)).unwrap_or(&0));

        let outputs: Vec<i64> = robot.run();

        tiles.insert((x, y), outputs[0]);

        if outputs[1] == 0 {
            heading += 3;
        } else if outputs[1] == 1 {
            heading += 1;
        }

        heading %= moves.len();

        let (dx, dy) = moves[heading];
        x += dx;
        y += dy;
    }

    tiles
}

struct Robot {
    instructions: HashMap<i64, i64>,
    pointer: i64,
    inputs: VecDeque<i64>,
    state: RobotState,
    relative_base: i64,
}
#[derive(PartialEq)]
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
                            .insert(param1, self.inputs.pop_back().unwrap());
                        self.pointer += 2;
                    } else {
                        self.state = RobotState::WaitingOnInput;
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
