use crate::helper::read_data;
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day20.txt");

    let grid: Vec<Vec<char>> = get_grid(&data);

    let (path, manhattan): (Vec<(i32, i32)>, Vec<Vec<i32>>) = legal_race(&grid);

    let p1: u32 = find_shortcuts(&path, &manhattan, 2, 100);
    let p2: u32 = find_shortcuts(&path, &manhattan, 20, 100);

    println!("{p1}\n{p2}");
}

fn get_grid(data: &String) -> Vec<Vec<char>> {
    data.lines().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>()
}

fn legal_race(grid: &Vec<Vec<char>>) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {

    let mut path: Vec<(i32, i32)> = Vec::new();
    let mut manhattan: Vec<Vec<i32>> = Vec::new();

    let mut i0: i32 = 0;
    let mut j0: i32 = 0;

    'outer: for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'S' {
                i0 = i as i32;
                j0 = j as i32;
                break 'outer
            }
        }
    }

    let mut next: Vec<(i32, i32, usize)> = vec![(i0, j0, 0)];

    let neighbors: [(i32, i32); 4] = [
        (0, 1),
        (1, 0),
        (0, -1),
        (-1, 0)
    ];

    while next.len() > 0 {
        let (i, j, t): (i32, i32, usize) = next.pop().unwrap();

        path.push((i, j));
        manhattan.push(vec![0;t+1]);

        for prior_t in 0..t {
            let (i_p, j_p): (i32, i32) = path[prior_t];

            let m_dist: i32 = (i - i_p).abs() + (j - j_p).abs();

            manhattan[prior_t].push(m_dist);

        }

        if grid[i as usize][j as usize] == 'E' {
            continue
        }

        for (di, dj) in neighbors {
            let i2: i32 = i + di;
            let j2: i32 = j + dj;

            if grid[i2 as usize][j2 as usize] != '#' && ((t < 1) || (i2, j2) != path[t-1]) {
                next.push((i2, j2, t+1));
                continue
            }
        }
    }


    (path, manhattan)
    
    

}

fn find_shortcuts(path: &Vec<(i32, i32)>, manhattan: &Vec<Vec<i32>>, cheat_time: i32, margin: i32) -> u32 {
    let mut cheats: u32 = 0;

    for t1 in 0..path.len()-3 {
        for t2 in t1+3..path.len() {
            let shortcut_time: i32 = manhattan[t1][t2];
            let time_save:i32  = (t2 - t1) as i32 - manhattan[t1][t2];

            if shortcut_time <= cheat_time && time_save >= margin {
                cheats += 1;
            }
        }
    }

    cheats
}