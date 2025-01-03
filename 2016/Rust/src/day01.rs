use crate::helper::read_data;
use std::collections::HashSet;
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day01.txt");

    let directions: Vec<(char, i32)> = get_directions(&data);

    let p1: i32 = follow_directions(&directions, false);
    let p2: i32 = follow_directions(&directions, true);

    println!("{p1}\n{p2}");
}

fn get_directions(data: &String) -> Vec<(char, i32)> {
    let mut directions: Vec<(char, i32)> = Vec::new();

    for direction in data.split(", ") {
        let turn = direction.chars().next().unwrap();
        let steps: i32 = direction[1..].parse().unwrap();

        directions.push((turn, steps));
    }

    directions
}

fn follow_directions(directions: &Vec<(char, i32)>, part2: bool) -> i32 {
    let headings: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    let mut heading: usize = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;

    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert((0, 0));

    for (turn, steps) in directions {
        if *turn == 'R' {
            heading += 1;
            heading %= 4;
        } else {
            heading += 3;
            heading %= 4;
        }

        let (di, dj): (i32, i32) = headings[heading];

        if part2 {
            for _ in 0..*steps {
                i += di;
                j += dj;

                if visited.contains(&(i, j)) {
                    return i.abs() + j.abs();
                } else {
                    visited.insert((i, j));
                }
            }
        } else {
            i += di * steps;
            j += dj * steps;
        }
    }

    i.abs() + j.abs()
}
