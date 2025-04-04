use crate::helper::read_data;
use std::collections::{HashMap, HashSet};
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day16.txt");
    let (samples, program): (Vec<Sample>, Vec<Vec<u32>>) = get_samples_and_program(&data);

    let p1 = samples
        .iter()
        .filter(|&sample| sample.test_ops())
        .collect::<Vec<_>>()
        .len();

    println!("{p1}");

    let decoder: Vec<u32> = decode(&samples);

    let p2: u32 = execute_program(&program, &decoder);

    println!("{p2}");
}

#[derive(Debug)]
struct Sample {
    code: Vec<u32>,
    before: Vec<u32>,
    after: Vec<u32>,
}

impl Sample {
    fn test_ops(&self) -> bool {
        self.get_valid_ops().len() >= 3
    }

    fn get_valid_ops(&self) -> HashSet<u32> {
        let mut code: Vec<u32> = self.code.clone();
        let mut valid_ops: HashSet<u32> = HashSet::new();

        for op in 0..16 {
            code[0] = op;
            if operate(&code, self.before.clone()) == self.after {
                valid_ops.insert(op);
            }
        }

        valid_ops
    }
}

fn get_samples_and_program(data: &String) -> (Vec<Sample>, Vec<Vec<u32>>) {
    let data: Vec<&str> = data.lines().collect();
    let mut samples: Vec<Sample> = Vec::new();

    let mut line_num: usize = 0;

    while line_num < data.len() {
        if data[line_num].trim().len() == 0 {
            break;
        }
        let before: Vec<u32> = data[line_num][9..]
            .trim_end_matches(']')
            .split(", ")
            .map(|num| num.parse::<u32>().unwrap())
            .collect();
        let code: Vec<u32> = data[line_num + 1]
            .split_whitespace()
            .map(|num| num.parse::<u32>().unwrap())
            .collect();
        let after: Vec<u32> = data[line_num + 2][9..]
            .trim_end_matches(']')
            .split(", ")
            .map(|num| num.parse::<u32>().unwrap())
            .collect();

        let sample = Sample {
            code,
            before,
            after,
        };

        samples.push(sample);

        line_num += 4;
    }

    line_num += 2;

    let mut codes: Vec<Vec<u32>> = Vec::new();

    while line_num < data.len() {
        let code: Vec<u32> = data[line_num]
            .split_whitespace()
            .map(|num| num.parse::<u32>().unwrap())
            .collect();

        codes.push(code);

        line_num += 1;
    }

    (samples, codes)
}

fn operate(code: &Vec<u32>, mut registers: Vec<u32>) -> Vec<u32> {
    let op: u32 = code[0];
    let a: u32 = code[1];
    let b: u32 = code[2];
    let c: usize = code[3] as usize;

    match op {
        0 => registers[c] = registers[a as usize] + registers[b as usize],
        1 => registers[c] = registers[a as usize] + b,
        2 => registers[c] = registers[a as usize] * registers[b as usize],
        3 => registers[c] = registers[a as usize] * b,
        4 => registers[c] = registers[a as usize] & registers[b as usize],
        5 => registers[c] = registers[a as usize] & b,
        6 => registers[c] = registers[a as usize] | registers[b as usize],
        7 => registers[c] = registers[a as usize] | b,
        8 => registers[c] = registers[a as usize],
        9 => registers[c] = a,
        10 => registers[c] = if a > registers[b as usize] { 1 } else { 0 },
        11 => registers[c] = if registers[a as usize] > b { 1 } else { 0 },
        12 => {
            registers[c] = if registers[a as usize] > registers[b as usize] {
                1
            } else {
                0
            }
        }
        13 => registers[c] = if a == registers[b as usize] { 1 } else { 0 },
        14 => registers[c] = if registers[a as usize] == b { 1 } else { 0 },
        15 => {
            registers[c] = if registers[a as usize] == registers[b as usize] {
                1
            } else {
                0
            }
        }
        _ => {}
    }

    registers
}

fn decode(samples: &Vec<Sample>) -> Vec<u32> {
    let mut candidates: HashMap<u32, HashSet<u32>> = HashMap::new();

    for sample in samples {
        let valid_ops: HashSet<u32> = sample.get_valid_ops();
        let op: u32 = sample.code[0];

        if let Some(valid) = candidates.get_mut(&op) {
            *valid = valid
                .intersection(&valid_ops)
                .cloned()
                .collect::<HashSet<u32>>();
        } else {
            candidates.insert(op, valid_ops);
        }
    }

    let mut decoder: HashMap<u32, u32> = HashMap::new();

    while decoder.len() < 16 {
        for (&op, cands) in &candidates {
            if cands.len() == 1 {
                decoder.insert(op, *cands.iter().next().unwrap());
            }
        }

        for (_, true_op) in &decoder {
            for op in 0..16 {
                candidates.get_mut(&op).unwrap().remove(&true_op);
            }
        }
    }

    (0..16).map(|op| decoder[&op]).collect::<Vec<u32>>()
}

fn execute_program(program: &Vec<Vec<u32>>, decoder: &Vec<u32>) -> u32 {
    let mut registers: Vec<u32> = vec![0; 4];

    for code in program {
        let mut code: Vec<u32> = code.clone();
        code[0] = decoder[code[0] as usize];

        registers = operate(&code, registers)
    }

    registers[0]
}
