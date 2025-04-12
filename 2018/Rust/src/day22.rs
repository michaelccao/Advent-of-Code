use crate::helper::read_data;
use std::collections::{HashMap, VecDeque};
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day22.txt");

    let (depth, target) = get_depth_and_target(&data);

    let p1: usize = calculate_risk(depth, &target);

    println!("{p1}");

    let p2: usize = rescue(depth, &target);

    println!("{p2}");
}

fn get_depth_and_target(data: &String) -> (usize, Vec<usize>) {
    let mut lines = data.lines();

    let depth: usize = lines
        .next()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse()
        .unwrap();

    let target: Vec<usize> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .split(",")
        .map(|coord| coord.parse::<usize>().unwrap())
        .collect();

    (depth, target)
}

fn calculate_risk(depth: usize, target: &Vec<usize>) -> usize {
    let (tx, ty) = (target[0], target[1]);

    let mut erosion: Vec<Vec<usize>> = vec![vec![0; tx + 1]; ty + 1];
    let mut total_risk: usize = 0;

    for y in 0..ty + 1 {
        for x in 0..tx + 1 {
            if y == 0 {
                erosion[y][x] = (x * 16807 + depth) % 20183
            } else if x == 0 {
                erosion[y][x] = (y * 48271 + depth) % 20183
            } else if y == ty && x == tx {
                erosion[y][x] = depth % 20183;
            } else {
                erosion[y][x] = erosion[y - 1][x] * erosion[y][x - 1];
                erosion[y][x] += depth;
                erosion[y][x] %= 20183
            }

            total_risk += erosion[y][x] % 3
        }
    }

    total_risk
}

fn rescue(depth: usize, target: &Vec<usize>) -> usize {
    let (tx, ty) = (target[0], target[1]);

    let mut erosion: Vec<Vec<usize>> = vec![vec![0; 3 * tx + 1]; 3 * ty + 1];
    let mut risk: Vec<Vec<usize>> = erosion.clone();

    for y in 0..3 * ty + 1 {
        for x in 0..3 * tx + 1 {
            if y == 0 {
                erosion[y][x] = (x * 16807 + depth) % 20183
            } else if x == 0 {
                erosion[y][x] = (y * 48271 + depth) % 20183
            } else if y == ty && x == tx {
                erosion[y][x] = depth % 20183;
            } else {
                erosion[y][x] = erosion[y - 1][x] * erosion[y][x - 1];
                erosion[y][x] += depth;
                erosion[y][x] %= 20183
            }

            risk[y][x] = erosion[y][x] % 3;
        }
    }

    let mut nodes: VecDeque<(usize, usize, usize, usize)> = VecDeque::from([(0, 0, 1, 0)]);
    let mut visited: HashMap<(usize, usize, usize), usize> = HashMap::new();
    visited.insert((0, 0, 1), 0);

    let mut shortest: usize = (tx + ty) * 8 + 7; // Worst case scenario requires an equipment switch per move

    while nodes.len() > 0 {
        let (y, x, gear, minutes) = nodes.pop_front().unwrap();

        if y == ty && x == tx {
            if gear == 1 && minutes < shortest {
                shortest = minutes;
            } else if gear != 1 && minutes + 7 < shortest {
                shortest = minutes + 7;
            }
            continue;
        }

        if y.abs_diff(ty) + x.abs_diff(tx) + minutes >= shortest {
            continue;
        }

        let mut neighbors: Vec<(usize, usize)> = Vec::new();

        if y > 0 {
            neighbors.push((y - 1, x));
        }

        if y < risk.len() - 1 {
            neighbors.push((y + 1, x));
        }

        if x > 0 {
            neighbors.push((y, x - 1));
        }

        if x < risk[0].len() - 1 {
            neighbors.push((y, x + 1));
        }

        for (y2, x2) in neighbors {
            if risk[y2][x2] != gear
                && (!visited.contains_key(&(y2, x2, gear))
                    || minutes + 1 < visited[&(y2, x2, gear)])
            {
                visited.insert((y2, x2, gear), minutes + 1);
                nodes.push_back((y2, x2, gear, minutes + 1));
            } else if risk[y2][x2] == gear {
                let gear2: usize = 3 - risk[y][x] - risk[y2][x2];

                if !visited.contains_key(&(y2, x2, gear2))
                    || minutes + 8 < visited[&(y2, x2, gear2)]
                {
                    visited.insert((y2, x2, gear2), minutes + 8);
                    nodes.push_back((y2, x2, gear2, minutes + 8));
                }
            }
        }
    }

    shortest
}
