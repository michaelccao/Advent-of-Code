use crate::helper::read_data;
use std::collections::HashMap;
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day23.txt");

    let instructions: Vec<(&str, &str, i32)> = get_instructions(&data);

    let p1: u32 = follow_instructions(&instructions, false);
    let p2: u32 = follow_instructions(&instructions, true);

    println!("{p1}\n{p2}");
}

fn get_instructions(data: &String) -> Vec<(&str, &str, i32)> {
    let mut instructions: Vec<(&str, &str, i32)> = Vec::new();

    for line in data.lines() {
        let s: Vec<&str> = line.trim().split(' ').collect();

        let instruction: &str = s[0];
        let register: &str = if instruction != "jmp" {
            s[1].trim_matches(',')
        } else {
            ""
        };
        let offset: i32 = if s.len() == 2 && instruction != "jmp" {
            0
        } else if instruction == "jmp" {
            s[1].parse().unwrap()
        } else {
            s[2].parse().unwrap()
        };

        instructions.push((instruction, register, offset));
    }

    instructions
}

fn follow_instructions<'a>(instructions: &Vec<(&str, &str, i32)>, part2: bool) -> u32 {
    let mut pointer: i32 = 0;

    let mut registers: HashMap<&str, u32> =
        HashMap::from([("a", if part2 { 1 } else { 0 }), ("b", 0), ("", 0)]);

    while pointer >= 0 && pointer < instructions.len() as i32 {
        let (cmd, register, offset): (&str, &str, i32) = instructions[pointer as usize];

        let register: &mut u32 = registers.get_mut(register).unwrap();

        match cmd {
            "hlf" => *register /= 2,
            "tpl" => *register *= 3,
            "inc" => *register += 1,
            "jmp" => {
                pointer += offset;
                continue;
            }
            "jie" => {
                if *register % 2 == 0 {
                    pointer += offset;
                    continue;
                }
            }
            "jio" => {
                if *register == 1 {
                    pointer += offset;
                    continue;
                }
            }
            _ => {}
        }

        pointer += 1;
    }

    registers["b"]
}
