use crate::helper::read_data;
use std::collections::{HashMap, HashSet};
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day13.txt");

    let fav_num: i32 = data.parse().unwrap();

    let visited = shortest_path(fav_num, 31, 39);

    let p1 = visited[&(31, 39)];

    let p2 = visited.values().filter(|dist| **dist <= 50).collect::<Vec<_>>().len();

    println!("{p1}\n{p2}");
}

fn is_open(x: i32, y: i32, fav_num: i32) -> bool {
    let mut num: i32 = x*x + 3*x + 2*x*y + y + y*y;
    num += fav_num;

    num.count_ones() % 2 == 0
}

fn shortest_path(fav_num: i32, target_x: i32, target_y: i32) -> HashMap<(i32, i32), u32> {

    let mut visited: HashMap<(i32, i32), u32> = HashMap::new();

    let mut nodes: Vec<(i32, i32, u32)> = vec![(1, 1, 0)];

    let neighbors: [(i32, i32); 4] = [
        (0, 1),
        (1, 0),
        (0, -1),
        (-1, 0)
    ];

    while nodes.len() > 0 {
        let (x, y, steps) = nodes.pop().unwrap();

        visited.insert((x, y), steps);

        if x == target_x && y == target_y {
            continue;
        }

        if let Some(shortest) = visited.get(&(31, 39)) {
            if steps >= *shortest {
                continue;
            }
        }

        for (dx, dy) in neighbors {
            let x2 = x + dx;
            let y2 = y + dy;

            if x2 >= 0 && y2 >= 0 && is_open(x2, y2, fav_num) && (!visited.contains_key(&(x2, y2)) || visited[&(x2, y2)] > steps + 1) {

                nodes.push((x2, y2, steps+1));

            }
        }


    }

    visited
}