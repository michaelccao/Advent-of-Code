use crate::helper::read_data;
use std::collections::{HashMap, HashSet};

pub fn main() {
    let data: String = read_data("../Data/Day07.txt");

    let grid: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();

    let p1: usize = split_beams(&grid);

    println!("{p1}");

    let p2: usize = beam_paths(&grid, 0, grid[0].len() / 2, HashMap::new()).0;

    println!("{p2}");
}

fn split_beams(grid: &Vec<Vec<char>>) -> usize {
    let mut beams: HashSet<usize> = HashSet::new();
    let mut splits: usize = 0;

    for j in 0..grid[0].len() {
        if grid[0][j] == 'S' {
            beams.insert(j);
        }
    }

    for i in 1..grid.len() {
        let mut beams2: HashSet<usize> = HashSet::new();

        for &beam in &beams {
            if grid[i][beam] == '^' {
                beams2.insert(beam - 1);
                beams2.insert(beam + 1);

                splits += 1;
            } else {
                beams2.insert(beam);
            }
        }

        beams = beams2;
    }

    splits
}

fn beam_paths(
    grid: &Vec<Vec<char>>,
    i: usize,
    j: usize,
    mut cache: HashMap<(usize, usize), usize>,
) -> (usize, HashMap<(usize, usize), usize>) {
    if let Some(&paths) = cache.get(&(i, j)) {
        return (paths, cache);
    }

    if i + 1 == grid.len() {
        return (1, cache);
    }

    if grid[i][j] != '^' {
        let res = beam_paths(grid, i + 1, j, cache);

        cache = res.1;
        cache.insert((i, j), res.0);

        return (res.0, cache);
    } else {
        let left = beam_paths(grid, i + 1, j - 1, cache);
        cache = left.1;

        let right = beam_paths(grid, i + 1, j + 1, cache);
        cache = right.1;

        cache.insert((i, j), left.0 + right.0);

        return (left.0 + right.0, cache);
    }
}
