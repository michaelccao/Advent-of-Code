use crate::helper::read_data;
use std::collections::HashSet;
use std::vec::Vec;

pub fn main() {
    
    let data = read_data("../Data/Day10.txt");

    let (grid, starts) = get_grid(&data);

    let p1 = starts.clone().iter().map(|(i0, j0)| dfs(&grid, *i0, *j0, true)).sum::<usize>();
    let p2 = starts.clone().iter().map(|(i0, j0)| dfs(&grid, *i0, *j0, false)).sum::<usize>();

    println!("{p1}");
    println!("{p2}");
    
}

fn get_grid(data: &String) -> (Vec<Vec<u8>>, Vec<(usize, usize)>) {
    let mut grid: Vec<Vec<u8>> = Vec::new();
    let mut starts: Vec<(usize, usize)> = Vec::new();

    for (i, line) in data.split("\n").enumerate() {
        let mut row: Vec<u8> = Vec::new();

        for (j, c) in line.trim().chars().enumerate() {
            let level:u8 = c.to_digit(10).unwrap() as u8;
            row.push(level);

            if level == 0 {
                starts.push((i, j));
            }
        }

        grid.push(row);
    }

    (grid, starts)
}

fn dfs(grid: &Vec<Vec<u8>>, i0: usize, j0: usize, part1: bool) -> usize {
    let mut nodes: Vec<(usize, usize)> = Vec::new();
    let mut good_paths: usize = 0;
    let mut trail_ends: HashSet<(usize, usize)> = HashSet::new();
    let rows = grid.len();
    let cols = grid[0].len();

    nodes.push((i0, j0));

    while nodes.len() > 0 {
        let (i, j) = nodes.pop().unwrap();

        if grid[i][j] == 9 {
            good_paths += 1;
            trail_ends.insert((i, j));
            continue
        }

        if i > 0 && grid[i-1][j] == grid[i][j] + 1 {
            nodes.push((i-1, j));
        }

        if i < rows-1 && grid[i+1][j] == grid[i][j] + 1 {
            nodes.push((i+1, j));
        }

        if j > 0 && grid[i][j-1] == grid[i][j] + 1 {
            nodes.push((i, j-1));
        }

        if j < cols-1 && grid[i][j+1] == grid[i][j] + 1 {
            nodes.push((i, j+1));
        }

    }

    if part1 {
        return trail_ends.len();
    } else {
        return good_paths;
    }
}