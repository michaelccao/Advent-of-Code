use crate::helper::read_data;

pub fn main() {
    let data: String = read_data("../Data/Day04.txt");
    let grid: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();

    let p1: usize = collect_rolls(&grid).0;

    println!("{p1}");

    let p2: usize = collect_rolls2(&grid);

    println!("{p2}");
}

fn collect_rolls(grid: &Vec<Vec<char>>) -> (usize, Vec<Vec<char>>) {
    let mut total: usize = 0;

    let mut grid2: Vec<Vec<char>> = grid.clone();

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] != '@' {
                continue;
            }
            let mut neighbors: usize = 0;

            for (i2, j2) in [
                (i - 1, j - 1),
                (i - 1, j),
                (i - 1, j + 1),
                (i, j - 1),
                (i, j + 1),
                (i + 1, j - 1),
                (i + 1, j),
                (i + 1, j + 1),
            ] {
                if i2 < grid.len() && j2 < grid[i].len() {
                    if grid[i2][j2] == '@' {
                        neighbors += 1;
                    }
                }
            }

            if neighbors < 4 {
                total += 1;
                grid2[i][j] = '.'
            }
        }
    }

    (total, grid2)
}

fn collect_rolls2(grid: &Vec<Vec<char>>) -> usize {
    let mut grid2: Vec<Vec<char>> = grid.clone();

    let mut total: usize = 0;

    loop {
        let res: (usize, Vec<Vec<char>>) = collect_rolls(&grid2);

        grid2 = res.1;
        total += res.0;

        if res.0 == 0 {
            break;
        }
    }

    total
}
