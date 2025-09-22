use crate::helper::read_data;
use std::collections::HashMap;

pub fn main() {
    let data: String = read_data("../Data/Day16.txt");

    let bits: Vec<bool> = hex_to_bin(&data);

    let (p1, _, p2) = read_packet(&bits, 0);

    println!("{p1}\n{p2}");
}

fn hex_to_bin(data: &String) -> Vec<bool> {
    let mut bits: Vec<bool> = Vec::new();

    let hex: HashMap<char, Vec<bool>> = HashMap::from([
        ('0', vec![false, false, false, false]),
        ('1', vec![false, false, false, true]),
        ('2', vec![false, false, true, false]),
        ('3', vec![false, false, true, true]),
        ('4', vec![false, true, false, false]),
        ('5', vec![false, true, false, true]),
        ('6', vec![false, true, true, false]),
        ('7', vec![false, true, true, true]),
        ('8', vec![true, false, false, false]),
        ('9', vec![true, false, false, true]),
        ('A', vec![true, false, true, false]),
        ('B', vec![true, false, true, true]),
        ('C', vec![true, true, false, false]),
        ('D', vec![true, true, false, true]),
        ('E', vec![true, true, true, false]),
        ('F', vec![true, true, true, true]),
    ]);

    for c in data.chars() {
        for &bit in &hex[&c] {
            bits.push(bit);
        }
    }

    bits
}

fn read_packet(bits: &Vec<bool>, start_pointer: usize) -> (usize, usize, usize) {
    let mut pointer: usize = start_pointer;

    let mut total_version: usize = bin_to_dec(&bits[pointer..pointer + 3]);
    let id: usize = bin_to_dec(&bits[pointer + 3..pointer + 6]);

    let mut sub_values: Vec<usize> = Vec::new();

    pointer += 6;

    if id == 4 {
        let mut bin_num: Vec<bool> = Vec::new();

        while bits[pointer] {
            for &bit in &bits[pointer + 1..pointer + 5] {
                bin_num.push(bit);
            }
            pointer += 5;
        }

        for &bit in &bits[pointer + 1..pointer + 5] {
            bin_num.push(bit);
        }
        pointer += 5;

        sub_values.push(bin_to_dec(&bin_num));
    } else if !bits[pointer] {
        pointer += 1;

        let total_sub_bits: usize = bin_to_dec(&bits[pointer..pointer + 15]);
        pointer += 15;

        let mut sub_bits: usize = 0;

        while sub_bits < total_sub_bits {
            let res: (usize, usize, usize) = read_packet(bits, pointer);

            total_version += res.0;

            pointer += res.1;
            sub_bits += res.1;

            sub_values.push(res.2);
        }
    } else if bits[pointer] {
        pointer += 1;

        let total_sub_packets: usize = bin_to_dec(&bits[pointer..pointer + 11]);
        pointer += 11;

        for _ in 0..total_sub_packets {
            let res: (usize, usize, usize) = read_packet(bits, pointer);

            total_version += res.0;
            pointer += res.1;
            sub_values.push(res.2);
        }
    }

    let value: usize = match id {
        0 => sub_values.iter().sum(),
        1 => sub_values.iter().product(),
        2 => *sub_values.iter().min().unwrap(),
        3 => *sub_values.iter().max().unwrap(),
        4 => sub_values[0],
        5 => {
            if sub_values[0] > sub_values[1] {
                1
            } else {
                0
            }
        }
        6 => {
            if sub_values[0] < sub_values[1] {
                1
            } else {
                0
            }
        }
        7 => {
            if sub_values[0] == sub_values[1] {
                1
            } else {
                0
            }
        }
        _ => 0,
    };

    return (total_version, pointer - start_pointer, value);
}

fn bin_to_dec(bits: &[bool]) -> usize {
    let mut num: usize = 0;

    for &b in bits {
        num *= 2;
        if b {
            num += 1;
        }
    }

    num
}
