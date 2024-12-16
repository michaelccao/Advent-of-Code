use crate::helper::read_data;
use std::collections::HashMap;
use std::vec::Vec;

pub fn main() {
    let data = read_data("../Data/Day15.txt");

    let (mut grid, directions) = get_grid_and_directions(&data);

    let mut grid2 = get_wide_grid(&grid);

    follow_directions(&mut grid, directions);

    follow_directions(&mut grid2, directions);

    let p1 = calculate_gps(&grid, 'O');
    let p2 = calculate_gps(&grid2, '[');

    println!("{p1}");
    println!("{p2}");

}

fn get_grid_and_directions(data: &String) -> (Vec<Vec<char>>, &str) {
    let split: Vec<&str> = data.split("\r\n\r\n").collect();

    let grid = split[0];
    let directions = split[1];

    let grid = grid
        .split("\r\n")
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();

    (grid, directions)
}

fn get_wide_grid(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut grid2: Vec<Vec<char>> = Vec::new();

    for line in grid {
        let mut line2: Vec<char> = Vec::new();
        for c in line {
            match c {
                '.' => {
                    line2.push('.');
                    line2.push('.');
                },
                '@' => {
                    line2.push('@');
                    line2.push('.');
                },
                '#' => {
                    line2.push('#');
                    line2.push('#');
                },
                'O' => {
                    line2.push('[');
                    line2.push(']');
                },
                _ => ()
            }
        }
        grid2.push(line2);
    }

    grid2
}

fn push(i0: i32, j0: i32, d:char, grid: &mut Vec<Vec<char>>) -> (i32, i32) {
    let mut visited: HashMap<(i32, i32), char> = HashMap::new();
    let mut nodes: Vec<(i32, i32)> = vec![(i0, j0)];

    let (di, dj): (i32, i32) = match d {
        '>' => (0, 1),
        '<' => (0, -1),
        '^' => (-1, 0),
        'v' => (1, 0),
        _ => (0, 0),
    };

    while nodes.len() > 0 {
        let (i, j) = nodes.pop().unwrap();

        if !visited.contains_key(&(i, j)) {
            visited.insert((i, j), grid[i as usize][j as usize]);
        }

        let i2 = i + di;
        let j2 = j + dj;

        match grid[i2 as usize][j2 as usize] {
            '#' => {
                return (i0, j0)
            },
                
            'O' => {
                nodes.push((i2, j2));
            },
            '[' => {
                if !visited.contains_key(&(i2, j2)) { nodes.push((i2, j2)) };
                if !visited.contains_key(&(i2, j2 + 1)) { nodes.push((i2, j2+1))};
            },
            ']' => {
                if !visited.contains_key(&(i2, j2)) { nodes.push((i2, j2)) };
                if !visited.contains_key(&(i2, j2 - 1)) { nodes.push((i2, j2-1))};
            },
            _ => (),
            
        }
    }

    for ((i, j), feature) in &visited {
        let i = *i;
        let j = *j;
        let feature = *feature;

        let i2 = i + di;
        let j2 = j + dj;

        grid[i2 as usize][j2 as usize] = feature;
        if !visited.contains_key(&(i - di, j - dj)) {
            grid[i as usize][j as usize] = '.';
        }
    }

    return (i0 + di, j0 + dj)
}

fn follow_directions(grid: &mut Vec<Vec<char>>, directions: &str) {
    let mut i0: i32 = 0;
    let mut j0: i32 = 0;

    for (i, line) in grid.into_iter().enumerate() {
        for (j, c) in line.into_iter().enumerate() {
            if *c == '@' {
                i0 = i as i32;
                j0 = j as i32;
            }
        }
    }

    for d in directions.chars() {
        if d == '\r' || d == '\n' {
            continue
        }
    
        (i0, j0) = push(i0, j0, d, grid);
        
        
    }
}

fn calculate_gps(grid: &Vec<Vec<char>>, box_char: char) -> u32 {

    let mut total: u32 = 0;

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == box_char {
                total += (100*i + j) as u32;
            }
        }
    }

    total
}
