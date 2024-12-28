use crate::helper::read_data;
use std::collections::HashSet;
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day11.txt");

    let pass: Vec<u8> = data.chars().map(|c| c as u8).collect();
    let p1: Vec<u8> = next_valid_pass(pass.clone());
    let p2: Vec<u8> = next_valid_pass(p1.clone());

    println!("{}\n{}", print_pass(&p1), print_pass(&p2));
}

fn next_pass(mut pass: Vec<u8>) -> Vec<u8> {
    const I: u8 = 'i' as u8;
    const O: u8 = 'o' as u8;
    const L: u8 = 'l' as u8;
    const Z: u8 = 'z' as u8;
    const A: u8 = 'a' as u8;

    let forbidden: HashSet<u8> = HashSet::from([I, O, L]);

    let mut pointer: usize = pass.len() - 1;

    loop {
        if forbidden.contains(&(pass[pointer] + 1)) {
            pass[pointer] += 2;

            for i in pointer + 1..pass.len() {
                pass[i] = A;
            }

            break;
        } else if pass[pointer] == Z {
            pass[pointer] = A;
            pointer -= 1;
        } else {
            pass[pointer] += 1;
            break;
        }
    }

    pass
}

fn print_pass(pass: &Vec<u8>) -> String {
    pass.iter().map(|c| *c as char).collect::<String>()
}

fn is_valid(pass: &Vec<u8>) -> bool {
    let mut pairs: u8 = 0;
    let mut consecutive: bool = false;

    let mut pointer: usize = 0;
    let mut pointer2: usize = 0;

    while pointer < pass.len() - 1 || pointer2 < pass.len() - 2 {
        if pointer2 < pass.len() - 2
            && pass[pointer2] == pass[pointer2 + 1] - 1
            && pass[pointer2] == pass[pointer2 + 2] - 2
        {
            consecutive = true;
            pointer2 = pass.len();
        } else {
            pointer2 += 1;
        }

        if pointer < pass.len() - 1 && pass[pointer] == pass[pointer + 1] {
            pairs += 1;
            pointer += 2;
        } else {
            pointer += 1;
        }

        if pairs == 2 && consecutive {
            return true;
        }
    }
    false
}

fn next_valid_pass(mut pass: Vec<u8>) -> Vec<u8> {
    pass = next_pass(pass);
    while !is_valid(&pass) {
        pass = next_pass(pass);
    }

    pass
}
