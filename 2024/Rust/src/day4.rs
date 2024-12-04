use crate::helper::read_data;

use std::vec::Vec;


pub fn main() {
    
    let data = read_data("../Data/Day4.txt");

    let grid = get_grid(&data);

    let p1 = search(&grid);

    println!("{p1}");

    let p2 = search2(&grid);

    println!("{p2}");

    
}

fn get_grid(data: &String) -> Vec<Vec<char>> {

    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in data.split("\n") {
        let mut line_vec:Vec<char> = Vec::new();
        for c in line.chars() {
            if c != '\r' {
                line_vec.push(c);
            }
        }
        grid.push(line_vec);
    }

    grid

}

fn search(grid: &Vec<Vec<char>>) -> i32 {
    let dv:[[i32; 2];8] = [[1, 0], [-1, 0], [0, 1], [0, -1], [1, 1], [-1, -1], [1, -1], [-1, 1]];

    let mut total = 0;

    let rows = grid.len();
    let columns = grid[0].len();

    for i in 0..rows {
        for j in 0..columns {
            if grid[i][j] != 'X' {
                continue
            }

            for [di, dj] in dv {
                let mut word = String::new();

                for k in 0..4 {

                    let i2 = i as i32 + k*di;
                    let j2 = j as i32 + k*dj;

                    if 0 <= i2 && i2 < rows as i32 && 0 <= j2 && j2 < columns as i32 {
                        word.push(grid[i2 as usize][j2 as usize]);
                    }
                }

                if word == "XMAS" {
                    total += 1
                }
            }
          
        }
    }

    total
}

fn search2(grid: &Vec<Vec<char>>) -> i32 {
    let mut total = 0;

    let rows = grid.len();
    let columns = grid[0].len();

    for i in 0..rows {
        for j in 0..columns {
            if grid[i][j] != 'A' {
                continue
            }

            if i == 0 || i == rows-1 || j == 0 || j == columns - 1 {
                continue
            }

            let cross1 = [grid[i-1][j-1], grid[i][j], grid[i+1][j+1]];
            let cross2 = [grid[i-1][j+1], grid[i][j], grid[i+1][j-1]];

            if (cross1 == ['S', 'A', 'M'] || cross1 == ['M', 'A', 'S']) && (cross2 == ['S', 'A', 'M'] || cross2 == ['M', 'A', 'S']) {
                total += 1;
            }
          
        }
    }

    total
}