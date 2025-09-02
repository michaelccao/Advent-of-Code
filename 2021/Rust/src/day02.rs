use crate::helper::read_data;

pub fn main() {
    let data: String = read_data("../Data/Day02.txt");

    let instructions = get_instructions(&data);

    let p1: u32 = follow_instructions(&instructions);

    println!("{p1}");

    let p2: u32 = follow_instructions2(&instructions);

    println!("{p2}");
}

fn get_instructions(data: &String) -> Vec<(String, u32)> {
    let mut instructions: Vec<(String, u32)> = Vec::new();

    for line in data.lines() {
        let mut line = line.split(" ");
        let movement: String = line.next().unwrap().to_string();
        let amount: u32 = line.next().unwrap().parse().unwrap();

        instructions.push((movement, amount))
    }

    instructions
}

fn follow_instructions(instructions: &Vec<(String, u32)>) -> u32 {
    let mut x: u32 = 0;
    let mut y: u32 = 0;

    for (movement, amount) in instructions {
        if movement == "forward" {
            x += amount;
        } else if movement == "down" {
            y += amount;
        } else if movement == "up" {
            y -= amount;
        }
    }

    x * y
}

fn follow_instructions2(instructions: &Vec<(String, u32)>) -> u32 {
    let mut x: u32 = 0;
    let mut y: u32 = 0;
    let mut aim: u32 = 0;

    for (movement, amount) in instructions {
        if movement == "forward" {
            x += amount;
            y += aim * amount;
        } else if movement == "down" {
            aim += amount;
        } else if movement == "up" {
            aim -= amount;
        }
    }

    x * y
}
