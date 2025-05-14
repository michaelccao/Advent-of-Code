use crate::helper::read_data;
use std::collections::{HashMap, HashSet, VecDeque};

pub fn main() {
    let data: String = read_data("../Data/Day20.txt");

    let grid: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();

    let (start, finish, warps) = get_warps(&grid);

    let paths: HashMap<(usize, usize), HashMap<(usize, usize), usize>> =
        get_paths(&grid, &warps, start, finish);

    let p1: usize = shortest_path(start, finish, &warps, &paths);

    println!("{p1}");

    let p2: usize = shortest_path_levels(&grid, start, finish, &warps, &paths);

    println!("{p2}");
}

fn shortest_path(
    start: (usize, usize),
    finish: (usize, usize),
    warps: &HashMap<(usize, usize), (usize, usize)>,
    paths: &HashMap<(usize, usize), HashMap<(usize, usize), usize>>,
) -> usize {
    let mut nodes: VecDeque<(usize, usize, usize)> = VecDeque::from([(start.0, start.1, 0)]);
    let mut visited: HashMap<(usize, usize), usize> = HashMap::from([((start.0, start.1), 0)]);

    let mut shortest: usize = usize::MAX;

    while nodes.len() > 0 {
        let (i, j, steps) = nodes.pop_front().unwrap();

        if (i, j) == finish {
            shortest = shortest.min(steps);
            continue;
        }

        if steps >= shortest {
            continue;
        }

        let destinations: &HashMap<(usize, usize), usize> = &paths[&(i, j)];

        for (&warp, &add_steps) in destinations.iter() {
            let (i2, j2) = if warp == finish { finish } else { warps[&warp] };
            let mut steps2 = steps + add_steps;
            if warp != finish {
                steps2 += 1;
            }

            if steps2 < *visited.get(&(i2, j2)).unwrap_or(&shortest) {
                visited.insert((i2, j2), steps2);
                nodes.push_back((i2, j2, steps2));
            }
        }
    }

    shortest
}

fn shortest_path_levels(
    grid: &Vec<Vec<char>>,
    start: (usize, usize),
    finish: (usize, usize),
    warps: &HashMap<(usize, usize), (usize, usize)>,
    paths: &HashMap<(usize, usize), HashMap<(usize, usize), usize>>,
) -> usize {
    let mut nodes: VecDeque<(usize, usize, usize, usize)> =
        VecDeque::from([(start.0, start.1, 0, 0)]);
    let mut visited: HashMap<(usize, usize, usize), usize> =
        HashMap::from([((start.0, start.1, 0), 0)]);

    let mut shortest: usize = usize::MAX;

    while nodes.len() > 0 {
        let (i, j, level, steps) = nodes.pop_front().unwrap();

        if steps >= shortest {
            continue;
        }

        let destinations: &HashMap<(usize, usize), usize> = &paths[&(i, j)];

        for (&warp, &add_steps) in destinations.iter() {
            if warp == finish {
                if level == 0 {
                    shortest = shortest.min(steps + add_steps);
                }

                continue;
            }

            let outer: bool = warp.0 == 2
                || warp.0 == grid.len() - 3
                || warp.1 == 2
                || warp.1 == grid[0].len() - 3;

            if level == 0 && outer {
                continue;
            }

            let steps2: usize = steps + add_steps + 1;
            let (i2, j2) = warps[&warp];
            let level2: usize = if outer { level - 1 } else { level + 1 };

            if steps2 < *visited.get(&(i2, j2, level2)).unwrap_or(&shortest) {
                visited.insert((i2, j2, level2), steps2);
                nodes.push_back((i2, j2, level2, steps2));
            }
        }
    }

    shortest
}

