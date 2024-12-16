use crate::helper::read_data;
use std::collections::{HashSet, HashMap};
use std::vec::Vec;
use regex::Regex;

pub fn main() {
    let data = read_data("../Data/Day14.txt");

    let robots = get_robots(&data);

    let p1_robots = move_robots(&robots, 100, 103, 101);

    let p1 = calculate_safety(&p1_robots, 103, 101);


    // Main idea: target drawing is highly ordered
    // Moving 103 steps back will keep all y-coordinates, but shift x-coordinates
    // Likewise moving 101 steps back will keep all x-coordinates, but shift y-coordinates
    // In either case, the picture will still remain some semblence of order
    // Looking at first 100 cases, we noticed a highly ordered picutre at t = 71 that cycles every 101 steps
    // Iterating in steps of 101 finds target picture
    let p2_robots = move_robots(&robots, 8050, 103, 101);

    let p2 = draw_robots(&p2_robots, 103, 101);

    println!("{p2}");


}

fn get_robots(data: &String) -> Vec<(i32, i32, i32, i32)> {

    let re = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();

    let robots = re.captures_iter(data).map(|caps| {
        let (_, [x, y, dx, dy]) = caps.extract();
        let x = x.parse::<i32>().unwrap();
        let y = y.parse::<i32>().unwrap();
        let dx = dx.parse::<i32>().unwrap();
        let dy = dy.parse::<i32>().unwrap();

        (x, y, dx, dy)
    }).collect::<Vec<_>>();

    robots

}

fn move_robots(robots: &Vec<(i32, i32, i32, i32)>, t: i32, rows: i32, cols: i32) -> Vec<(i32, i32)> {

    robots.into_iter().map(|(x, y, dx, dy)| ( ((x + t*dx) % cols + cols) % cols, ((y + t*dy) % rows + rows) % rows ) ).collect::<Vec<(i32, i32)>>()

}

fn calculate_safety(robots: &Vec<(i32, i32)>, rows:i32, cols: i32) -> i32 {

    let mut quad1: i32 = 0;
    let mut quad2: i32 = 0;
    let mut quad3: i32 = 0;
    let mut quad4: i32 = 0;

    for (x, y) in robots {
        let x = *x;
        let y = *y;

        if x < cols / 2 && y < rows / 2 {
            quad1 += 1
        } else if x > cols / 2 && y < rows / 2 {
            quad2 += 1
        } else if x < cols / 2 && y > rows / 2 {
            quad3 += 1
        } else if x > cols / 2 && y > rows / 2 {
            quad4 += 1
        }
    }

    quad1*quad2*quad3*quad4
}

fn draw_robots(robots: &Vec<(i32, i32)>, rows: usize, cols:usize) -> String {
    let mut grid = vec![vec!['.'; cols]; rows];

    let mut output_string: String = String::new();

    for (x, y) in robots {
        grid[*y as usize][*x as usize] = '#';
    }

    for line in grid {
        for c in line {
            output_string.push(c);
        }
        output_string.push('\n');
    }

    output_string
}