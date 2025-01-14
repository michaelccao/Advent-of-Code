use crate::helper::read_data;
use std::collections::{HashMap, HashSet};
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day23.txt");

    let instructions = get_instructions(&data);

    let p1 = follow_instructions(&instructions, 7);

    println!("{p1}");

    // let p2 = follow_instructions(&instructions, 12);

    // println!("{p2}");
}

fn get_instructions(data: &String) -> Vec<[String;3]> {

    let mut instructions: Vec<[String;3]> = Vec::new();

    for line in data.lines() {
        let mut line = line.trim().split_whitespace();
        let cmd = line.next().unwrap().to_string();
        let param1 = line.next().unwrap().to_string();
        let param2 = line.next().unwrap_or_default().to_string();

        instructions.push([cmd, param1, param2]);

    }

    instructions

}

fn follow_instructions(instructions: &Vec<[String;3]>, start: i32) -> i32 {
    let mut instructions = instructions.clone();

    let mut registers: HashMap<String, i32> = HashMap::new();

    registers.insert("a".to_string(), start);

    let mut pointer: i32 = 0;

    while pointer >= 0 && pointer < instructions.len() as i32 {
        (pointer, instructions, registers) = execute_instruction(pointer, instructions, registers);
    }

    registers[&"a".to_string()]
}

fn execute_instruction(mut pointer: i32, mut instructions: Vec<[String;3]>, mut registers: HashMap<String, i32>) -> (i32, Vec<[String;3]>, HashMap<String, i32>) {
    let [cmd, p, q] = &instructions[pointer as usize];

    let (p_is_reg, p_val) = if let Ok(x) = p.parse::<i32>() {
        (false, x)
    } else {
        (true, registers[p])
    };

    let (q_is_reg, q_val) = if let Ok(y) = q.parse::<i32>() {
        (false, y)
    } else {
        (true, registers.get(q).cloned().unwrap_or_default())
    };

    match &cmd[..] {
        "cpy" => {
            if q_is_reg {
                registers.insert(q.to_owned(), p_val);
            }
        },
        "inc" => {
            if p_is_reg {
                let val = registers.get_mut(p).unwrap();
                *val += 1;
            }
        },
        "dec" => {
            if p_is_reg {
                let val = registers.get_mut(p).unwrap();
                *val -= 1;
            }
        },
        "jnz" => {
            if p_val != 0 {
                pointer += q_val;

                return (pointer, instructions, registers);
            }
        },
        "tgl" => {
            let pointer2 = pointer + p_val;

            if pointer2 >= instructions.len() as i32 || pointer2 < 0 {
                pointer += 1;
                
                return (pointer, instructions, registers);
            }

            let [cmd2, _, q2] = &mut instructions[pointer2 as usize];

            if *q2 == String::default() {
                if cmd2 == "inc" {
                    *cmd2 = "dec".to_string();
                } else {
                    *cmd2 = "inc".to_string();
                }
            } else {
                if cmd2 == "jnz" {
                    *cmd2 = "cpy".to_string();
                } else {
                    *cmd2 = "jnz".to_string();
                }
            }

        }
        _ => {},
    }

    pointer += 1;

    (pointer, instructions, registers)

}

// A cycle can occur with a negative jump
// We assume that our cycles cause linear changes
// For simplicity, we only look at "jnz register constant" commands
fn resolve_cycle(pointer: i32, mut registers: HashMap<String, i32>) {

    let mut end_registers = registers.clone();

}