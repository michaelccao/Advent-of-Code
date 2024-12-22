use crate::helper::read_data;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day21.txt");

    let codes: Vec<&str> = data.lines().collect::<Vec<_>>();

    let numpad: HashMap<char, (i32, i32)> = HashMap::from([
        ('7', (0, 0)),
        ('8', (0, 1)),
        ('9', (0, 2)),
        ('4', (1, 0)),
        ('5', (1, 1)),
        ('6', (1, 2)),
        ('1', (2, 0)),
        ('2', (2, 1)),
        ('3', (2, 2)),
        ('0', (3, 1)),
        ('A', (3, 2)),
    ]);

    let dpad: HashMap<char, (i32, i32)> = HashMap::from([
        ('^', (0, 1)),
        ('A', (0, 2)),
        ('<', (1, 0)),
        ('v', (1, 1)),
        ('>', (1, 2)),
    ]);

    let num_map: HashMap<(char, char), Vec<String>> = get_map(&numpad, (3, 0));
    let d_map: HashMap<(char, char), Vec<String>> = get_map(&dpad, (0, 0));

    encode("029A", &num_map);
}

fn get_map(
    pad: &HashMap<char, (i32, i32)>,
    forbidden: (i32, i32),
) -> HashMap<(char, char), Vec<String>> {
    let mut move_map: HashMap<(char, char), Vec<String>> = HashMap::new();

    for (b, (i, j)) in pad {
        for (b2, (i2, j2)) in pad {
            let mut moves: Vec<String> = Vec::new();

            let di: i32 = i2 - i;
            let dj: i32 = j2 - j;

            let vertical: String = vec![if di >= 0 { 'v' } else { '^' }; di.abs() as usize]
                .iter()
                .collect();

            let horizontal: String = vec![if dj >= 0 { '>' } else { '<' }; dj.abs() as usize]
                .iter()
                .collect();

            if di == 0 {
                moves.push(horizontal)
            } else if dj == 0 {
                moves.push(vertical)
            } else {
                let hv: String = format!("{}{}", &horizontal, &vertical);

                let vh: String = format!("{}{}", &vertical, &horizontal);

                if (*i, *j2) != forbidden {
                    moves.push(hv);
                }

                if (*i2, *j) != forbidden {
                    moves.push(vh);
                }
            }

            move_map.insert((*b, *b2), moves);
        }
    }

    move_map
}

fn encode(code: &str, move_map: &HashMap<(char, char), Vec<String>>) {
    let first_a: usize = code.find('A').unwrap();

    if first_a == code.len() - 1 {
        let mut codes: Vec<Vec<String>> = Vec::new();
        let mut prev_char: char = 'A';

        for c in code.chars() {
            let moves = move_map[&(prev_char, c)]
                .iter()
                .map(|m| format!("{m}A"))
                .collect::<Vec<String>>();
            codes.push(moves);

            prev_char = c;
        }

        println!("{codes:?}");

        string_product(codes);
    } else {
    }
}

fn string_product(codes: Vec<Vec<String>>) {
    let mut counter: Vec<usize> = vec![0; codes.len()];
    let total = codes.iter().map(|s| s.len()).collect::<Vec<_>>();

    
}