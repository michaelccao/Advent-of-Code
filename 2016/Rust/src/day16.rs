use crate::helper::read_data;
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day16.txt");
    let start_state: Vec<bool> = data.chars().map(|c| c == '1').collect();

    let p1: String = bits_to_string(&checksum(&dragon_curve(&start_state, 272)));

    let p2: String = bits_to_string(&checksum(&dragon_curve(&start_state, 35651584)));

    println!("{p1}\n{p2}");
}

fn dragon_curve(start: &Vec<bool>, disk_size: usize) -> Vec<bool> {
    let mut a: Vec<bool> = start.clone();

    while a.len() < disk_size {
        let mut b: Vec<bool> = a.clone();

        b.reverse();
        b = b.iter().map(|x| !*x).collect();

        a.push(false);

        a.append(&mut b);
    }

    a[0..disk_size].to_vec()
}

fn checksum(a: &Vec<bool>) -> Vec<bool> {
    let mut b: Vec<bool> = Vec::new();

    for i in 0..a.len() / 2 {
        b.push(a[2 * i] == a[2 * i + 1]);
    }

    if b.len() % 2 == 1 {
        return b;
    } else {
        return checksum(&b);
    }
}

fn bits_to_string(a: &Vec<bool>) -> String {
    a.iter().map(|x| if *x { '1' } else { '0' }).collect()
}
