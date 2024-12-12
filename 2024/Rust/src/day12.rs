use crate::helper::read_data;
use std::collections::{HashSet, HashMap};
use std::vec::Vec;

pub fn main() {
    
    let data = read_data("../Data/Day12.txt");
    
    let grid = get_grid(&data);

    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut p1: u32 = 0;
    let mut p2: u32 = 0;

    for i0 in 0..grid.len() {
        for j0 in 0..grid[0].len() {
            if !visited.contains(&(i0 as i32, j0 as i32)) {
                let (v, p, s) = get_area(i0, j0, &grid);
                
                visited.extend(&v);

                p1 += (v.len() as u32)*(p as u32);
                p2 += (v.len() as u32)*s
            }
        }
    }

    println!("{p1}");
    println!("{p2}");
}

fn get_grid(data: &String) -> Vec<Vec<char>> {
    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in data.split("\n") {
        let mut row: Vec<char> = Vec::new();

        for c in line.trim().chars() {
            row.push(c);
        }

        grid.push(row);
    }

    grid
}

fn get_area(i0: usize, j0: usize, grid: &Vec<Vec<char>>) -> (HashSet<(i32, i32)>, usize, u32) {

    let neighbors:HashMap<char, (i32, i32)> = HashMap::from([
        ('N', (-1, 0)),
        ('S', (1, 0)),
        ('E', (0, 1)),
        ('W', (0, -1))
    ]);

    let plant:char = grid[i0][j0];
    let rows:i32 = grid.len() as i32;
    let cols:i32 = grid[0].len() as i32;

    let i0:i32 = i0 as i32;
    let j0:i32 = j0 as i32;

    let mut nodes:Vec<(i32, i32)> = Vec::from([(i0, j0)]);
    let mut fences: HashSet<(i32, i32, char)> = HashSet::new();
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    while nodes.len() > 0 {
        let (i, j) = nodes.pop().unwrap();
        visited.insert((i, j));

        for (&heading, (di, dj)) in &neighbors {
            let i2 = i + di;
            let j2 = j + dj;

            if i2 >= 0 && i2 < rows && j2 >= 0 && j2 < cols && grid[i2 as usize][j2 as usize] == plant{

                if !visited.contains(&(i2, j2)) {
                    nodes.push((i2, j2));
                }
            } else {
                fences.insert((i, j, heading));
            }
        }
    }

    let perimeter = fences.len();

    let mut sides:u32 = 0;

    while fences.len() > 0 {
        
        let (i, j, heading) = fences.iter().next().unwrap().clone();

        fences.take(&(i, j, heading));
        
        let mut side:Vec<(i32, i32, char)> = vec![(i, j, heading)];
        sides += 1;
        

        while side.len() > 0 {

            let (i, j, heading) = side.pop().unwrap();

            for (_, (di, dj)) in &neighbors {
                if fences.contains(&(i+di, j+dj, heading)) {
                    fences.take(&(i+di, j+dj, heading));
                    side.push((i+di, j+dj, heading));
                }
            }
        }
    }

    (visited, perimeter, sides)

}