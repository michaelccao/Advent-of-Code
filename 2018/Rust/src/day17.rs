use crate::helper::read_data;
use std::collections::HashSet;
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day17.txt");

    let clay: HashSet<(i32, i32)> = get_clay_grid(&data);

    let (p1, p2): (usize, usize) = simulate_water(&clay);

    println!("{p1}\n{p2}");
}

fn get_clay_grid(data: &String) -> HashSet<(i32, i32)> {
    let mut clay: HashSet<(i32, i32)> = HashSet::new();

    for line in data.lines() {
        let mut line = line.trim().split(", ");

        let mut fixed = line.next().unwrap().split("=");
        let fixed_axis: &str = fixed.next().unwrap();
        let fixed_coord: i32 = fixed.next().unwrap().parse().unwrap();

        let mut ranged = line.next().unwrap().split("=");
        ranged.next();

        let mut ranged_values = ranged.next().unwrap().split("..");
        let range_start: i32 = ranged_values.next().unwrap().parse().unwrap();
        let range_end: i32 = ranged_values.next().unwrap().parse().unwrap();

        for i in range_start..range_end + 1 {
            if fixed_axis == "x" {
                clay.insert((i, fixed_coord));
            } else if fixed_axis == "y" {
                clay.insert((fixed_coord, i));
            }
        }
    }

    clay
}

fn simulate_water(clay: &HashSet<(i32, i32)>) -> (usize, usize) {
    let mut forbidden: HashSet<(i32, i32)> = clay.clone();
    let min_y: i32 = forbidden.iter().map(|&(y, _)| y).min().unwrap();
    let max_y: i32 = forbidden.iter().map(|&(y, _)| y).max().unwrap();

    let mut nodes: Vec<(i32, i32)> = vec![(0, 500)];

    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert((0, 500));

    while nodes.len() > 0 {
        let (i, j) = nodes.pop().unwrap();

        if i > max_y {
            continue;
        }

        let down: (i32, i32) = (i + 1, j);

        if !forbidden.contains(&down) {
            visited.insert(down);
            nodes.push(down);
        } else {
            let (left_next, left_visited) = horizontal_flow(i, j, &forbidden, -1);
            let (right_next, right_visited) = horizontal_flow(i, j, &forbidden, 1);

            visited = visited.union(&left_visited).cloned().collect();
            visited = visited.union(&right_visited).cloned().collect();

            if left_next.is_some() {
                let left_next = left_next.unwrap();
                if !visited.contains(&left_next) {
                    visited.insert(left_next);
                    nodes.push(left_next);
                }
            }

            if right_next.is_some() {
                let right_next = right_next.unwrap();

                if !visited.contains(&right_next) {
                    visited.insert(right_next);
                    nodes.push(right_next);
                }
            }

            if left_next.is_none() && right_next.is_none() {
                forbidden = forbidden.union(&left_visited).cloned().collect();
                forbidden = forbidden.union(&right_visited).cloned().collect();
                nodes.push((i - 1, j));
            }
        }
    }

    (
        visited
            .iter()
            .filter(|&(y, _)| *y >= min_y && *y <= max_y)
            .count(),
        forbidden.difference(clay).count(),
    )
}

fn horizontal_flow(
    i: i32,
    j: i32,
    forbidden: &HashSet<(i32, i32)>,
    dj: i32,
) -> (Option<(i32, i32)>, HashSet<(i32, i32)>) {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert((i, j));

    let mut nodes: Vec<(i32, i32)> = vec![(i, j)];

    while nodes.len() > 0 {
        let (i, j) = nodes.pop().unwrap();

        if !forbidden.contains(&(i + 1, j)) {
            return (Some((i + 1, j)), visited);
        }

        if !forbidden.contains(&(i, j + dj)) {
            visited.insert((i, j + dj));
            nodes.push((i, j + dj));
        }
    }

    (None, visited)
}
