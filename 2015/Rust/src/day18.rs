use crate::helper::read_data;
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day18.txt");

    let grid: Vec<Vec<bool>> = data
        .lines()
        .map(|line| line.trim().chars().map(|c| c == '#').collect::<Vec<bool>>())
        .collect();

    let p1: u32 = next_n_grid(&grid, 100, false)
        .iter()
        .map(|row| row.iter().map(|light| *light as u32).sum::<u32>())
        .sum();
    let p2: u32 = next_n_grid(&grid, 100, true)
        .iter()
        .map(|row| row.iter().map(|light| *light as u32).sum::<u32>())
        .sum();

    println!("{p1}\n{p2}");
}

fn switch(i: usize, j: usize, grid: &Vec<Vec<bool>>, part2: bool) -> bool {
    if part2 && (i == 0 || i == grid.len() - 1) && (j == 0 || j == grid[0].len() - 1) {
        return true;
    }

    let i: i32 = i as i32;
    let j: i32 = j as i32;

    let mut neighbors_on: u32 = 0;

    let rows: i32 = grid.len() as i32;
    let cols: i32 = grid[0].len() as i32;

    let neighbors: [(i32, i32); 8] = [
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
    ];

    for (di, dj) in neighbors {
        let i2: i32 = i + di;
        let j2: i32 = j + dj;

        if i2 >= 0 && i2 < rows && j2 >= 0 && j2 < cols && grid[i2 as usize][j2 as usize] {
            neighbors_on += 1;
        }
    }

    if grid[i as usize][j as usize] && neighbors_on != 2 && neighbors_on != 3 {
        return false;
    } else if !grid[i as usize][j as usize] && neighbors_on == 3 {
        return true;
    } else {
        return grid[i as usize][j as usize];
    }
}

fn next_grid(grid: &Vec<Vec<bool>>, part2: bool) -> Vec<Vec<bool>> {
    let mut grid2: Vec<Vec<bool>> = grid.clone();

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            grid2[i][j] = switch(i, j, grid, part2);
        }
    }

    grid2
}

fn next_n_grid(grid: &Vec<Vec<bool>>, n: u32, part2: bool) -> Vec<Vec<bool>> {
    let mut grid2 = grid.clone();

    for _ in 0..n {
        grid2 = next_grid(&grid2, part2);
    }

    grid2
}
