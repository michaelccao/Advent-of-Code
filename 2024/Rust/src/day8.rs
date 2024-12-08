use crate::helper::read_data;
use std::collections::{HashMap, HashSet};

pub fn main() {
    
    let data = read_data("../Data/Day8.txt");

    let (ants, rows, cols) = get_ants(&data);

    let p1 = solve(&ants, &rows, &cols, false);

    let p2 = solve(&ants, &rows, &cols, true);

    println!("{}", p1);
    println!("{}", p2);
    
}

fn get_ants(data: &String) -> (HashMap<char, Vec<(i32, i32)>>, i32, i32) {

    let mut ants: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let mut rows:i32 = 0;
    let mut cols:i32 = 0;
    
    for (x, line) in data.split("\n").enumerate() {
        rows = x as i32;
        for (y, c) in line.trim().chars().enumerate() {
            if c != '.' {
                match ants.get_mut(&c) {
                    Some(coords) => { coords.push((x as i32, y as i32)); },
                    None => { ants.insert(c, vec![(x as i32, y as i32)]); }
                };
            }
            cols = y as i32;
        }
    }

    rows += 1;
    cols += 1;

    (ants, rows, cols)
}

fn solve(ants: &HashMap<char, Vec<(i32, i32)>>, rows: &i32, cols: &i32, part2: bool) -> usize {

    let mut nodes:HashSet<(i32, i32)> = HashSet::new();

    for (ant, coords) in ants {
        for i in 0..coords.len() {
            for j in i+1..coords.len() {
                let (x, y) = coords[i];
                let (x2, y2) = coords[j];

                let dx = x2 - x;
                let dy = y2 - y;

                let candidates:Vec<(i32, i32)>;

                if part2 {
                    candidates = (-rows-cols-1..rows+cols+1).map(|step| (x + step*dx, y + step*dy)).collect();
                } else {
                    candidates = vec![(x + 2*dx, y + 2*dy), (x - dx, y - dy)];
                }
                
                for (nx, ny) in candidates {
                    if nx >= 0 && nx < *rows && ny >= 0 && ny < *cols {
                        nodes.insert((nx, ny));
                    }
                }
                
            }
        }
    }

    nodes.len()
}