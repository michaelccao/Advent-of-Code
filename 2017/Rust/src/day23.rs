use crate::helper::read_data;
use std::collections::HashMap;
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day23.txt");

    let instructions: Vec<(&str, &str, &str)> = get_instructions(&data);

    let p1: u32 = run_instructions(&instructions);
    let p2: u32 = part2();

    println!("{p1}\n{p2}");
}

fn get_instructions(data: &String) -> Vec<(&str, &str, &str)> {
    let mut instructions: Vec<(&str, &str, &str)> = Vec::new();

    for line in data.lines() {
        let mut line = line.trim().split_whitespace();
        let command: &str = line.next().unwrap();
        let param1: &str = line.next().unwrap();
        let param2: &str = line.next().unwrap();

        instructions.push((command, param1, param2));
    }

    instructions
}

fn run_instructions(instructions: &Vec<(&str, &str, &str)>) -> u32 {
    let mut registers: HashMap<&str, i64> = HashMap::new();

    let mut pointer: i64 = 0;

    let num_instructions: i64 = instructions.len() as i64;

    let mut mul_runs: u32 = 0;

    while pointer >= 0 && pointer < num_instructions {
        let muls: u32;

        (registers, pointer, muls) = execute_instruction(instructions, registers, pointer);

        mul_runs += muls;
    }

    mul_runs
}

fn get_value(param: &str, registers: &HashMap<&str, i64>) -> i64 {
    if let Ok(val) = param.parse::<i64>() {
        val
    } else {
        *registers.get(param).unwrap_or(&0)
    }
}

fn execute_instruction<'a>(
    instructions: &Vec<(&'a str, &'a str, &'a str)>,
    mut registers: HashMap<&'a str, i64>,
    mut pointer: i64,
) -> (HashMap<&'a str, i64>, i64, u32) {
    let (command, param1, param2) = instructions[pointer as usize];
    let mut muls: u32 = 0;

    match command {
        "set" => {
            registers.insert(param1, get_value(param2, &registers));
        }
        "sub" => {
            registers.insert(
                param1,
                get_value(param1, &registers) - get_value(param2, &registers),
            );
        }
        "mul" => {
            registers.insert(
                param1,
                get_value(param1, &registers) * get_value(param2, &registers),
            );
            muls += 1;
        }
        "jnz" => {
            if get_value(param1, &registers) != 0 {
                let jump = get_value(param2, &registers);

                pointer += jump - 1;
            }
        }
        _ => {}
    }

    pointer += 1;

    (registers, pointer, muls)
}

// Analysis of program reveals it simply counts composite numbers from B to C inclusive
// in steps of 17

fn part2() -> u32 {
    let mut composites: u32 = 0;

    let mut b: u32 = 81 * 100 + 100000;
    let c: u32 = b + 17000;

    while b <= c {
        if is_composite(b) {
            composites += 1;
        }

        b += 17;
    }

    composites
}

fn is_composite(n: u32) -> bool {
    let root = (n as f32).sqrt() as u32;

    for d in 2..root + 1 {
        if n % d == 0 {
            return true;
        }
    }

    false
}
