use crate::helper::read_data;
use std::collections::HashSet;

pub fn main() {
    let data: String = read_data("../Data/Day11.txt");

    let lights: Vec<Vec<u32>> = data
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let p1: usize = count_flashes(&lights, 100);

    println!("{p1}");

    let p2: usize = find_sync(&lights);

    println!("{p2}");
}

fn count_flashes(lights: &Vec<Vec<u32>>, steps: u32) -> usize {
    let mut lights: Vec<Vec<u32>> = lights.clone();
    let mut num_flashes: usize = 0;

    for _ in 0..steps {
        let mut flashes: Vec<(usize, usize)> = Vec::new();

        for i in 0..lights.len() {
            for j in 0..lights[i].len() {
                lights[i][j] += 1;

                if lights[i][j] == 10 {
                    flashes.push((i, j));
                }
            }
        }

        let mut flashed: HashSet<(usize, usize)> = HashSet::new();

        while flashes.len() > 0 {
            let (i, j) = flashes.pop().unwrap();

            let neighbors = [
                (i - 1, j - 1),
                (i - 1, j),
                (i - 1, j + 1),
                (i, j - 1),
                (i, j + 1),
                (i + 1, j - 1),
                (i + 1, j),
                (i + 1, j + 1),
            ];

            for (i2, j2) in neighbors {
                if i2 < lights.len() && j2 < lights[i2].len() {
                    lights[i2][j2] += 1;
                    if lights[i2][j2] == 10 && !flashed.contains(&(i2, j2)) {
                        flashes.push((i2, j2));
                    }
                }
            }

            flashed.insert((i, j));
        }

        num_flashes += flashed.len();

        for (i, j) in flashed {
            lights[i][j] = 0;
        }
    }

    num_flashes
}

fn find_sync(lights: &Vec<Vec<u32>>) -> usize {
    let mut lights: Vec<Vec<u32>> = lights.clone();
    let mut steps: usize = 0;

    loop {
        steps += 1;
        let mut flashes: Vec<(usize, usize)> = Vec::new();

        for i in 0..lights.len() {
            for j in 0..lights[i].len() {
                lights[i][j] += 1;

                if lights[i][j] == 10 {
                    flashes.push((i, j));
                }
            }
        }

        let mut flashed: HashSet<(usize, usize)> = HashSet::new();

        while flashes.len() > 0 {
            let (i, j) = flashes.pop().unwrap();

            let neighbors = [
                (i - 1, j - 1),
                (i - 1, j),
                (i - 1, j + 1),
                (i, j - 1),
                (i, j + 1),
                (i + 1, j - 1),
                (i + 1, j),
                (i + 1, j + 1),
            ];

            for (i2, j2) in neighbors {
                if i2 < lights.len() && j2 < lights[i2].len() {
                    lights[i2][j2] += 1;
                    if lights[i2][j2] == 10 && !flashed.contains(&(i2, j2)) {
                        flashes.push((i2, j2));
                    }
                }
            }

            flashed.insert((i, j));
        }

        if flashed.len() == 100 {
            return steps;
        }

        for (i, j) in flashed {
            lights[i][j] = 0;
        }
    }
}
