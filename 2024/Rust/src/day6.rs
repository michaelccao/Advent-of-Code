use crate::helper::read_data;

use std::vec::Vec;
use std::collections::{HashSet,HashMap};


pub fn main() {
    
    let data = read_data("../Data/Day6_Test.txt");

    let (grid, i0, j0) = get_grid(&data);
    
    println!("{:?}", traverse(&grid, &i0, &j0));
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

fn traverse(grid: &Vec<Vec<char>>, i0: &usize, j0: &usize) -> (bool, usize) {
    let mut visited: HashMap<(usize, usize), HashSet<usize>> = HashMap::new();

    let mut i: usize = *i0;
    let mut j: usize = *j0;

    let mut heading: usize = 0;
    let vel: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    for _ in 0..10 {
        match visited.get_mut(&(i, j)) {
            None => {
                visited.insert((i, j), HashSet::from([heading]));
            },
            Some(headings) => {
                if headings.contains(&heading) {
                    return (true, 0)
                } else {
                    headings.insert(heading);
                }
            }
        }
    }

    return (false, visited.len())
}