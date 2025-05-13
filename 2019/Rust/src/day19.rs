use crate::helper::read_data;
use std::collections::{HashMap, HashSet, VecDeque};

pub fn main() {
    let data: String = read_data("../Data/Day19.txt");
    let instructions: HashMap<i64, i64> = data
        .split(",")
        .enumerate()
        .map(|(i, num)| (i as i64, num.parse::<i64>().unwrap()))
        .collect();

    let p1: i64 = find_affected_points(&instructions, 0, 50, 0, 50);

    println!("{p1}");

    let p2: i64 = find_best_spot(&instructions, 100);

    println!("{p2:?}");
}

fn find_affected_points(
    instructions: &HashMap<i64, i64>,
    xi: i64,
    xf: i64,
    yi: i64,
    yf: i64,
) -> i64 {
    let mut affected: i64 = 0;

    let mut tm = TractorMap {
        instructions: instructions.clone(),
        cache: HashMap::new()
    };

    for x in xi..xf {
        for y in yi..yf {
            if tm.get_point(x, y) {
                affected += 1;
            }
        }
    }

    affected
}

struct TractorMap {
    instructions: HashMap<i64, i64>,
    cache: HashMap<(i64, i64), bool>,
}

impl TractorMap {
    fn get_point(&mut self, x: i64, y: i64) -> bool {
        if let Some(tractor) = self.cache.get(&(x, y)) {
            return *tractor;
        } else {
            let mut robot: Robot = Robot {
                instructions: self.instructions.clone(),
                pointer: 0,
                inputs: VecDeque::from([x, y]),
                state: RobotState::WaitingOnInput,
                relative_base: 0,
            };

            let tractor: bool = robot.run()[0] == 1;

            self.cache.insert((x, y), tractor);

            return tractor;
        }
    }

    fn fits_ship(&mut self, x: i64, y: i64, size: i64) -> bool {

        for x2 in x..x+size {
            if !self.get_point(x2, y) {
                return false
            }
        }

        for y2 in y..y+size {
            if !self.get_point(x, y2) {
                return false
            }
        }

        true
    }
}

fn find_best_spot(instructions: &HashMap<i64, i64>, size: i64) -> i64 {
    let mut tm: TractorMap = TractorMap {
        instructions: instructions.clone(),
        cache: HashMap::new()
    };

    let start_y: i64 = 50;
    let mut start_x: i64 = 0;

    for x in 0..50 {
        if tm.get_point(x, start_y) {
            start_x = x;
            break;
        }
    }

    let mut nodes: VecDeque<(i64, i64)> = VecDeque::from([(start_x, start_y)]);
    let mut visited: HashSet<(i64, i64)> = HashSet::from([(start_x, start_y)]);

    let mut closest: (i64, i64) = (1000000, 1000000);


    while nodes.len() > 0 {
        let (x, y) = nodes.pop_front().unwrap();

        if x+y >= closest.0 + closest.1 {
            continue;
        }

        if tm.fits_ship(x, y, size) {
            if x + y < closest.0 + closest.1 {
                closest = (x, y);
            }
            continue;
        }

        for (x2, y2) in [(x+1, y), (x, y+1)] {
            if visited.contains(&(x2, y2)) || !tm.get_point(x2, y2) {
                continue;
            } else {
                visited.insert((x2, y2));
                nodes.push_back((x2, y2));
            }
        }
    }

    closest.0*10000 + closest.1
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
