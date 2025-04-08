use crate::helper::read_data;
use std::collections::HashMap;
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day18.txt");
    let landscape: Landscape = get_landscape(&data);

    let p1: u32 = landscape.transform_n(10).score();

    println!("{p1}");

    let p2: u32 = landscape.transform_n(1000000000).score();

    println!("{p2}");
}

fn get_landscape(data: &String) -> Landscape {
    let mut grid: Vec<Vec<Acre>> = Vec::new();

    for line in data.lines() {
        let mut row: Vec<Acre> = Vec::new();
        for c in line.trim().chars() {
            match c {
                '.' => row.push(Acre::Open),
                '|' => row.push(Acre::Tree),
                '#' => row.push(Acre::Lumberyard),
                _ => {}
            }
        }
        grid.push(row);
    }

    Landscape { grid }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Landscape {
    grid: Vec<Vec<Acre>>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Acre {
    Open,
    Tree,
    Lumberyard,
}

impl Landscape {
    fn transform(&self) -> Landscape {
        let mut grid2 = self.grid.clone();

        for i in 0..grid2.len() {
            for j in 0..grid2[0].len() {
                let mut neighbors: HashMap<Acre, u32> =
                    HashMap::from([(Acre::Open, 0), (Acre::Tree, 0), (Acre::Lumberyard, 0)]);

                let top: bool = i == 0;
                let left: bool = j == 0;
                let bottom: bool = i == grid2.len() - 1;
                let right: bool = j == grid2[0].len() - 1;

                if !top {
                    *neighbors.get_mut(&self.grid[i - 1][j]).unwrap() += 1;
                }
                if !left {
                    *neighbors.get_mut(&self.grid[i][j - 1]).unwrap() += 1;
                }
                if !bottom {
                    *neighbors.get_mut(&self.grid[i + 1][j]).unwrap() += 1;
                }
                if !right {
                    *neighbors.get_mut(&self.grid[i][j + 1]).unwrap() += 1;
                }
                if !top && !left {
                    *neighbors.get_mut(&self.grid[i - 1][j - 1]).unwrap() += 1;
                }
                if !top && !right {
                    *neighbors.get_mut(&self.grid[i - 1][j + 1]).unwrap() += 1;
                }
                if !bottom && !left {
                    *neighbors.get_mut(&self.grid[i + 1][j - 1]).unwrap() += 1;
                }
                if !bottom && !right {
                    *neighbors.get_mut(&self.grid[i + 1][j + 1]).unwrap() += 1;
                }

                match grid2[i][j] {
                    Acre::Open => {
                        if neighbors[&Acre::Tree] >= 3 {
                            grid2[i][j] = Acre::Tree
                        }
                    }
                    Acre::Tree => {
                        if neighbors[&Acre::Lumberyard] >= 3 {
                            grid2[i][j] = Acre::Lumberyard
                        }
                    }
                    Acre::Lumberyard => {
                        if neighbors[&Acre::Lumberyard] == 0 || neighbors[&Acre::Tree] == 0 {
                            grid2[i][j] = Acre::Open
                        }
                    }
                }
            }
        }

        Landscape { grid: grid2 }
    }

    fn transform_n(&self, n: u64) -> Landscape {
        let mut landscape: Landscape = self.clone();

        let mut landscapes: HashMap<Landscape, u64> = HashMap::from([(landscape.clone(), 0)]);

        let mut buffer: Vec<Landscape> = Vec::from([landscape.clone()]);

        for t in 1..n + 1 {
            landscape = landscape.transform();

            if landscapes.contains_key(&landscape) {
                let prev_t: u64 = landscapes[&landscape];
                let cycle_length: u64 = t - prev_t;
                let target_t: u64 = prev_t + (n - prev_t) % cycle_length;

                return buffer[target_t as usize].clone();
            } else {
                landscapes.insert(landscape.clone(), t);
                buffer.push(landscape.clone());
            }
        }

        landscape
    }

    fn score(&self) -> u32 {
        let mut count: HashMap<Acre, u32> =
            HashMap::from([(Acre::Open, 0), (Acre::Tree, 0), (Acre::Lumberyard, 0)]);

        for row in &self.grid {
            for acre in row {
                *count.get_mut(acre).unwrap() += 1;
            }
        }

        count[&Acre::Tree] * count[&Acre::Lumberyard]
    }

    fn to_string(&self) -> String {
        let mut output = String::new();

        for row in &self.grid {
            for acre in row {
                match *acre {
                    Acre::Open => output.push('.'),
                    Acre::Tree => output.push('|'),
                    Acre::Lumberyard => output.push('#'),
                }
            }
            output.push('\n');
        }

        output
    }
}
