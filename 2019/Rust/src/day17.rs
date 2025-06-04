use crate::helper::read_data;
use std::collections::{HashMap, VecDeque};

pub fn main() {
    let data: String = read_data("../Data/Day17.txt");
    let instructions: HashMap<i64, i64> = data
        .split(",")
        .enumerate()
        .map(|(i, num)| (i as i64, num.parse::<i64>().unwrap()))
        .collect();

    let grid: Vec<Vec<char>> = get_map(&instructions);

    let p1: usize = calculate_alignment_parameters(&grid);

    println!("{p1}");

    let p2: i64 = run_route(&instructions);

    println!("{p2}");
}

// Thesis: There exists a route where each non-intersection is visited only once
// and intersections are visited only twice
fn run_route(instructions: &HashMap<i64, i64>) -> i64 {
    // Upon visual inspection, the route is
    // R,10,R,10,R,6,R,4,R,10,R,10,L,4,R,10,R,10,R,6,R,4,R,4,L,4,L,10,L,10,R,10,R,10,R,6,
    // R,4,R,10,R,10,L,4,R,4,L,4,L,10,L,10,R,10,R,10,L,4,R,4,L,4,L,10,L,10,R,10,R,10,L,4

    // A: R,10,R,10,R,6,R,4
    // B: R,10,R,10,L,4
    // C: R,4,L,4,L,10,L,10

    // A,B,A,C,A,B,C,B,C,B

    let a: &str = "R,10,R,10,R,6,R,4";
    let b: &str = "R,10,R,10,L,4";
    let c: &str = "R,4,L,4,L,10,L,10";

    let routine: &str = "A,B,A,C,A,B,C,B,C,B";

    let mut robot = Robot {
        instructions: instructions.clone(),
        pointer: 0,
        inputs: VecDeque::new(),
        state: RobotState::WaitingOnInput,
        relative_base: 0,
    };

    robot.instructions.insert(0, 2);

    // Input movement instructions
    for subroutine in [routine, a, b, c] {
        for command in subroutine.chars() {
            robot.inputs.push_back(command as u8 as i64);
        }
        robot.inputs.push_back(10);
    }

    // No playback
    robot.inputs.push_back('n' as u8 as i64);
    robot.inputs.push_back(10);

    let output: Vec<i64> = robot.run();

    *output.last().unwrap()
}

fn calculate_alignment_parameters(grid: &Vec<Vec<char>>) -> usize {
    let mut total: usize = 0;

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] != '#' {
                continue;
            }

            if i > 0
                && i + 1 < grid.len()
                && j > 0
                && j + 1 < grid[i].len()
                && grid[i - 1][j] == '#'
                && grid[i + 1][j] == '#'
                && grid[i][j - 1] == '#'
                && grid[i][j + 1] == '#'
            {
                total += i * j;
            }
        }
    }

    total
}

fn get_map(instructions: &HashMap<i64, i64>) -> Vec<Vec<char>> {
    let mut robot = Robot {
        instructions: instructions.clone(),
        pointer: 0,
        inputs: VecDeque::new(),
        state: RobotState::WaitingOnInput,
        relative_base: 0,
    };

    let outputs: Vec<i64> = robot.run();

    let grid_str: String = outputs.iter().map(|&o| o as u8 as char).collect();

    // Visual inspection used for solution
    // Copy-pasted output to separate notepad and used ^,>,v,< to draw out path
    // println!("{grid_str}");

    let mut grid: Vec<Vec<char>> = grid_str
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    grid.pop(); // Removes last empty row

    grid
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
