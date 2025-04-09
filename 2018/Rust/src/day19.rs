use crate::helper::read_data;
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day19.txt");

    let (instructions, pointer_reg) = get_instructions(&data);

    let p1: usize = execute_program(&instructions, pointer_reg);

    println!("{p1}");

    // Analysis of program shows that it generates a big number
    // Then loops through each number to see if it's a divisor
    // and adds it to Register 0

    // In our specific case, the big number was 10551320
    let p2: usize = add_divisors(10551320);

    println!("{p2}");
}

fn get_instructions(data: &String) -> (Vec<(&str, usize, usize, usize)>, usize) {
    let mut pointer_reg: usize = 0;
    let mut instructions: Vec<(&str, usize, usize, usize)> = Vec::new();

    let lines: Vec<&str> = data.lines().collect();

    for i in 0..lines.len() {
        if i == 0 {
            pointer_reg = lines[i].split_whitespace().collect::<Vec<&str>>()[1]
                .parse::<usize>()
                .unwrap();
        } else {
            let line: Vec<&str> = lines[i].trim().split_whitespace().collect();
            let a: usize = line[1].parse().unwrap();
            let b: usize = line[2].parse().unwrap();
            let c: usize = line[3].parse().unwrap();

            instructions.push((line[0], a, b, c));
        }
    }

    (instructions, pointer_reg)
}

fn execute_program(instructions: &Vec<(&str, usize, usize, usize)>, pointer_reg: usize) -> usize {
    let mut registers: Vec<usize> = vec![0; 6];

    let mut pointer: usize = registers[pointer_reg];

    while pointer < instructions.len() {
        registers[pointer_reg] = pointer;
        registers = operate(instructions[pointer], &registers);
        pointer = registers[pointer_reg];
        pointer += 1;
    }

    registers[0]
}

fn operate(instruction: (&str, usize, usize, usize), registers: &Vec<usize>) -> Vec<usize> {
    let mut registers: Vec<usize> = registers.clone();

    let (op, a, b, c) = instruction;

    match op {
        "addr" => registers[c] = registers[a] + registers[b],
        "addi" => registers[c] = registers[a] + b,
        "mulr" => registers[c] = registers[a] * registers[b],
        "muli" => registers[c] = registers[a] * b,
        "banr" => registers[c] = registers[a] & registers[b],
        "bani" => registers[c] = registers[a] & b,
        "borr" => registers[c] = registers[a] | registers[b],
        "bori" => registers[c] = registers[a] | b,
        "setr" => registers[c] = registers[a],
        "seti" => registers[c] = a,
        "gtir" => registers[c] = if a > registers[b] { 1 } else { 0 },
        "gtri" => registers[c] = if registers[a] > b { 1 } else { 0 },
        "gtrr" => registers[c] = if registers[a] > registers[b] { 1 } else { 0 },
        "eqir" => registers[c] = if a == registers[b] { 1 } else { 0 },
        "eqri" => registers[c] = if registers[a] == b { 1 } else { 0 },
        "eqrr" => registers[c] = if registers[a] == registers[b] { 1 } else { 0 },
        _ => {}
    }

    registers
}

fn add_divisors(n: usize) -> usize {
    (1..n + 1).filter(|&num| n % num == 0).sum()
}
