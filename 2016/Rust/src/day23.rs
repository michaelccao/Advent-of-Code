use crate::helper::read_data;
use std::collections::{HashMap, HashSet};
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day23.txt");

    let instructions = get_instructions(&data);

    let p1 = follow_instructions(&instructions, 7);

    println!("{p1}");

    let p2 = follow_instructions(&instructions, 12);

    println!("{p2}");
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
        let [cmd, p, q] = &instructions[pointer as usize];

        match &cmd[..] {
            "cpy" => {
                let p = if let Ok(x) = p.parse::<i32>() {
                    x
                } else {
                    registers[p]
                };

                if q.parse::<i32>().is_err() {
                    registers.insert(q.to_owned(), p);
                }
            },
            "inc" => {
                if p.parse::<i32>().is_err() {
                    let val = registers.get_mut(p).unwrap();
                    *val += 1;
                }
            },
            "dec" => {
                if p.parse::<i32>().is_err() {
                    let val = registers.get_mut(p).unwrap();
                    *val -= 1;
                }
            },
            "jnz" => {
                let p = if let Ok(x) = p.parse::<i32>() {
                    x
                } else {
                    registers[p]
                };

                let q = if let Ok(y) = q.parse::<i32>() {
                    y
                } else {
                    registers[q]
                };

                if p != 0 {
                    pointer += q;
                    continue;
                }
            },
            "tgl" => {
                let p_parse = p.parse::<i32>();
                let pointer2 = pointer + if p_parse.is_ok() {p_parse.unwrap()} else {registers[p]};

                if pointer2 >= instructions.len() as i32 || pointer2 < 0 {
                    pointer += 1;
                    continue;
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
    }

    registers[&"a".to_string()]
}