fn explore(
    grid: &Vec<Vec<char>>,
    warps: &HashMap<(usize, usize), (usize, usize)>,
    start: (usize, usize),
    finish: (usize, usize),
) -> HashMap<(usize, usize), usize> {
    let mut nodes: VecDeque<(usize, usize, usize)> = VecDeque::from([(start.0, start.1, 0)]);
    let mut visited: HashMap<(usize, usize), usize> = HashMap::from([((start.0, start.1), 0)]);

    let mut destinations: HashMap<(usize, usize), usize> = HashMap::new();

    while nodes.len() > 0 {
        let (i, j, steps) = nodes.pop_front().unwrap();

        if (warps.contains_key(&(i, j)) || (i, j) == finish) && (i, j) != start {
            if steps < *destinations.get(&(i, j)).unwrap_or(&usize::MAX) {
                destinations.insert((i, j), steps);
            }
        }

        for (i2, j2) in [(i - 1, j), (i, j + 1), (i + 1, j), (i, j - 1)] {
            if grid[i2][j2] == '.' && steps + 1 < *visited.get(&(i2, j2)).unwrap_or(&usize::MAX) {
                visited.insert((i2, j2), steps + 1);
                nodes.push_back((i2, j2, steps + 1));
            }
        }
    }

    destinations
}

fn get_paths(
    grid: &Vec<Vec<char>>,
    warps: &HashMap<(usize, usize), (usize, usize)>,
    start: (usize, usize),
    finish: (usize, usize),
) -> HashMap<(usize, usize), HashMap<(usize, usize), usize>> {
    let mut starts: Vec<(usize, usize)> = warps.keys().cloned().collect();
    starts.push(start);

    let mut paths: HashMap<(usize, usize), HashMap<(usize, usize), usize>> = HashMap::new();

    for start in starts {
        paths.insert(start, explore(grid, warps, start, finish));
    }

    paths
}

fn get_warps(
    grid: &Vec<Vec<char>>,
) -> (
    (usize, usize),
    (usize, usize),
    HashMap<(usize, usize), (usize, usize)>,
) {
    let mut warps: HashMap<String, HashSet<(usize, usize)>> = HashMap::new();

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            let c: char = grid[i][j];

            if c.is_ascii_alphabetic() {
                let first_letter = c;
                let mut second_letter: char = ' ';
                let mut warp: (usize, usize) = (0, 0);

                if i + 1 < grid.len() && grid[i + 1][j].is_ascii_alphabetic() {
                    second_letter = grid[i + 1][j];

                    if i + 2 < grid.len() && grid[i + 2][j] == '.' {
                        warp = (i + 2, j);
                    } else {
                        warp = (i - 1, j);
                    }
                } else if j + 1 < grid[0].len() && grid[i][j + 1].is_ascii_alphabetic() {
                    second_letter = grid[i][j + 1];

                    if j + 2 < grid[0].len() && grid[i][j + 2] == '.' {
                        warp = (i, j + 2);
                    } else {
                        warp = (i, j - 1);
                    }
                } else {
                    continue;
                }

                let warp_key: String = format!("{first_letter}{second_letter}");

                warps = update_warps(warps, warp_key, warp)
            }
        }
    }

    let start: (usize, usize) = *warps[&"AA".to_string()].iter().next().unwrap();
    let finish: (usize, usize) = *warps[&"ZZ".to_string()].iter().next().unwrap();

    let mut warps2: HashMap<(usize, usize), (usize, usize)> = HashMap::new();

    for (_, warps) in warps.iter() {
        let warps: Vec<(usize, usize)> = warps.iter().cloned().collect();
        if warps.len() == 2 {
            warps2.insert(warps[0], warps[1]);
            warps2.insert(warps[1], warps[0]);
        }
    }

    (start, finish, warps2)
}

fn update_warps(
    mut warps: HashMap<String, HashSet<(usize, usize)>>,
    warp_key: String,
    warp: (usize, usize),
) -> HashMap<String, HashSet<(usize, usize)>> {
    if let Some(points) = warps.get_mut(&warp_key) {
        points.insert(warp);
    } else {
        warps.insert(warp_key, HashSet::from([warp]));
    }

    warps
}
