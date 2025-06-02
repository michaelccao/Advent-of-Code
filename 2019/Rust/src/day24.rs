use crate::helper::read_data;
use std::collections::{HashMap, HashSet};

pub fn main() {
    let data: String = read_data("../Data/Day24.txt");
    let grid: Vec<Vec<bool>> = data
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect();

    let p1: u32 = cycle(&grid);

    println!("{p1}");

    let p2: u32 = recursive_mutation(&grid);

    println!("{p2}");
}

fn mutate(grid: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut grid2 = grid.clone();

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let mut neighbors = 0;

            if i > 0 && grid[i - 1][j] {
                neighbors += 1;
            }

            if i + 1 < grid.len() && grid[i + 1][j] {
                neighbors += 1;
            }

            if j > 0 && grid[i][j - 1] {
                neighbors += 1;
            }

            if j + 1 < grid[i].len() && grid[i][j + 1] {
                neighbors += 1;
            }

            if grid[i][j] && neighbors != 1 {
                grid2[i][j] = false;
            } else if !grid[i][j] && (neighbors == 1 || neighbors == 2) {
                grid2[i][j] = true;
            }
        }
    }

    grid2
}

fn score_grid(grid: &Vec<Vec<bool>>) -> u32 {
    let mut score: u32 = 0;

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] {
                score += 2_u32.pow((i * grid[0].len() + j) as u32)
            }
        }
    }

    score
}

fn cycle(grid: &Vec<Vec<bool>>) -> u32 {
    let mut stages: HashSet<Vec<Vec<bool>>> = HashSet::new();

    stages.insert(grid.clone());

    let mut grid: Vec<Vec<bool>> = grid.clone();

    loop {
        grid = mutate(&grid);

        if !stages.insert(grid.clone()) {
            return score_grid(&grid);
        }
    }
}

struct RecursiveGrid {
    grid: HashMap<(i32, i32, i32), bool>,
    size: i32,
    min_depth: i32,
    max_depth: i32,
}

impl RecursiveGrid {
    fn count_neighboring_bugs(&self, depth: i32, i: i32, j: i32) -> u32 {
        let mut neighbors: Vec<(i32, i32, i32)> = Vec::new();

        let middle: i32 = self.size / 2;

        // Top
        if i == 0 {
            neighbors.push((depth - 1, middle - 1, middle));
        } else if i - 1 == middle && j == middle {
            for j2 in 0..self.size {
                neighbors.push((depth + 1, self.size - 1, j2))
            }
        } else {
            neighbors.push((depth, i - 1, j));
        }

        // Right
        if j == self.size - 1 {
            neighbors.push((depth - 1, middle, middle + 1))
        } else if j + 1 == middle && i == middle {
            for i2 in 0..self.size {
                neighbors.push((depth + 1, i2, 0));
            }
        } else {
            neighbors.push((depth, i, j + 1));
        }

        // Down
        if i == self.size - 1 {
            neighbors.push((depth - 1, middle + 1, middle));
        } else if i + 1 == middle && j == middle {
            for j2 in 0..self.size {
                neighbors.push((depth + 1, 0, j2));
            }
        } else {
            neighbors.push((depth, i + 1, j));
        }

        // Left
        if j == 0 {
            neighbors.push((depth - 1, middle, middle - 1));
        } else if j - 1 == middle && i == middle {
            for i2 in 0..self.size {
                neighbors.push((depth + 1, i2, self.size - 1));
            }
        } else {
            neighbors.push((depth, i, j - 1));
        }

        let mut neighboring_bugs: u32 = 0;

        for (d2, i2, j2) in neighbors {
            if self.grid.get(&(d2, i2, j2)) == Some(&true) {
                neighboring_bugs += 1;
            }
        }

        neighboring_bugs
    }

    fn mutate(&mut self) {
        let mut grid2: HashMap<(i32, i32, i32), bool> = self.grid.clone();

        for depth in self.min_depth - 1..self.max_depth + 2 {
            for i in 0..self.size {
                for j in 0..self.size {
                    if i == self.size / 2 && j == self.size / 2 {
                        continue;
                    }
                    let neighboring_bugs = self.count_neighboring_bugs(depth, i, j);
                    let current: bool = *self.grid.get(&(depth, i, j)).unwrap_or(&false);

                    if current && neighboring_bugs != 1 {
                        grid2.insert((depth, i, j), false);
                        self.min_depth = self.min_depth.min(depth);
                        self.max_depth = self.max_depth.max(depth);
                    } else if !current && (neighboring_bugs == 1 || neighboring_bugs == 2) {
                        grid2.insert((depth, i, j), true);
                        self.min_depth = self.min_depth.min(depth);
                        self.max_depth = self.max_depth.max(depth);
                    }
                }
            }
        }

        self.grid = grid2;
    }
}

fn recursive_mutation(grid: &Vec<Vec<bool>>) -> u32 {
    let mut rc: RecursiveGrid = RecursiveGrid {
        grid: HashMap::new(),
        size: grid.len() as i32,
        min_depth: 0,
        max_depth: 0,
    };

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            rc.grid.insert((0, i as i32, j as i32), grid[i][j]);
        }
    }

    for _ in 0..200 {
        rc.mutate();
    }

    rc.grid
        .values()
        .map(|&b| if b { 1_u32 } else { 0_u32 })
        .sum()
}
