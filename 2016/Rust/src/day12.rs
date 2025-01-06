use crate::helper::read_data;
use std::collections::HashMap;
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day12.txt");

    let instructions: Vec<&str> = data.lines().map(|line| line.trim()).collect();

    let p1: i32 = follow_instructions(&instructions, 0);
    let p2: i32 = follow_instructions(&instructions, 1);

    println!("{p1}\n{p2}");
}

fn follow_instructions(instructions: &Vec<&str>, c_register: i32) -> i32 {
    let mut registers: HashMap<&str, i32> =
        HashMap::from([("a", 0), ("b", 0), ("c", c_register), ("d", 0)]);

    let mut pointer: i32 = 0;

    while pointer >= 0 && pointer < instructions.len() as i32 {
        let mut instruction = instructions[pointer as usize].split_whitespace();
        let command = instruction.next().unwrap();

        match command {
            "cpy" => {
                let value: &str = instruction.next().unwrap();
                let register: &str = instruction.next().unwrap();

                if let Ok(val) = value.parse::<i32>() {
                    registers.insert(register, val);
                } else {
                    registers.insert(register, registers[value]);
                }
            }
            "inc" => {
                let register: &str = instruction.next().unwrap();

                *registers.get_mut(register).unwrap() += 1;
            }
            "dec" => {
                let register: &str = instruction.next().unwrap();

                *registers.get_mut(register).unwrap() -= 1;
            }
            "jnz" => {
                let register: &str = instruction.next().unwrap();

                let jump: i32 = instruction.next().unwrap().parse().unwrap();

                if let Ok(val) = register.parse::<i32>() {
                    if val != 0 {
                        pointer += jump;
                        continue;
                    }
                } else {
                    if registers[register] != 0 {
                        pointer += jump;
                        continue;
                    }
                }
            }
            _ => {}
        }

        pointer += 1;
    }

    registers["a"]
}
