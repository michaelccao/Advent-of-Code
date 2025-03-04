use crate::helper::read_data;
use std::collections::{HashMap, HashSet};
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day06.txt");

    let locations: Vec<(i32, i32)> = get_locations(&data);

    // A location has infinite area if it can infinitely expand in the
    // strict North/South/East/West direction
    // Expansion in the North is stopped by a location that is more North
    // Than East/West
    // I.e. 90 degree cone centered on North direction
    // (Anything right on the edge creates ties which also stops infinite area)
    // Therefore a location has finite area if there are locations in
    // all four quadrants

    // This also means any finite area never touches the border that
    // bounds all the locations

    // let finite_locations: Vec<(i32, i32)> = get_finite_locations(&locations);

    let p1: usize = get_areas(&locations);

    // Part 2
    // The best x,y coordinates have equal numbers of locations
    // East vs West
    // North vs South
    // Shifting by one pixel only changes distance by +/- 1 per
    // location

    let p2: u32 = best_area(&locations, 10000);

    println!("{p1}\n{p2}");
}

fn get_locations(data: &String) -> Vec<(i32, i32)> {
    let mut locations: Vec<(i32, i32)> = Vec::new();

    for line in data.lines() {
        let mut line = line.trim().split(", ");

        let x: i32 = line.next().unwrap().parse().unwrap();
        let y: i32 = line.next().unwrap().parse().unwrap();

        locations.push((x, y));
    }

    locations
}

fn get_areas(locations: &Vec<(i32, i32)>) -> usize {
    let mut x_min: i32 = locations[0].0;
    let mut y_min: i32 = locations[0].1;
    let mut x_max: i32 = x_min;
    let mut y_max: i32 = y_min;

    for i in 0..locations.len() {
        let (x, y) = locations[i];

        if x < x_min {
            x_min = x;
        }

        if x > x_max {
            x_max = x;
        }

        if y < y_min {
            y_min = y;
        }

        if y > y_max {
            y_max = y;
        }
    }

    let mut areas: HashMap<usize, Vec<(i32, i32)>> = HashMap::new();

    for x in x_min..x_max + 1 {
        for y in y_min..y_max + 1 {
            let (x2, y2): (i32, i32) = locations[0];

            let mut shortest: i32 = (x2 - x).abs() + (y2 - y).abs();
            let mut tie: bool = false;
            let mut closest: usize = 0;

            for i in 1..locations.len() {
                let (x2, y2): (i32, i32) = locations[i];

                let dist = (x2 - x).abs() + (y2 - y).abs();

                if dist < shortest {
                    shortest = dist;
                    tie = false;
                    closest = i;
                } else if dist == shortest {
                    tie = true;
                }
            }

            if !tie {
                if let Some(area) = areas.get_mut(&closest) {
                    area.push((x, y));
                } else {
                    areas.insert(closest, vec![(x, y)]);
                }
            }
        }
    }

    let mut largest_finite_area: usize = 0;

    'outer: for (_, area) in areas {
        for &(x, y) in &area {
            if x == x_min || x == x_max || y == y_min || y == y_max {
                continue 'outer;
            }
        }
        if area.len() > largest_finite_area {
            largest_finite_area = area.len();
        }
    }

    largest_finite_area
}

fn best_area(locations: &Vec<(i32, i32)>, limit: i32) -> u32 {
    let mut x_sorted: Vec<i32> = locations.iter().map(|&(x, _)| x).collect();
    x_sorted.sort();

    let mut y_sorted: Vec<i32> = locations.iter().map(|&(_, y)| y).collect();
    y_sorted.sort();

    let num_locs: usize = locations.len();

    let x: i32 = x_sorted[num_locs / 2];
    let y: i32 = y_sorted[num_locs / 2];

    let mut nodes: Vec<(i32, i32)> = vec![(x, y)];

    let dx_dy: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    let mut visited: HashSet<(i32, i32)> = HashSet::from([(x, y)]);
    let mut area: u32 = 0;

    while nodes.len() > 0 {
        let (x, y) = nodes.pop().unwrap();

        let total_dist = x_sorted.iter().map(|&x2| (x2 - x).abs()).sum::<i32>()
            + y_sorted.iter().map(|&y2| (y2 - y).abs()).sum::<i32>();

        if total_dist < limit {
            area += 1;
        } else {
            continue;
        }

        for (dx, dy) in dx_dy {
            if !visited.contains(&(x + dx, y + dy)) {
                visited.insert((x + dx, y + dy));
                nodes.push((x + dx, y + dy));
            }
        }
    }

    area
}
