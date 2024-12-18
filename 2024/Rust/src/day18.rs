use crate::helper::read_data;
use std::collections::{HashMap, HashSet};
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day18.txt");
    let obstacles: Vec<(i32, i32)> = get_obstacles(&data);

    let p1:i32 = travel(0, 0, &obstacles, 1024, 71, 71);
    let p2: (i32, i32) = find_first_blocker(&obstacles, 1024, 71, 71);

    println!("{p1}\n{p2:?}");
}

fn get_obstacles(data: &String) -> Vec<(i32, i32)> {
    let mut obstacles: Vec<(i32, i32)> = Vec::new();

    for line in data.split('\n') {
        let coords: Vec<i32> = line.trim().split(',').map(|num| num.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        obstacles.push((coords[0], coords[1]));
    }

    obstacles
}

fn travel(i0: i32, j0: i32, obstacles: &Vec<(i32, i32)>, t: usize, height: i32, width: i32) -> i32 {

    let obstacles: HashSet<(i32, i32)> = HashSet::from_iter(obstacles[..t].iter().cloned());

    let mut visited: HashMap<(i32, i32), i32> = HashMap::new();

    let mut nodes: Vec<(i32, i32, i32)> = vec![(i0, j0, 0)];

    let mut best_score: i32 = height*width;

    let neighbors: [(i32, i32); 4] = [
        (0, 1),
        (1, 0),
        (0, -1),
        (-1, 0),
    ];

    while nodes.len() > 0 {
        let (i, j, score): (i32, i32, i32) = nodes.pop().unwrap();

        visited.insert((i, j), score);

        if score >= best_score {
            continue;
        }

        if i == height - 1 && j == width - 1 && score < best_score {
            best_score = score;
        }

        for (di, dj) in neighbors {
            let i2: i32 = i + di;
            let j2: i32 = j + dj;
            let score2: i32 = score + 1;

            let i_valid: bool = i2 >= 0 && i2 < height;
            let j_valid: bool = j2 >= 0 && j2 < height;
            
            let clear: bool = !obstacles.contains(&(j2, i2));

            let improvement: bool = !visited.contains_key(&(i2, j2)) || *visited.get(&(i2, j2)).unwrap() > score2;

            if i_valid && j_valid && clear && improvement {
                nodes.push((i2, j2, score2));
            }
        }
    }

    best_score
}

fn find_first_blocker(obstacles: &Vec<(i32, i32)>, lower_bound: usize, height: i32, width: i32) -> (i32, i32) {
    let mut lower_bound: usize = lower_bound;
    let mut upper_bound: usize = obstacles.len();

    while lower_bound < upper_bound - 1 {
        let t: usize = (lower_bound + upper_bound) / 2;
        let score: i32 = travel(0, 0, obstacles, t, height, width);

        if score < height*width {
            lower_bound = t;
        } else {
            upper_bound = t;
        }
    }

    obstacles[lower_bound]
}