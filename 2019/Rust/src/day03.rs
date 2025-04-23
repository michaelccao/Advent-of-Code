use crate::helper::read_data;
use std::collections::{HashMap, HashSet};

pub fn main() {
    let data: String = read_data("../Data/Day03.txt");
    let routes: Vec<Vec<(&str, i32)>> = data
        .lines()
        .map(|line| {
            line.split(",")
                .map(|d| (&d[0..1], d[1..].parse::<i32>().unwrap()))
                .collect::<Vec<(&str, i32)>>()
        })
        .collect();

    let p1: i32 = trace_routes(&routes, false);

    println!("{p1}");

    let p2: i32 = trace_routes(&routes, true);

    println!("{p2}");
}

fn trace_routes(routes: &Vec<Vec<(&str, i32)>>, part2: bool) -> i32 {
    let mut trace: Vec<HashMap<(i32, i32), i32>> = vec![HashMap::new(); routes.len()];

    for i in 0..routes.len() {
        let route: &Vec<(&str, i32)> = &routes[i];

        let mut x: i32 = 0;
        let mut y: i32 = 0;

        let mut total_steps: i32 = 0;

        for &(direction, steps) in route {
            for _ in 0..steps {
                if direction == "U" {
                    y += 1;
                } else if direction == "D" {
                    y -= 1;
                } else if direction == "L" {
                    x -= 1;
                } else if direction == "R" {
                    x += 1;
                }

                total_steps += 1;

                if !trace[i].contains_key(&(x, y)) {
                    trace[i].insert((x, y), total_steps);
                }
            }
        }
    }

    let mut intersections: Vec<(i32, i32)> = trace[0]
        .keys()
        .collect::<HashSet<_>>()
        .intersection(&trace[1].keys().collect::<HashSet<_>>())
        .cloned()
        .cloned()
        .collect();

    if part2 {
        intersections.sort_by_key(|coords| trace[0][&coords] + trace[1][&coords]);

        trace[0][&intersections[0]] + trace[1][&intersections[0]]
    } else {
        intersections.sort_by_key(|&(x, y)| x.abs() + y.abs());

        intersections[0].0.abs() + intersections[0].1.abs()
    }
}
