use crate::helper::read_data;

pub fn main() {
    let data: String = read_data("../Data/Day03.txt");
    let grid: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();

    let p1: u64 = ride_toboggan(&grid, 3, 1);

    println!("{p1}");

    let slopes: Vec<(usize, usize)> = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let p2: u64 = trial_slopes(&grid, &slopes);

    println!("{p2}");
}

fn ride_toboggan(grid: &Vec<Vec<char>>, right: usize, down: usize) -> u64 {
    let mut trees: u64 = 0;

    let mut i: usize = 0;
    let mut j: usize = 0;

    while i < grid.len() {
        if grid[i][j] == '#' {
            trees += 1;
        }

        i += down;
        j += right;
        j %= grid[0].len();
    }

    trees
}

fn trial_slopes(grid: &Vec<Vec<char>>, slopes: &Vec<(usize, usize)>) -> u64 {
    let mut total: u64 = 1;

    for &(right, down) in slopes {
        total *= ride_toboggan(grid, right, down)
    }

    total
}
