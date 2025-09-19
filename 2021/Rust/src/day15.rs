use crate::helper::read_data;
use std::collections::{BinaryHeap, HashMap};

pub fn main() {
    let data: String = read_data("../Data/Day15.txt");
    let grid: Vec<Vec<usize>> = data.lines().map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as usize).collect()).collect();

    let p1: usize = find_shortest_path(&grid);

    println!("{p1}");

    let p2: usize = find_shortest_path(&make_big_tile(&grid));

    println!("{p2}");
}

fn find_shortest_path(grid:&Vec<Vec<usize>>) -> usize {
    let mut nodes: BinaryHeap<(i32, usize, usize, usize)> = BinaryHeap::from([(0, 0, 0, 0)]);

    let mut shortest: usize = usize::MAX;

    let mut visited: HashMap<(usize, usize), usize> = HashMap::from([((0, 0), 0)]);

    let dest_i: usize = grid.len() - 1;
    let dest_j: usize = grid[0].len() - 1;

    while let Some((_total_cost, base_cost, i, j)) = nodes.pop() {

        if i == dest_i && j == dest_j {
            shortest = shortest.min(base_cost);
        }

        let neighbors: [(usize, usize); 4] = [
            (i-1, j),
            (i, j-1),
            (i+1, j),
            (i, j+1),
        ];

        for (i2, j2) in neighbors {
            if i2 < grid.len() && j2 < grid.len() {
                let base_cost2: usize = base_cost + grid[i2][j2];
                let total_cost2: usize = base_cost2 + dest_i - i2 + dest_j - j2;
                if let Some(&visited_cost) = visited.get(&(i2, j2)) {
                    if total_cost2 < visited_cost {
                        visited.insert((i2, j2), total_cost2);
                        nodes.push(((total_cost2 as i32)*-1, base_cost2, i2, j2));
                    }
                } else {
                    visited.insert((i2, j2), total_cost2);
                    nodes.push(((total_cost2 as i32)*-1, base_cost2, i2, j2));
                }
            }
        }
    }

    shortest
}

fn make_big_tile(grid: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {

    let mut grid2: Vec<Vec<usize>> = vec![vec![0;grid[0].len()*5];grid.len()*5];

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            for tile_i in 0..5 {
                for tile_j in 0..5 {
                    let mut cost: usize = grid[i][j] + tile_i + tile_j;
                    while cost > 9 {
                        cost -= 9;
                    }
                    grid2[tile_i*grid.len() + i][tile_j*grid[i].len() + j] = cost;
                }
            }
        }
    }

    grid2
}