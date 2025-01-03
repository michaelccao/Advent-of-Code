use crate::helper::read_data;
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day02.txt");

    let pad1: [&str; 3] = ["123", "456", "789"];
    let pad1: Vec<Vec<char>> = pad1.map(|nums| nums.chars().collect::<Vec<char>>()).iter().cloned().collect::<Vec<_>>();

    let pad2: [&str; 5] = ["zz1zz", "z234z", "56789", "zABCz", "zzDzz"];
    let pad2: Vec<Vec<char>>  = pad2.map(|nums| nums.chars().collect::<Vec<char>>()).iter().cloned().collect::<Vec<_>>();


    let p1: String = follow_instructions2(&data, &pad1);

    let p2: String = follow_instructions2(&data, &pad2);

    println!("{p1}\n{p2}");
}

fn follow_instructions2(data: &String, pad: &Vec<Vec<char>>) -> String {
    let mut code: String = String::new();

    let mut i: usize = 1;
    let mut j: usize = 1;

    for line in data.lines() {
        let line = line.trim();

        for c in line.chars() {
            match c {
                'U' => {
                    if i > 0 && pad[i-1][j] != 'z' {
                        i -= 1;
                    }
                }
                'R' => {
                    if j < pad[0].len()-1 && pad[i][j+1] != 'z' {
                        j += 1;
                    }
                }
                'D' => {
                    if i < pad.len()-1 && pad[i+1][j] != 'z' {
                        i += 1;
                    }
                }
                'L' => {
                    if j > 0 && pad[i][j-1] != 'z' {
                        j -= 1;
                    }
                }
                _ => {}
            }
        }

        code.push(pad[i][j]);

    }

    code
}
