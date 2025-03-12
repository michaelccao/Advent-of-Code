use crate::helper::read_data;
use std::cmp::max;
use std::collections::HashMap;
use std::vec::Vec;

pub fn main() {
    let serial_num: i32 = read_data("../Data/Day11.txt").parse().unwrap();
    let cells = calculate_cells(serial_num);

    let p1: (usize, usize) = best_3cluster(&cells);
    let p2: (usize, usize, usize) = best_cluster(&cells);

    println!("{p1:?}\n{p2:?}");
}

fn calculate_power(x: usize, y: usize, serial_num: i32) -> i32 {
    let rack_id: i32 = x as i32 + 10;
    let mut power = rack_id * y as i32;
    power += serial_num;
    power *= rack_id;
    power /= 100;
    power %= 10;
    power -= 5;

    power
}

fn calculate_cells(serial_num: i32) -> Vec<Vec<i32>> {
    let mut cells: Vec<Vec<i32>> = vec![vec![0; 300]; 300];

    for i in 0..cells.len() {
        let y: usize = i + 1;
        for j in 0..cells[i].len() {
            let x: usize = j + 1;
            cells[i][j] = calculate_power(x, y, serial_num);
        }
    }

    cells
}

fn best_3cluster(cells: &Vec<Vec<i32>>) -> (usize, usize) {
    let mut best: (usize, usize) = (0, 0);
    let mut best_power: i32 = 0;

    for i in 0..cells.len() - 2 {
        for j in 0..cells.len() - 2 {
            let mut power: i32 = cells[i][j] + cells[i + 1][j] + cells[i + 2][j];
            power += cells[i][j + 1] + cells[i + 1][j + 1] + cells[i + 2][j + 1];
            power += cells[i][j + 2] + cells[i + 1][j + 2] + cells[i + 2][j + 2];

            if power > best_power {
                best_power = power;
                best = (i, j);
            }
        }
    }

    (best.1 + 1, best.0 + 1)
}

fn calculate_cluster(
    cells: &Vec<Vec<i32>>,
    i: usize,
    j: usize,
    s: usize,
    mut cache: HashMap<(usize, usize, usize), i32>,
) -> (i32, HashMap<(usize, usize, usize), i32>) {
    if s == 1 {
        return (cells[i][j], cache);
    } else if cache.contains_key(&(i, j, s)) {
        return (cache[&(i, j, s)], cache);
    } else if s == 2 {
        let total = cells[i][j] + cells[i + 1][j] + cells[i + 1][j + 1] + cells[i][j + 1];
        cache.insert((i, j, s), total);

        return (total, cache);
    } else {
        let mut total: i32 = cells[i][j];

        for d in 1..s {
            total += cells[i + d][j];
            total += cells[i][j + d];
        }

        let (subcluster, cache) = calculate_cluster(cells, i + 1, j + 1, s - 1, cache);

        total += subcluster;

        return (total, cache);
    }
}

fn best_cluster(cells: &Vec<Vec<i32>>) -> (usize, usize, usize) {
    let mut best: (usize, usize, usize) = (0, 0, 1);
    let mut best_power: i32 = cells[0][0];
    let mut cache: HashMap<(usize, usize, usize), i32> = HashMap::new();

    for i in 0..cells.len() {
        for j in 0..cells.len() {
            let max_s = cells.len() - max(i, j);
            for s in 1..max_s + 1 {
                let (power, cache2) = calculate_cluster(cells, i, j, s, cache);
                cache = cache2;
                if power > best_power {
                    best_power = power;
                    best = (i, j, s);
                }
            }
        }
    }

    (best.1 + 1, best.0 + 1, best.2)
}
