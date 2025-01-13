use crate::helper::read_data;
use std::collections::{HashMap, HashSet, VecDeque};
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day22.txt");

    let disk: Vec<Vec<(i32, i32)>> = get_disk(&data);

    let p1: usize = get_viable_pairs(&disk).len();

    let p2: usize = move_data(&disk, (disk.len() - 1, 0));

    println!("{p1}\n{p2}");
}

fn get_disk(data: &String) -> Vec<Vec<(i32, i32)>> {
    let mut disk: Vec<Vec<(i32, i32)>> = Vec::new();

    let mut row: Vec<(i32, i32)> = Vec::new();
    let mut row_num: i32 = 0;

    for line in data.lines() {
        let line: Vec<&str> = line.trim().split_whitespace().collect();

        if !line[0].starts_with("/dev") {
            continue;
        }

        let mut xy = line[0].split('-');
        xy.next();

        let x: i32 = xy.next().unwrap()[1..].parse().unwrap();

        let used: i32 = line[2].split('T').next().unwrap().parse().unwrap();
        let avail: i32 = line[3].split('T').next().unwrap().parse().unwrap();

        if x == row_num {
            row.push((used, avail));
        } else {
            disk.push(row);
            row = Vec::new();
            row.push((used, avail));

            row_num += 1;
        }
    }

    disk.push(row);

    disk
}

fn get_viable_pairs(disk: &Vec<Vec<(i32, i32)>>) -> HashSet<((usize, usize), (usize, usize))> {
    let mut viable_pairs: HashSet<((usize, usize), (usize, usize))> = HashSet::new();

    let rows: usize = disk.len();
    let cols: usize = disk[0].len();

    for i in 0..rows {
        for j in 0..cols {
            for k in 0..rows {
                for l in 0..cols {
                    if i == k && j == l {
                        continue;
                    }

                    let d1: (i32, i32) = disk[i][j];
                    let d2: (i32, i32) = disk[k][l];

                    if d1.0 > 0 && d1.0 <= d2.1 {
                        viable_pairs.insert(((i, j), (k, l)));
                    }
                }
            }
        }
    }

    viable_pairs
}

fn move_data(disk: &Vec<Vec<(i32, i32)>>, mut target: (usize, usize)) -> usize {
    let rows: usize = disk.len();
    let cols: usize = disk[0].len();

    let pairs: HashSet<((usize, usize), (usize, usize))> = get_viable_pairs(disk);

    // We see that the only the empty sector can be moved into
    let move_into: HashSet<(usize, usize)> = pairs.iter().map(|k| k.1).collect();
    assert!(move_into.len() == 1);

    let mut empty: (usize, usize) = *move_into.iter().next().unwrap();

    // Most sectors have data that is less than the max space of all sectors
    // Identify unmoveable sectors
    let forbidden: HashSet<(usize, usize)> = get_unmovable(disk);

    // Get shortest path from target to (0, 0)
    let path: Vec<(usize, usize)> = shortest_path(target, (0, 0), &forbidden, rows, cols).unwrap();

    // Idea: Move our empty sector to "pave" our shortest path from target to (0, 0)
    // Each step, we move from our empty sector to the next sector in our path
    // Then make a move to swap empty sector and target sector
    // Then repeat

    let mut steps: usize = 0;

    // Our forbidden sectors are our large filled sectors plus the target sector
    let mut forbidden2 = forbidden.clone();
    forbidden2.insert(target);

    for p in path {
        // Move empty to path
        steps += shortest_path(empty, p, &forbidden2, rows, cols)
            .unwrap()
            .len();

        // Swap empty with target
        empty = target;
        target = p;
        steps += 1;

        // Update forbidden2
        forbidden2 = forbidden.clone();
        forbidden2.insert(target);
    }

    steps
}

fn get_unmovable(disk: &Vec<Vec<(i32, i32)>>) -> HashSet<(usize, usize)> {
    let min_space: i32 = disk
        .iter()
        .map(|row| {
            row.iter()
                .map(|(used, avail)| *used + *avail)
                .min()
                .unwrap()
        })
        .min()
        .unwrap();

    let mut unmovable: HashSet<(usize, usize)> = HashSet::new();

    for i in 0..disk.len() {
        for j in 0..disk[0].len() {
            if disk[i][j].0 > min_space {
                unmovable.insert((i, j));
            }
        }
    }

    unmovable
}

fn shortest_path(
    start: (usize, usize),
    end: (usize, usize),
    forbidden: &HashSet<(usize, usize)>,
    rows: usize,
    cols: usize,
) -> Option<Vec<(usize, usize)>> {
    let mut shortest: Option<Vec<(usize, usize)>> = None;

    let mut visited: HashMap<(usize, usize), usize> = HashMap::new();

    let mut nodes: VecDeque<Vec<(usize, usize)>> = VecDeque::from([vec![start]]);

    while nodes.len() > 0 {
        let path: Vec<(usize, usize)> = nodes.pop_front().unwrap();

        let &(i, j) = path.last().unwrap();

        if (i, j) == end {
            if shortest.is_none() || shortest.clone().unwrap().len() > path.len() {
                shortest = Some(path);
            }
            continue;
        }

        if shortest.is_some() && path.len() >= shortest.clone().unwrap().len() {
            continue;
        }

        let mut neighbors: Vec<(usize, usize)> = Vec::new();

        if i > 0 {
            neighbors.push((i - 1, j));
        }
        if i < rows - 1 {
            neighbors.push((i + 1, j));
        }
        if j > 0 {
            neighbors.push((i, j - 1));
        }
        if j < cols - 1 {
            neighbors.push((i, j + 1));
        }

        // Use Manhattan distance to prioritize our DFS
        neighbors.sort_by_key(|&(i2, j2)| i2.abs_diff(end.0) + j2.abs_diff(end.1));
        neighbors.reverse();

        for neighbor in neighbors {
            if !forbidden.contains(&neighbor)
                && (!visited.contains_key(&neighbor) || visited[&neighbor] > path.len() + 1)
            {
                visited.insert(neighbor.clone(), path.len() + 1);

                let mut path2: Vec<(usize, usize)> = path.clone();
                path2.push(neighbor);

                nodes.push_front(path2);
            }
        }
    }

    // Don't include start in path
    if let Some(path) = shortest {
        Some(path[1..].to_vec())
    } else {
        None
    }
}
