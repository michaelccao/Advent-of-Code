use crate::helper::read_data;
use itertools::Itertools;
use std::collections::{HashMap, VecDeque};
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day24.txt");

    let (grid, locations) = get_grid(&data);

    let matrix: Vec<Vec<u32>> = calculate_matrix(&locations, &grid);

    let p1: u32 = traveling_salesman(&matrix, false);
    let p2: u32 = traveling_salesman(&matrix, true);

    println!("{p1}\n{p2}");
}

fn get_grid(data: &String) -> (Vec<Vec<bool>>, HashMap<usize, (usize, usize)>) {
    let mut grid: Vec<Vec<bool>> = Vec::new();

    let mut locations: HashMap<usize, (usize, usize)> = HashMap::new();

    for line in data.lines() {
        let mut row: Vec<bool> = Vec::new();
        for c in line.trim().chars() {
            if c == '#' {
                row.push(false);
            } else if c == '.' {
                row.push(true);
            } else {
                let poi: usize = c.to_digit(10).unwrap() as usize;
                let i: usize = grid.len();
                let j: usize = row.len();

                locations.insert(poi, (i, j));

                row.push(true);
            }
        }
        grid.push(row);
    }

    (grid, locations)
}

fn fastest_route(start: (usize, usize), end: (usize, usize), grid: &Vec<Vec<bool>>) -> u32 {
    let mut shortest: u32 = (grid.len() * grid[0].len()) as u32;

    let mut visited: HashMap<(usize, usize), u32> = HashMap::new();

    let mut nodes: VecDeque<((usize, usize), u32)> = VecDeque::from([(start, 0)]);

    while nodes.len() > 0 {
        let ((i, j), steps) = nodes.pop_back().unwrap();

        if (i, j) == end {
            if steps < shortest {
                shortest = steps;
            }
            continue;
        }

        if steps >= shortest {
            continue;
        }

        for (i2, j2) in [(i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1)] {
            if grid[i2][j2] {
                if let Some(steps2) = visited.get_mut(&(i2, j2)) {
                    if steps + 1 < *steps2 {
                        *steps2 = steps + 1;
                        nodes.push_back(((i2, j2), steps + 1));
                    }
                } else {
                    visited.insert((i2, j2), steps + 1);
                    nodes.push_back(((i2, j2), steps + 1));
                }
            }
        }
    }

    shortest
}

fn calculate_matrix(
    locations: &HashMap<usize, (usize, usize)>,
    grid: &Vec<Vec<bool>>,
) -> Vec<Vec<u32>> {
    let num_locations = locations.len();

    let mut matrix: Vec<Vec<u32>> = vec![vec![0; num_locations]; num_locations];

    for i in 0..num_locations {
        for j in i + 1..num_locations {
            let dist: u32 = fastest_route(locations[&i], locations[&j], grid);
            matrix[i][j] = dist;
            matrix[j][i] = dist;
        }
    }

    matrix
}

fn traveling_salesman(matrix: &Vec<Vec<u32>>, part2: bool) -> u32 {
    let routes = (1..matrix.len()).permutations(matrix.len() - 1);

    let mut shortest: Option<u32> = None;

    for route in routes {
        let mut route_length: u32 = 0;
        let mut prev: usize = 0;

        for stop in route {
            route_length += matrix[prev][stop];
            prev = stop;
        }

        if part2 {
            route_length += matrix[prev][0];
        }

        if shortest.is_none() {
            shortest = Some(route_length);
        } else if route_length < shortest.unwrap() {
            shortest = Some(route_length);
        }
    }

    shortest.unwrap()
}
