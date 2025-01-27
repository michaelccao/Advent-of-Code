use crate::helper::read_data;
use std::collections::HashMap;
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day08.txt");

    let commands: Vec<Vec<&str>> = data
        .lines()
        .map(|line| line.trim().split_whitespace().collect::<Vec<&str>>())
        .collect();

    let (p1, p2): (i32, i32) = follow_commands(&commands);

    println!("{p1}\n{p2}");
}

fn follow_commands(commands: &Vec<Vec<&str>>) -> (i32, i32) {
    let mut registers: HashMap<&str, i32> = HashMap::new();

    let mut largest: i32 = 0;

    for command in commands {
        let reg1: &str = command[0];
        let op: &str = command[1];
        let delta: i32 = command[2].parse().unwrap();

        let reg2: &str = command[4];
        let comp: &str = command[5];
        let check: i32 = command[6].parse().unwrap();

        let reg1_value: i32 = if let Some(x) = registers.get(reg1) {
            *x
        } else {
            registers.insert(reg1, 0);
            0
        };

        let reg2_value: i32 = if let Some(y) = registers.get(reg2) {
            *y
        } else {
            registers.insert(reg2, 0);
            0
        };

        let execute: bool = match comp {
            "==" => reg2_value == check,
            "!=" => reg2_value != check,
            ">" => reg2_value > check,
            ">=" => reg2_value >= check,
            "<" => reg2_value < check,
            "<=" => reg2_value <= check,
            _ => false,
        };

        let new_val: i32 = match op {
            "inc" => reg1_value + delta,
            "dec" => reg1_value - delta,
            _ => reg1_value,
        };

        if execute {
            registers.insert(reg1, new_val);

            if new_val > largest {
                largest = new_val;
            }
        }
    }

    (*registers.values().max().unwrap(), largest)
}
