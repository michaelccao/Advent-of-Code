use crate::helper::read_data;
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day09.txt");

    let p1: usize = parse(&data, false);
    let p2: usize = parse(&data, true);
    println!("{p1}\n{p2}");
}

fn parse(data: &String, part2: bool) -> usize {
    let mut pointer: usize = 0;

    let mut num_chars: usize = 0;

    let s: Vec<char> = data.chars().collect();

    let mut buffer: String = String::new();
    let mut marker: bool = false;

    while pointer < s.len() {
        let c = s[pointer];

        if c == ' ' {
            pointer += 1;
        } else if c == '(' {
            marker = true;
            pointer += 1;
        } else if marker && c == ')' {
            let mut axb = buffer.split('x');
            let a: usize = axb.next().unwrap().parse().unwrap();
            let b: usize = axb.next().unwrap().parse().unwrap();

            if part2 {
                let sub: String = s[pointer + 1..pointer + 1 + a].iter().collect();
                num_chars += parse(&sub, true) * b;
            } else {
                num_chars += a * b;
            }

            pointer += a + 1;

            marker = false;
            buffer = String::new();
        } else if marker {
            buffer.push(c);
            pointer += 1;
        } else {
            num_chars += 1;
            pointer += 1;
        }
    }

    num_chars
}
