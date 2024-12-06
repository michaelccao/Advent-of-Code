use crate::helper::read_data;

use std::vec::Vec;
use std::collections::{HashSet,HashMap};


pub fn main() {
    
    let data = read_data("../Data/Day6.txt");

    let (mut grid, i0, j0) = get_grid(&data);
    
    let (_, visited) = traverse(&grid, &i0, &j0);

    println!("{}", visited.len());

    println!("{}", find_obstacles(&mut grid, &i0, &j0, &visited));
}

fn get_grid(data: &String) -> (Vec<Vec<char>>, usize, usize) {

    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut i0:usize = 0;
    let mut j0:usize = 0;

    for (i, line) in data.split("\n").enumerate() {
        let mut line_vec:Vec<char> = Vec::new();
        for (j, c) in line.chars().enumerate() {
            if c != '\r' {
                line_vec.push(c);
            }

            if c == '^' {
                i0 = i;
                j0 = j;
            }
        }
        grid.push(line_vec);
    }
    
    (grid, i0, j0)

}

fn traverse(grid: &Vec<Vec<char>>, i0: &usize, j0: &usize) -> (bool, HashMap<(i32, i32), HashSet<usize>>) {
    let mut visited: HashMap<(i32, i32), HashSet<usize>> = HashMap::new();

    let mut i: i32 = *i0 as i32;
    let mut j: i32 = *j0 as i32;

    let rows = grid.len();
    let cols = grid[0].len();

    let mut heading: usize = 0;
    let vel: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    while i >= 0 && i < rows as i32 && j >= 0 && j < cols as i32 {
        match visited.get_mut(&(i, j)) {
            None => {
                visited.insert((i, j), HashSet::from([heading]));
            },
            Some(headings) => {
                if headings.contains(&heading) {
                    return (true, visited);
                } else {
                    headings.insert(heading);
                }
            }
        };

        let (di, dj) = vel[heading];

        let i2 = i + di;
        let j2 = j + dj;

        if i2 >= 0 && i2 < rows as i32 && j2 >= 0 && j2 < cols as i32 && grid[i2 as usize][j2 as usize] == '#' {
            heading = (heading + 1) % 4;
        } else {
            i = i2;
            j = j2;
        }

    }

    return (false, visited);
}

fn find_obstacles(grid: &mut Vec<Vec<char>>, i0: &usize, j0: &usize, visited: &HashMap<(i32, i32), HashSet<usize>>) -> i32 {
    let mut obstacles:i32 = 0;

    for ((i, j), _) in visited {
        if *i as usize == *i0 && *j as usize == *j0 {
            continue
        }

        grid[*i as usize][*j as usize] = '#';

        if traverse(&grid, i0, j0).0 {
            obstacles += 1;
        }

        grid[*i as usize][*j as usize] = '.';
    }

    obstacles

    
}