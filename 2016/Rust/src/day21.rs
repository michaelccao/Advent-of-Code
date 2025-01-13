use crate::helper::read_data;
use std::vec::Vec;
use itertools::Itertools;

pub fn main() {
    let data: String = read_data("../Data/Day21.txt");
    let instructions: Vec<&str> = data.lines().map(|line| line.trim()).collect();

    let p1 = scramble_password("abcdefgh", &instructions);

    println!("{p1}");

    // Too lazy to do this "properly", just brute force it LMAO
    for password in "abcdefgh".chars().permutations(8) {
        let password: String = password.iter().collect::<String>();
        if scramble_password( &password[..], &instructions) == "fbgdceah".to_string() {
            println!("{password}");
        }
    }

}

fn scramble_password(start: &str, instructions: &Vec<&str>) -> String {

    let mut s: Vec<char> = start.chars().collect();

    for instruction in instructions {
        let instruction: Vec<&str> = instruction.split_whitespace().collect();

        match instruction[0] {
            "rotate" => {rotate(&mut s, instruction)},
            "swap" => {swap(&mut s, instruction)},
            "reverse" => {
                let x: usize = instruction[2].parse().unwrap();
                let y: usize = instruction[4].parse().unwrap();

                for i in 0..(y-x+1)/2 {
                    s.swap(x+i, y-i);
                }
            },
            "move" => {
                let x: usize = instruction[2].parse().unwrap();
                let y: usize = instruction[5].parse().unwrap();
                let removed = s.remove(x);
                s.insert(y, removed);
            },
            _ => {}
        }
    }

    s.iter().collect::<String>()

}

fn rotate(s: &mut Vec<char>, instruction: Vec<&str>) {
    let shift: usize = match instruction[1] {
        "right" => instruction[2].parse().unwrap(),
        "left" => s.len() - instruction[2].parse::<usize>().unwrap(),
        "based" => { let ind = s.iter().position(|&x| x == instruction[6].chars().next().unwrap()).unwrap();
            (1 + ind + if ind >= 4 {1} else {0}) % s.len() }, 
        _ => 0
    };

    s.rotate_right(shift);
}

fn swap(s: &mut Vec<char>, instruction: Vec<&str>) {

    if instruction[1] == "letter" {
        let x = instruction[2].chars().next().unwrap();
        let y = instruction[5].chars().next().unwrap();

        let mut x_ind: usize = 0;
        let mut y_ind: usize = 0;

        for i in 0..s.len() {
            if s[i] == x {
                x_ind = i;
            }

            if s[i] == y {
                y_ind = i;
            }
        }

        s.swap(x_ind, y_ind);

    } else {
        let x_ind: usize = instruction[2].parse().unwrap();
        let y_ind: usize = instruction[5].parse().unwrap();

        s.swap(x_ind, y_ind);
    }

}