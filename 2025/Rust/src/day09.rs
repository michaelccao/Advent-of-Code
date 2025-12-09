use crate::helper::read_data;
use std::collections::{HashMap, HashSet};

pub fn main() {
    let data: String = read_data("../Data/Day09.txt");

    let tiles: Vec<Vec<u64>> = data.lines().map(|line| line.split(",").map(|s| s.parse().unwrap()).collect()).collect();

    let p1 = largest_rect(&tiles);

    println!("{p1}");

    let p2 = largest_contained_rect(&tiles);

    println!("{p2}");
}

fn largest_rect(tiles: &Vec<Vec<u64>>) -> u64 {

    let mut largest: u64 = 0;

    for i in 0..tiles.len() {
        let x0 = &tiles[i];

        for j in i+1..tiles.len() {
            let x1 = &tiles[j];

            largest = largest.max((x0[0].abs_diff(x1[0]) + 1) * (x0[1].abs_diff(x1[1]) + 1) );
        }
    }

    largest
}

fn largest_contained_rect(tiles: &Vec<Vec<u64>>) -> u64 {

    let mut fences: HashMap<u64, HashSet<u64>> = HashMap::new();

    for i in 0..tiles.len() {
        let x0 = tiles[i][0];
        let y0 = tiles[i][1];

        let x1 = tiles[(i+1) % tiles.len()][0];
        let y1 = tiles[(i+1) % tiles.len()][1];

        for x in x0.min(x1)..x0.max(x1)+1 {
            for y in y0.min(y1)..y0.max(y1)+1 {

                if let Some(ys) = fences.get_mut(&x) {
                    ys.insert(y);
                } else {
                    fences.insert(x, HashSet::from([y]));
                }

            }
        }


    }

    let mut largest: u64 = 0;

    let mut cache: HashMap<(u64, u64), bool> = HashMap::new();

    for i in 0..tiles.len() {
        let x0 = tiles[i][0];
        let y0 = tiles[i][1];
        'candidate: for j in i+1..tiles.len() {
            let x1 = tiles[j][0];
            let y1 = tiles[j][1];

            let area = largest.max((x0.abs_diff(x1) + 1) * (y0.abs_diff(y1) + 1));

            if area <= largest {
                continue;
            }

            for x in x0.min(x1)..x0.max(x1)+1 {
                if outside(x, y1, &fences, &mut cache) || outside(x, y0, &fences, &mut cache) {
                    continue 'candidate
                }
            }

            for y in y0.min(y1)..y0.max(y1)+1 {
                if outside(x0, y, &fences, &mut cache) || outside(x1, y, &fences, &mut cache) {
                    continue 'candidate
                }
            }
            
            largest = area;
        }
    }


    largest
}

// Draw line from x, y to x, inf.
// If line crosses fence even number of times, (x, y) is outside
fn outside(x: u64, y: u64, fences: &HashMap<u64, HashSet<u64>>, cache: &mut HashMap<(u64, u64), bool>) -> bool {

    if let Some(&is_out) = cache.get(&(x, y)) {
        return is_out
    }

    let mut crossings: u32 = 0;
    
    if let Some(ys) = fences.get(&x) {
        if ys.contains(&y) {
            return false;
        }
        for &fy in ys {
            if fy > y && !ys.contains(&(fy+1)) {
                crossings += 1;
            }
        }
    }

    cache.insert((x, y), crossings % 2 == 0);

    cache[&(x, y)]
}
