use crate::helper::read_data;
use std::collections::HashMap;
use std::vec::Vec;

pub fn main() {
    let data = read_data("../Data/Day17.txt");

    let (a, b, c, program) = get_registers_and_program(&data);

    let p1 = execute_program(a, b, c, &program);

    let bc_map = get_bc_combos();

    let p2 = determine_bits(&program, &bc_map);

    println!("{p1:?}\n{p2}");
}

fn get_registers_and_program(data: &String) -> (u64, u64, u64, Vec<u64>) {

    let split: Vec<_> = data.split("\r\n\r\n").collect();

    let registers = split[0].split("\r\n").map(|line| line.split(": ").collect::<Vec<_>>()[1].parse::<u64>().unwrap()).collect::<Vec<u64>>();
    let program = split[1].split(": ").collect::<Vec<_>>()[1].split(",").map(|num| num.parse::<u64>().unwrap()).collect::<Vec<u64>>();

    (registers[0], registers[1], registers[2], program)
}

fn execute_program(a: u64, b: u64, c: u64, program: &Vec<u64>) -> Vec<u64> {

    let mut combo: Vec<u64> = vec![0, 1, 2, 3, a, b, c];

    let mut outputs: Vec<u64> = Vec::new();

    let mut pointer:usize = 0;

    while pointer < program.len()-1 {
        let opcode = program[pointer];
        let operand = program[pointer+1];

        match opcode {
            0 => combo[4] >>= combo[operand as usize],
            1 => combo[5] ^= operand,
            2 => combo[5] = combo[operand as usize] % 8,
            3 => {
                if combo[4] != 0 { pointer = operand as usize; continue; };
                
            }
            4 => combo[5] = combo[5] ^ combo[6],
            5 => outputs.push(combo[operand as usize] % 8),
            6 => combo[5] = combo[4] >> combo[operand as usize],
            7 => combo[6] = combo[4] >> combo[operand as usize],
            _ => (),
        }

        pointer += 2;
    }


    outputs
}

// Find valid B, C pairs that generate specific values
fn get_bc_combos() -> HashMap<u64, Vec<(u64, u64)>> {
    let mut bc_map: HashMap<u64, Vec<(u64, u64)>> = HashMap::new();

    // This is one cycle of our original program
    // without the initialization of B and C through A
    let program: Vec<u64> = vec![1, 2, 1, 3, 4, 4, 5, 5];


    for b in 0..8u64 {
        for c in 0..8u64 {
            let shift = b ^ 2;

            if shift < 3 {
                let b_bits = format!("{:03b}", b);
                let c_bits = format!("{:03b}", c);

                if c_bits[shift as usize..] != b_bits[..3-shift as usize] {
                    continue;
                }
            }

            let out = execute_program(0, b, c, &program)[0];

            match bc_map.get_mut(&out) {
                Some(bc_pairs) => {bc_pairs.push((b, c));},
                None => {bc_map.insert(out, vec![(b,c)]);},
            }
        }
    }

    bc_map
}

fn determine_bits(program: &Vec<u64>, bc_map: &HashMap<u64, Vec<(u64, u64)>>) -> u64 {
    
    let total_bits:usize = 3*program.len();

    let mut nodes: Vec<(usize, Vec<u64>)> = vec![(0, vec![2;total_bits])];

    let mut candidates: Vec<Vec<u64>> = Vec::new();

    while nodes.len() > 0 {
        let (pointer, guess) = nodes.pop().unwrap();

        if pointer == program.len() {
            candidates.push(guess);
            continue;
        }

        let target = program[pointer];

        for (b,c) in &bc_map[&target] {
            let mut guess2 = guess.clone();

            let b_shift: i32 = (b ^ 2) as i32;

            let b_ind: usize = total_bits - 3*pointer - 3;
            let c_ind: i32 = b_ind as i32 - b_shift;

            let mut valid: bool = true;

            for i in 0..3_usize {
                let b_bit = (b >> (2 - i) as u64) % 2;

                if guess2[b_ind + i] == 2 {
                    guess2[b_ind + i] = b_bit;
                } else if guess2[b_ind + i] != b_bit {
                    valid = false;
                    break;
                }
            }

            if !valid { continue; }

            for j in 0..3_i32 {
                let c_bit = (c >> (2 - j) as u64) % 2;

                if c_ind + j < 0 {
                    if c_bit != 0 {
                        valid = false;
                        break
                    }
                } else if guess2[(c_ind + j) as usize] == 2 {
                    guess2[(c_ind + j) as usize] = c_bit;
                } else if guess2[(c_ind + j) as usize] != c_bit {
                    valid = false;
                    break;
                }
            }

            if !valid { continue; }

            nodes.push((pointer + 1, guess2));

        }
    }

    let mut num = candidates.iter().min().unwrap().clone();
    num.reverse();

    let num: u64 = num.iter().enumerate().map(|(i, b)| (2_u64).pow(i as u32)*b).sum();

    num
}