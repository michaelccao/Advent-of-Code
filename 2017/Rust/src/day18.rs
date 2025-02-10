use crate::helper::read_data;
use std::collections::VecDeque;
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day18.txt");
    let instructions: Vec<Vec<&str>> = data
        .lines()
        .map(|line| line.trim().split_whitespace().collect())
        .collect();

    let p1: i64 = follow_instructions(&instructions);

    let p2 = deadlock(&instructions);

    println!("{p1}\n{p2}");
}

fn deadlock(instructions: &Vec<Vec<&str>>) -> u32 {
    let mut p1_sends: u32 = 0;

    let mut p0_regs: Vec<i64> = vec![0; 26];
    let mut p1_regs: Vec<i64> = vec![0; 26];

    p1_regs['p' as usize - 'a' as usize] = 1;

    let mut p0_queue: VecDeque<i64> = VecDeque::new();
    let mut p1_queue: VecDeque<i64> = VecDeque::new();

    let mut pointer0: usize = 0;
    let mut pointer1: usize = 0;

    while pointer0 < instructions.len() && pointer1 < instructions.len() {
        let new_sends: u32;

        (p0_regs, pointer0, p1_queue, p0_queue, _) =
            follow_instructions2(p0_regs, instructions, pointer0, p1_queue, p0_queue);
        (p1_regs, pointer1, p0_queue, p1_queue, new_sends) =
            follow_instructions2(p1_regs, instructions, pointer1, p0_queue, p1_queue);

        p1_sends += new_sends;

        if instructions[pointer0][0] == "rcv"
            && p0_queue.len() == 0
            && instructions[pointer1][0] == "rcv"
            && p1_queue.len() == 0
        {
            break;
        }
    }

    p1_sends
}

fn follow_instructions2(
    mut registers: Vec<i64>,
    instructions: &Vec<Vec<&str>>,
    mut pointer: usize,
    mut sends: VecDeque<i64>,
    mut queue: VecDeque<i64>,
) -> (Vec<i64>, usize, VecDeque<i64>, VecDeque<i64>, u32) {
    let a: usize = 'a' as usize;
    let mut new_sends: u32 = 0;

    while pointer < instructions.len() {
        let command: &str = instructions[pointer][0];
        let param1: &str = instructions[pointer][1];

        match command {
            "snd" => {
                sends.push_back(value(param1, &registers));
                new_sends += 1;
            }
            "set" => {
                registers[param1.chars().next().unwrap() as usize - a] =
                    value(instructions[pointer][2], &registers);
            }
            "add" => {
                registers[param1.chars().next().unwrap() as usize - a] +=
                    value(instructions[pointer][2], &registers);
            }
            "mul" => {
                registers[param1.chars().next().unwrap() as usize - a] *=
                    value(instructions[pointer][2], &registers);
            }
            "mod" => {
                registers[param1.chars().next().unwrap() as usize - a] %=
                    value(instructions[pointer][2], &registers);
            }
            "rcv" => {
                if queue.len() > 0 {
                    registers[param1.chars().next().unwrap() as usize - a] =
                        queue.pop_front().unwrap();
                } else {
                    return (registers, pointer, sends, queue, new_sends);
                }
            }
            "jgz" => {
                if value(param1, &registers) > 0 {
                    pointer += value(instructions[pointer][2], &registers) as usize;
                    continue;
                }
            }
            _ => {}
        }

        pointer += 1;
    }

    (registers, pointer, sends, queue, new_sends)
}

fn follow_instructions(instructions: &Vec<Vec<&str>>) -> i64 {
    let mut pointer: usize = 0;

    let mut registers: Vec<i64> = vec![0; 26];

    let a: usize = 'a' as usize;

    let mut played: Option<i64> = None;

    while pointer < instructions.len() {
        let command: &str = instructions[pointer][0];
        let param1: &str = instructions[pointer][1];

        match command {
            "snd" => {
                played = Some(value(param1, &registers));
            }
            "set" => {
                registers[param1.chars().next().unwrap() as usize - a] =
                    value(instructions[pointer][2], &registers);
            }
            "add" => {
                registers[param1.chars().next().unwrap() as usize - a] +=
                    value(instructions[pointer][2], &registers);
            }
            "mul" => {
                registers[param1.chars().next().unwrap() as usize - a] *=
                    value(instructions[pointer][2], &registers);
            }
            "mod" => {
                registers[param1.chars().next().unwrap() as usize - a] %=
                    value(instructions[pointer][2], &registers);
            }
            "rcv" => {
                if value(param1, &registers) != 0 {
                    return played.unwrap();
                }
            }
            "jgz" => {
                if value(param1, &registers) > 0 {
                    pointer += value(instructions[pointer][2], &registers) as usize;
                    continue;
                }
            }
            _ => {}
        }

        pointer += 1;
    }

    0
}

fn value(param: &str, registers: &Vec<i64>) -> i64 {
    let a: usize = 'a' as usize;

    if let Ok(val) = param.parse::<i64>() {
        val
    } else {
        let register = param.chars().next().unwrap() as usize - a;

        registers[register]
    }
}
