use crate::helper::read_data;
use std::collections::HashSet;
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day14.txt");

    let (grid, p1): (Vec<Vec<bool>>, u32) = get_grid(&data);

    let p2: u32 = get_regions(&grid);

    println!("{p1}\n{p2}");
}

fn get_grid(data: &String) -> (Vec<Vec<bool>>, u32) {
    let mut grid: Vec<Vec<bool>> = Vec::new();
    let mut used: u32 = 0;

    for row_num in 0..128 {
        let key: String = format!("{data}-{row_num}");

        let (row, used2): (Vec<bool>, u32) = knot_hash(&key);

        used += used2;

        grid.push(row);
    }

    (grid, used)
}

fn get_regions(grid: &Vec<Vec<bool>>) -> u32 {
    let rows: usize = grid.len();
    let cols: usize = grid[0].len();

    let mut regions: u32 = 0;
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] && !visited.contains(&(i, j)) {
                regions += 1;
                explore_region(grid, i, j, &mut visited);
            }
        }
    }

    regions
}

fn explore_region(
    grid: &Vec<Vec<bool>>,
    i: usize,
    j: usize,
    visited: &mut HashSet<(usize, usize)>,
) {
    let rows: usize = grid.len();
    let cols: usize = grid[0].len();

    let mut nodes: Vec<(usize, usize)> = vec![(i, j)];

    while nodes.len() > 0 {
        let (i, j): (usize, usize) = nodes.pop().unwrap();

        let mut neighbors: Vec<(usize, usize)> = Vec::new();

        if i > 0 {
            neighbors.push((i - 1, j));
        }
        if j > 0 {
            neighbors.push((i, j - 1));
        }
        if i < rows - 1 {
            neighbors.push((i + 1, j));
        }
        if j < cols - 1 {
            neighbors.push((i, j + 1));
        }

        for (i2, j2) in neighbors {
            if grid[i2][j2] && !visited.contains(&(i2, j2)) {
                visited.insert((i2, j2));
                nodes.push((i2, j2));
            }
        }
    }
}

fn knot_hash(key: &String) -> (Vec<bool>, u32) {
    let mut lengths: Vec<usize> = key.chars().map(|c| c as usize).collect();

    lengths.append(&mut vec![17, 31, 73, 47, 23]);

    let mut circle: Vec<u32> = (0..256).collect();

    let mut pointer: usize = 0;
    let circle_size: usize = circle.len();
    let mut skip: usize = 0;

    for _ in 0..64 {
        for &l in &lengths {
            for i in 0..l / 2 {
                let a: usize = (pointer + i) % circle_size;
                let b: usize = (pointer + l - i - 1) % circle_size;

                circle.swap(a, b);
            }

            pointer += l + skip;
            pointer %= circle_size;

            skip += 1;
        }
    }

    let mut knot_hash: Vec<bool> = Vec::new();

    let mut used: u32 = 0;

    for i in 0..circle_size / 16 {
        let mut dense_hash: u32 = 0;
        for j in 0..16 {
            dense_hash ^= circle[16 * i + j];
        }

        for bit in 0..8_u32 {
            let occupation: bool = (dense_hash >> (8 - bit - 1)) & 1 == 1;

            if occupation {
                used += 1
            }

            knot_hash.push((dense_hash >> (8 - bit - 1)) & 1 == 1);
        }
    }

    (knot_hash, used)
}
