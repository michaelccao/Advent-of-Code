use crate::helper::read_data;
use std::collections::{HashMap, VecDeque};

pub fn main() {
    let data: String = read_data("../Data/Day15.txt");
    let instructions: HashMap<i64, i64> = data
        .split(",")
        .enumerate()
        .map(|(i, num)| (i as i64, num.parse::<i64>().unwrap()))
        .collect();

    let (p1, oxygen_robot) = find_oxygen_system(&instructions);

    println!("{p1}");

    let p2: u32 = explore(&oxygen_robot);

    println!("{p2}");
}

fn find_oxygen_system(instructions: &HashMap<i64, i64>) -> (u32, Robot) {
    let robot: Robot = Robot {
        instructions: instructions.clone(),
        pointer: 0,
        inputs: VecDeque::new(),
        state: RobotState::WaitingOnInput,
        relative_base: 0,
    };

    let mut nodes: VecDeque<(Robot, i32, i32, u32)> = VecDeque::from([(robot.clone(), 0, 0, 0)]);
    let mut shortest: u32 = u32::MAX;
    let mut visited: HashMap<(i32, i32), u32> = HashMap::from([((0, 0), 0)]);

    let mut oxygen_robot: Robot = robot.clone();

    let directions: HashMap<i64, (i32, i32)> =
        HashMap::from([(1, (0, 1)), (2, (0, -1)), (3, (-1, 0)), (4, (1, 0))]);

    while nodes.len() > 0 {
        let (robot, x, y, steps) = nodes.pop_front().unwrap();

        if steps > shortest {
            continue;
        }

        for (direction, (dx, dy)) in &directions {
            let x2: i32 = x + dx;
            let y2: i32 = y + dy;

            let mut robot2: Robot = robot.clone();

            robot2.inputs.push_back(*direction);

            let output: i64 = robot2.run()[0];

            if output == 1 && (!visited.contains_key(&(x2, y2)) || steps + 1 < visited[&(x2, y2)]) {
                nodes.push_back((robot2, x2, y2, steps + 1));
                visited.insert((x2, y2), steps + 1);
            } else if output == 2 {
                shortest = shortest.min(steps + 1);
                oxygen_robot = robot2
            }
        }
    }

    (shortest, oxygen_robot)
}

fn explore(robot: &Robot) -> u32 {
    let robot: Robot = robot.clone();

    let mut nodes: VecDeque<(Robot, i32, i32, u32)> = VecDeque::from([(robot, 0, 0, 0)]);
    let mut visited: HashMap<(i32, i32), u32> = HashMap::from([((0, 0), 0)]);

    let directions: HashMap<i64, (i32, i32)> =
        HashMap::from([(1, (0, 1)), (2, (0, -1)), (3, (-1, 0)), (4, (1, 0))]);

    while nodes.len() > 0 {
        let (robot, x, y, steps) = nodes.pop_front().unwrap();

        for (direction, (dx, dy)) in &directions {
            let x2: i32 = x + dx;
            let y2: i32 = y + dy;

            let mut robot2: Robot = robot.clone();

            robot2.inputs.push_back(*direction);

            let output: i64 = robot2.run()[0];

            if output == 1 && (!visited.contains_key(&(x2, y2)) || steps + 1 < visited[&(x2, y2)]) {
                nodes.push_back((robot2, x2, y2, steps + 1));
                visited.insert((x2, y2), steps + 1);
            }
        }
    }

    *visited.values().max().unwrap()
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
