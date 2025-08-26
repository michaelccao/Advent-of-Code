use crate::helper::read_data;
use std::collections::{HashMap, HashSet};

pub fn main() {
    let data: String = read_data("../Data/Day24.txt");

    // Hexagonal system can be represented with 2 vectors
    // We'll choose E, and NE as our vectors
    // W is -E
    // SW is -NE
    // SE is E - NE
    // NW is NE - E

    let flips: Vec<(i32, i32)> = get_flips(&data);

    let black_tiles: HashSet<(i32, i32)> = set_tiles(&flips);
    let p1: usize = black_tiles.len();

    println!("{p1}");

    let p2: usize = mutate_n(&black_tiles, 100).len();

    println!("{p2}");
}

fn get_flips(data: &String) -> Vec<(i32, i32)> {
    let mut flips: Vec<(i32, i32)> = Vec::new();

    for line in data.lines() {
        let mut e: i32 = 0;
        let mut ne: i32 = 0;

        let mut buffer: String = String::new();

        for c in line.chars() {
            buffer.push(c);

            match buffer.as_str() {
                "e" => {
                    e += 1;
                    buffer = String::new();
                }
                "w" => {
                    e -= 1;
                    buffer = String::new();
                }
                "ne" => {
                    ne += 1;
                    buffer = String::new();
                }
                "sw" => {
                    ne -= 1;
                    buffer = String::new();
                }
                "se" => {
                    e += 1;
                    ne -= 1;
                    buffer = String::new();
                }
                "nw" => {
                    e -= 1;
                    ne += 1;
                    buffer = String::new();
                }
                _ => {}
            }
        }

        flips.push((e, ne));
    }

    flips
}

fn set_tiles(flips: &Vec<(i32, i32)>) -> HashSet<(i32, i32)> {
    let mut black_tiles: HashSet<(i32, i32)> = HashSet::new();

    for tile in flips {
        if black_tiles.contains(tile) {
            black_tiles.remove(tile);
        } else {
            black_tiles.insert(tile.clone());
        }
    }

    black_tiles
}

fn mutate(black_tiles: &HashSet<(i32, i32)>) -> HashSet<(i32, i32)> {
    let mut black_tiles2: HashSet<(i32, i32)> = black_tiles.clone();

    let dr: [(i32, i32); 6] = [(1, 0), (0, 1), (-1, 0), (0, -1), (1, -1), (-1, 1)];

    let mut num_neighbors: HashMap<(i32, i32), u32> = HashMap::new();

    for &(i, j) in black_tiles {
        let mut neighbors: u32 = 0;
        for (di, dj) in dr {
            if black_tiles.contains(&(i + di, j + dj)) {
                neighbors += 1;
            } else {
                if let Some(count) = num_neighbors.get_mut(&(i + di, j + dj)) {
                    *count += 1;
                } else {
                    num_neighbors.insert((i + di, j + dj), 1);
                }
            }
        }

        if neighbors == 0 || neighbors > 2 {
            black_tiles2.remove(&(i, j));
        }
    }

    for (tile, count) in num_neighbors {
        if count == 2 {
            black_tiles2.insert(tile);
        }
    }

    black_tiles2
}

fn mutate_n(black_tiles: &HashSet<(i32, i32)>, n: u32) -> HashSet<(i32, i32)> {
    let mut black_tiles2: HashSet<(i32, i32)> = black_tiles.clone();

    for _ in 0..n {
        black_tiles2 = mutate(&black_tiles2);
    }

    black_tiles2
}
