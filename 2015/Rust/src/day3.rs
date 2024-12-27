use crate::helper::read_data;
use std::collections::HashMap;
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day3.txt");

    let p1: usize = deliver_presents(&data);
    let p2: usize = deliver_presents_w_robo(&data);

    println!("{p1}\n{p2}");
}

fn deliver_presents(data: &String) -> usize {
    let mut houses: HashMap<(i32, i32), u32> = HashMap::new();
    houses.insert((0, 0), 1);

    let (mut i, mut j): (i32, i32) = (0, 0);

    let moves: HashMap<char, (i32, i32)> =
        HashMap::from([('>', (0, 1)), ('v', (1, 0)), ('<', (0, -1)), ('^', (-1, 0))]);

    for mv in data.chars() {
        let (di, dj) = moves[&mv];

        i += di;
        j += dj;

        if houses.contains_key(&(i, j)) {
            *houses.get_mut(&(i, j)).unwrap() += 1;
        } else {
            houses.insert((i, j), 1);
        }
    }

    houses.len()
}

fn deliver_presents_w_robo(data: &String) -> usize {
    let mut houses: HashMap<(i32, i32), u32> = HashMap::new();
    houses.insert((0, 0), 2);

    let (mut i, mut j): (i32, i32) = (0, 0);
    let (mut i2, mut j2): (i32, i32) = (0, 0);

    let moves: HashMap<char, (i32, i32)> =
        HashMap::from([('>', (0, 1)), ('v', (1, 0)), ('<', (0, -1)), ('^', (-1, 0))]);

    for (k, mv) in data.chars().enumerate() {
        let (di, dj) = moves[&mv];

        if k % 2 == 0 {
            i += di;
            j += dj;

            if houses.contains_key(&(i, j)) {
                *houses.get_mut(&(i, j)).unwrap() += 1;
            } else {
                houses.insert((i, j), 1);
            }
        } else {
            i2 += di;
            j2 += dj;

            if houses.contains_key(&(i2, j2)) {
                *houses.get_mut(&(i2, j2)).unwrap() += 1;
            } else {
                houses.insert((i2, j2), 1);
            }
        }
    }

    houses.len()
}
