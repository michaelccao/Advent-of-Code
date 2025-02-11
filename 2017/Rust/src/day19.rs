use crate::helper::read_data;
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day19.txt");

    let grid: Vec<Vec<char>> = get_grid(&data);

    let (p1, p2) = follow_path(&grid);

    println!("{p1}\n{p2}");
}

fn get_grid(data: &String) -> Vec<Vec<char>> {
    let grid: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();

    grid
}

fn follow_path(grid: &Vec<Vec<char>>) -> (String, u32) {
    let mut letters: String = String::new();
    let mut steps: u32 = 0;

    let rows: i32 = grid.len() as i32;
    let cols: i32 = grid[0].len() as i32;

    let start_j: usize = grid[0].iter().position(|&x| x == '|').unwrap();

    let mut nodes: Vec<(usize, usize, i32, i32)> = vec![(0, start_j, 1, 0)];

    let headings: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

    while nodes.len() > 0 {
        let (i, j, di, dj) = nodes.pop().unwrap();

        steps += 1;

        if grid[i][j] != '|' && grid[i][j] != '+' && grid[i][j] != '-' {
            letters.push(grid[i][j]);
        }

        let i2: i32 = i as i32 + di;
        let j2: i32 = j as i32 + dj;

        if i2 >= 0 && i2 < rows && j2 >= 0 && j2 < cols && grid[i2 as usize][j2 as usize] != ' ' {
            nodes.push((i2 as usize, j2 as usize, di, dj));
        } else {
            for (di2, dj2) in headings {
                if di2 == -di && dj2 == -dj {
                    continue;
                }

                let i2: i32 = i as i32 + di2;
                let j2: i32 = j as i32 + dj2;

                if i2 >= 0
                    && i2 < rows
                    && j2 >= 0
                    && j2 < cols
                    && grid[i2 as usize][j2 as usize] != ' '
                {
                    nodes.push((i2 as usize, j2 as usize, di2, dj2));
                    break;
                }
            }
        }
    }

    (letters, steps)
}
