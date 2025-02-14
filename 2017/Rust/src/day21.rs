use crate::helper::read_data;
use std::collections::HashMap;
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day21.txt");

    let rules: HashMap<Vec<Vec<bool>>, Vec<Vec<bool>>> = get_rules(&data);

    let p1: u64 = enhance_n(&rules, 5);
    let p2: u64 = enhance_n(&rules, 18);

    println!("{p1}\n{p2}");
}

fn enhance_n(rules: &HashMap<Vec<Vec<bool>>, Vec<Vec<bool>>>, n: u32) -> u64 {
    let mut grid: Vec<Vec<bool>> = vec![
        vec![false, true, false],
        vec![false, false, true],
        vec![true, true, true],
    ];

    for _ in 0..n {
        grid = enhance(&grid, rules);
    }

    grid.iter()
        .map(|row| row.iter().map(|&b| b as u64).sum::<u64>())
        .sum::<u64>()
}

fn get_rules(data: &String) -> HashMap<Vec<Vec<bool>>, Vec<Vec<bool>>> {
    let mut rules: HashMap<Vec<Vec<bool>>, Vec<Vec<bool>>> = HashMap::new();

    for line in data.lines() {
        let mut line = line.trim().split(" => ");
        let pattern_str: &str = line.next().unwrap();
        let enhanced_str: &str = line.next().unwrap();

        let mut pattern: Vec<Vec<bool>> = str_to_grid(pattern_str);
        let enhanced: Vec<Vec<bool>> = str_to_grid(enhanced_str);

        rules.insert(pattern.clone(), enhanced.clone());
        rules.insert(vflip(&pattern), enhanced.clone());
        rules.insert(hflip(&pattern), enhanced.clone());

        for _ in 0..3 {
            pattern = rotate(&pattern);

            rules.insert(pattern.clone(), enhanced.clone());
            rules.insert(vflip(&pattern), enhanced.clone());
            rules.insert(hflip(&pattern), enhanced.clone());
        }
    }

    rules
}

fn str_to_grid(pattern: &str) -> Vec<Vec<bool>> {
    let mut grid: Vec<Vec<bool>> = Vec::new();
    let mut rows = pattern.split('/');

    loop {
        if let Some(row) = rows.next() {
            let row: Vec<bool> = row.chars().map(|c| c == '#').collect();
            grid.push(row);
        } else {
            break;
        }
    }

    grid
}

// Flip along central vertical axis
fn hflip(grid: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut grid: Vec<Vec<bool>> = grid.clone();
    let cols: usize = grid[0].len();

    for row in 0..grid.len() {
        for j in 0..grid[row].len() / 2 {
            grid[row].swap(j, cols - j - 1);
        }
    }

    grid
}

// Flip along central horizontal axis
fn vflip(grid: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut grid: Vec<Vec<bool>> = grid.clone();
    let rows: usize = grid.len();

    for i in 0..grid.len() / 2 {
        grid.swap(i, rows - 1 - i);
    }

    grid
}

// Rotate 90 deg clockwise
fn rotate(grid: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut grid2: Vec<Vec<bool>> = grid.clone();
    let rows: usize = grid2.len();
    let cols: usize = grid2[0].len();

    for i in 0..rows {
        for j in 0..cols {
            grid2[j][rows - 1 - i] = grid[i][j];
        }
    }

    grid2
}

fn subdivide(grid: &Vec<Vec<bool>>) -> Vec<Vec<Vec<Vec<bool>>>> {
    let size: usize = grid.len();
    let subsize: usize = if size % 2 == 0 { 2 } else { 3 };

    let mut subs: Vec<Vec<Vec<Vec<bool>>>> =
        vec![vec![vec![vec![false; subsize]; subsize]; size / subsize]; size / subsize];

    for i in 0..size / subsize {
        for j in 0..size / subsize {
            for k in 0..subsize {
                for l in 0..subsize {
                    subs[i][j][k][l] = grid[subsize * i + k][subsize * j + l];
                }
            }
        }
    }

    subs
}

fn regroup(subs: &Vec<Vec<Vec<Vec<bool>>>>) -> Vec<Vec<bool>> {
    let num_subs: usize = subs.len();
    let subsize: usize = subs[0][0].len();

    let mut grid: Vec<Vec<bool>> = vec![vec![false; subsize * num_subs]; subsize * num_subs];

    for i in 0..num_subs {
        for j in 0..num_subs {
            for k in 0..subsize {
                for l in 0..subsize {
                    grid[subsize * i + k][subsize * j + l] = subs[i][j][k][l];
                }
            }
        }
    }

    grid
}

fn enhance(
    grid: &Vec<Vec<bool>>,
    rules: &HashMap<Vec<Vec<bool>>, Vec<Vec<bool>>>,
) -> Vec<Vec<bool>> {
    let mut subs: Vec<Vec<Vec<Vec<bool>>>> = subdivide(grid);

    for i in 0..subs.len() {
        for j in 0..subs[0].len() {
            subs[i][j] = rules[&subs[i][j]].clone();
        }
    }

    regroup(&subs)
}
