use crate::helper::read_data;
use std::collections::HashMap;
use std::vec::Vec;

pub fn main() {
    let data = read_data("../Data/Day16.txt");

    let grid = get_grid(&data);

    let (i0, j0) = find_start(&grid);

    let (p1, p2) = travel(&grid, i0, j0);

    println!("{p1}, {p2}");
}

fn get_grid(data: &String) -> Vec<Vec<char>> {
    data.split('\n')
        .map(|line| line.trim().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

fn find_start(grid: &Vec<Vec<char>>) -> (i32, i32) {
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'S' {
                return (i as i32, j as i32);
            }
        }
    }

    return (-1, -1);
}

fn travel(grid: &Vec<Vec<char>>, i0: i32, j0: i32) -> (i32, u32) {
    let neighbors: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    let mut visited: HashMap<(i32, i32, i32), i32> = HashMap::new();

    let mut nodes: Vec<(i32, i32, i32, i32, Vec<(i32, i32)>)> = vec![(i0, j0, 0, 0, vec![])];

    let mut best_score: i32 = (grid.len() * grid[0].len() * 1001) as i32;

    let mut good_tiles: HashMap<(i32, i32), i32> = HashMap::new();

    while nodes.len() > 0 {
        let (i, j, heading, score, history) = nodes.pop().unwrap();

        visited.insert((i, j, heading), score);

        if score > best_score {
            continue;
        }

        if grid[i as usize][j as usize] == 'E' && score <= best_score {
            best_score = score;

            good_tiles.insert((i, j), score);

            for (i, j) in history {
                good_tiles.insert((i, j), score);
            }
            continue;
        }

        for k in -1..2 {
            let heading2 = (heading + k + 4) % 4;
            let (di, dj) = neighbors[heading2 as usize];
            let score2 = score + 1 + 1000 * k.abs();
            let i2 = i + di;
            let j2 = j + dj;
            let mut history2 = history.clone();
            history2.push((i, j));

            if grid[i2 as usize][j2 as usize] != '#'
                && (!visited.contains_key(&(i2, j2, heading2))
                    || score2 <= *visited.get(&(i2, j2, heading2)).unwrap())
            {
                nodes.push((i2, j2, heading2, score2, history2));
            }
        }
    }

    let num_good_tiles = good_tiles
        .into_iter()
        .map(|(_k, v)| if v == best_score { 1 } else { 0 })
        .sum::<u32>();

    (best_score, num_good_tiles)
}
