use crate::helper::read_data;
use std::collections::HashMap;

pub fn main() {
    let data: String = read_data("../Data/Day14.txt");

    let instructions: Vec<&str> = data.lines().collect();

    let p1: u64 = follow_instructions(&instructions);

    println!("{p1}");

    let p2: u64 = follow_instructions2(&instructions);

    println!("{p2}");
}

fn follow_instructions(instructions: &Vec<&str>) -> u64 {
    let mut cache: HashMap<u64, u64> = HashMap::new();
    let mut mask: Vec<char> = vec!['X'; 36];

    for instruction in instructions {
        if instruction.starts_with("mask") {
            let mask_str = instruction.split(" = ").last().unwrap();
            mask = mask_str.chars().collect();
        } else if instruction.starts_with("mem") {
            let mut instruction = instruction.split("] = ");
            let address: u64 = instruction.next().unwrap()[4..].parse().unwrap();
            let mut value: u64 = instruction.next().unwrap().parse().unwrap();

            for (i, &m) in mask.iter().enumerate() {
                if m == 'X' {
                    continue;
                }

                let bit = (value >> (35 - i)) % 2;

                if bit == 1 && m == '0' {
                    value -= 2u64.pow(35 - i as u32);
                } else if bit == 0 && m == '1' {
                    value += 2u64.pow(35 - i as u32);
                }
            }

            cache.insert(address, value);
        }
    }

    cache.values().sum::<u64>()
}

fn follow_instructions2(instructions: &Vec<&str>) -> u64 {
    let mut cache: HashMap<u64, u64> = HashMap::new();
    let mut mask: Vec<char> = vec!['X'; 36];

    for instruction in instructions {
        if instruction.starts_with("mask") {
            let mask_str: &str = instruction.split(" = ").last().unwrap();
            mask = mask_str.chars().collect();
        } else if instruction.starts_with("mem") {
            let mut instruction = instruction.split("] = ");
            let address: u64 = instruction.next().unwrap()[4..].parse().unwrap();
            let mut address_bits: Vec<char> = format!("{:036b}", address).chars().collect();
            let value: u64 = instruction.next().unwrap().parse().unwrap();

            for i in 0..mask.len() {
                if mask[i] == 'X' {
                    address_bits[i] = 'X';
                } else if mask[i] == '1' {
                    address_bits[i] = '1';
                }
            }

            for address in generate_addresses(&address_bits) {
                cache.insert(address, value);
            }
        }
    }

    cache.values().sum::<u64>()
}

fn generate_addresses(address_bits: &Vec<char>) -> Vec<u64> {
    let mut addresses: Vec<u64> = Vec::new();

    let mut nodes: Vec<(Vec<char>, usize)> = vec![(address_bits.clone(), 0)];

    while nodes.len() > 0 {
        let (bits, pointer) = nodes.pop().unwrap();

        let mut changed: bool = false;

        for i in pointer..bits.len() {
            if bits[i] == 'X' {
                let mut bit0: Vec<char> = bits.clone();
                bit0[i] = '0';
                nodes.push((bit0, i));

                let mut bit1: Vec<char> = bits.clone();
                bit1[i] = '1';
                nodes.push((bit1, i));

                changed = true;

                break;
            }
        }

        if !changed {
            let bit_str: String = bits.iter().collect();
            addresses.push(u64::from_str_radix(&bit_str, 2).unwrap())
        }
    }

    addresses
}
