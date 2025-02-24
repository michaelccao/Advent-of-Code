use crate::helper::read_data;
use std::collections::{HashMap, HashSet};
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day23.txt");

    let instructions: Vec<(&str, &str, &str)> = get_instructions(&data);

    let p1 = run_instructions(&instructions, 0).0;
    let p2 = run_instructions(&instructions, 1).1;

    println!("{p1}\n{p2}");

    // Instructions analysis with a = 1
    // Lines 1-8: Set b to 108100 and c to 125100
    // Lines 9-20: Set g=0, d=2, 
}

fn get_instructions(data: &String) -> Vec<(&str, &str, &str)> {
    let mut instructions: Vec<(&str, &str, &str)> = Vec::new();

    for line in data.lines() {
        let mut line = line.trim().split_whitespace();
        let command = line.next().unwrap();
        let param1 = line.next().unwrap();
        let param2 = line.next().unwrap();

        instructions.push((command, param1, param2));
    }

    instructions
}

fn run_instructions(instructions: &Vec<(&str, &str, &str)>, a_start: i64) -> (u32, i64) {

    let mut registers: HashMap<&str, i64> = HashMap::new();

    registers.insert("a", a_start);

    let mut pointer: i64 = 0;

    let num_instructions: i64 = instructions.len() as i64;

    let mut mul_runs: u32 = 0;

    while pointer >= 0 && pointer < num_instructions {
        let muls: u32;

        (registers, pointer, muls) = execute_instruction(instructions, registers, pointer);

        mul_runs += muls;
    }

    (mul_runs, *registers.get("h").unwrap_or(&0))

}

fn get_value(param: &str, registers: &HashMap<&str, i64>) -> i64 {

    if let Ok(val) = param.parse::<i64>() {
        val
    } else {
        *registers.get(param).unwrap_or(&0)
    }

}

fn execute_instruction<'a>(instructions: &Vec<(&'a str, &'a str, &'a str)>, mut registers: HashMap<&'a str, i64>, mut pointer: i64) -> (HashMap<&'a str, i64>, i64, u32) {

    let (command, param1, param2) = instructions[pointer as usize];
    let mut muls: u32 = 0;

    match command {
        "set" => {
            registers.insert(param1, get_value(param2, &registers));
        },
        "sub" => {
            registers.insert(param1, get_value(param1, &registers) - get_value(param2, &registers));
        },
        "mul" => {
            registers.insert(param1, get_value(param1, &registers)*get_value(param2, &registers));
            muls += 1;
        },
        "jnz" => {
            if get_value(param1, &registers) != 0 {
                let jump = get_value(param2, &registers);

                if jump >= 0 || param1.parse::<i64>().is_ok() {
                    pointer += jump - 1;
                } else {
                    (registers, muls) = resolve_loop(instructions, pointer, registers);
                }
                
            }
        }
        _ => {},
    }

    pointer += 1;

    (registers, pointer, muls)

}

fn resolve_loop<'a>(instructions: &Vec<(&'a str, &'a str, &'a str)>, pointer: i64, mut registers: HashMap<&'a str, i64>) -> (HashMap<&'a str, i64>, u32) {
    let (_, key_register, jump) = instructions[pointer as usize];

    let mut pointer2: i64 = pointer + get_value(jump, &registers);

    let mut registers2: HashMap<&str, i64> = registers.clone();

    let mut mul_runs: u32 = 0;

    while pointer2 != pointer {
        let muls: u32;
        (registers, pointer2, muls) = execute_instruction(instructions, registers, pointer2);

        mul_runs += muls;
        
    }

    let cycles: i64 = registers2[key_register] / (registers2[key_register] - registers[key_register]);

    for &reg in registers.keys() {
        let original_value = registers2[reg];
        let new_value = registers[reg];
        let final_value = original_value + (new_value - original_value)*cycles;

        registers2.insert(reg, final_value);
    }

    // Edge case for loop involving Line 15/16
    if registers["b"] % registers["d"] == 0 && pointer == 19 {
        registers2.insert("f", 0);
    }

    (registers2, mul_runs*(cycles as u32))

}