use crate::helper::read_data;
use std::collections::{HashMap, HashSet, VecDeque};

pub fn main() {
    let data: String = read_data("../Data/Day18.txt");

    let (grid, entrance, keys) = get_grid(&data);

    let p1: u32 = grab_keys(&grid, entrance, &keys);

    println!("{p1}");

    let p2: u32 = grab_keys_robo(&grid, entrance, &keys);

    println!("{p2}");
}

fn get_grid(
    data: &String,
) -> (
    Vec<Vec<char>>,
    (usize, usize),
    HashMap<char, (usize, usize)>,
) {
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut entrance: (usize, usize) = (0, 0);
    let mut keys: HashMap<char, (usize, usize)> = HashMap::new();

    for (i, line) in data.lines().enumerate() {
        let mut row: Vec<char> = Vec::new();
        for (j, c) in line.chars().enumerate() {
            if c == '@' {
                entrance = (i, j);
            } else if c != '#' && c != '.' && c.is_ascii_lowercase() {
                keys.insert(c, (i, j));
            }

            row.push(c)
        }

        grid.push(row);
    }

    (grid, entrance, keys)
}

fn explore(
    grid: &Vec<Vec<char>>,
    start: (usize, usize),
    keys: &HashSet<char>,
) -> HashMap<char, u32> {
    let mut reachable_keys: HashMap<char, u32> = HashMap::new();

    let mut nodes: VecDeque<(usize, usize, u32)> = VecDeque::from([(start.0, start.1, 0)]);

    let mut visited: HashMap<(usize, usize), u32> = HashMap::from([((start.0, start.1), 0)]);

    while nodes.len() > 0 {
        let (i, j, steps) = nodes.pop_front().unwrap();

        if grid[i][j].is_lowercase() && !keys.contains(&grid[i][j]) {
            reachable_keys.insert(grid[i][j], steps);
            continue;
        }

        let mut neighbors: Vec<(usize, usize)> = Vec::new();

        if i > 0 {
            neighbors.push((i - 1, j));
        }

        if i < grid.len() - 1 {
            neighbors.push((i + 1, j));
        }

        if j > 0 {
            neighbors.push((i, j - 1));
        }

        if j < grid[0].len() - 1 {
            neighbors.push((i, j + 1));
        }

        for (i2, j2) in neighbors {
            if (grid[i2][j2] == '.'
                || grid[i2][j2].is_ascii_lowercase()
                || keys.contains(&grid[i2][j2].to_lowercase().next().unwrap()))
                && steps + 1 < *visited.get(&(i2, j2)).unwrap_or(&u32::MAX)
            {
                visited.insert((i2, j2), steps + 1);
                nodes.push_back((i2, j2, steps + 1));
            }
        }
    }

    reachable_keys
}

fn grab_keys(
    grid: &Vec<Vec<char>>,
    entrance: (usize, usize),
    keys: &HashMap<char, (usize, usize)>,
) -> u32 {
    let mut nodes: Vec<(usize, usize, Vec<char>, u32)> =
        vec![(entrance.0, entrance.1, Vec::new(), 0)];

    let mut visited: HashMap<(usize, usize, Vec<char>), u32> =
        HashMap::from([((entrance.0, entrance.1, Vec::new()), 0)]);

    let mut shortest: u32 = u32::MAX;

    while nodes.len() > 0 {
        let (i, j, held_keys, steps) = nodes.pop().unwrap();

        if steps + (keys.len() - held_keys.len()) as u32 >= shortest {
            continue;
        }

        if held_keys.len() == keys.len() {
            shortest = shortest.min(steps);
            continue;
        }

        for (next_key, add_steps) in explore(grid, (i, j), &held_keys.iter().cloned().collect()) {
            let mut held_keys2 = held_keys.clone();
            held_keys2.push(next_key);
            held_keys2.sort();

            let (i2, j2) = keys[&next_key];

            if steps + add_steps
                < *visited
                    .get(&(i2, j2, held_keys2.clone()))
                    .unwrap_or(&shortest)
            {
                visited.insert((i2, j2, held_keys2.clone()), steps + add_steps);
                nodes.push((i2, j2, held_keys2, steps + add_steps));
            }
        }
    }

    shortest
}

fn grab_keys_robo(
    grid: &Vec<Vec<char>>,
    entrance: (usize, usize),
    keys: &HashMap<char, (usize, usize)>,
) -> u32 {
    let mut grid: Vec<Vec<char>> = grid.clone();
    let (i0, j0) = entrance;

    grid[i0][j0] = '#';
    grid[i0 + 1][j0] = '#';
    grid[i0 - 1][j0] = '#';
    grid[i0][j0 - 1] = '#';
    grid[i0][j0 + 1] = '#';

    let mut nodes: Vec<(
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
        Vec<char>,
        u32,
    )> = vec![(
        i0 - 1,
        j0 - 1,
        i0 + 1,
        j0 - 1,
        i0 - 1,
        j0 + 1,
        i0 + 1,
        j0 + 1,
        Vec::new(),
        0,
    )];

    let mut visited: HashMap<
        (
            usize,
            usize,
            usize,
            usize,
            usize,
            usize,
            usize,
            usize,
            Vec<char>,
        ),
        u32,
    > = HashMap::from([(
        (
            i0 - 1,
            j0 - 1,
            i0 + 1,
            j0 - 1,
            i0 - 1,
            j0 + 1,
            i0 + 1,
            j0 + 1,
            Vec::new(),
        ),
        0,
    )]);

    let mut shortest: u32 = u32::MAX;

    while nodes.len() > 0 {
        let (i, j, i2, j2, i3, j3, i4, j4, held_keys, steps) = nodes.pop().unwrap();

        if steps + (keys.len() - held_keys.len()) as u32 >= shortest {
            continue;
        }

        if held_keys.len() == keys.len() {
            shortest = shortest.min(steps);
            continue;
        }

        let held_keys_set: HashSet<char> = held_keys.iter().cloned().collect();

        let robot1: HashMap<char, u32> = explore(&grid, (i, j), &held_keys_set);
        let robot2: HashMap<char, u32> = explore(&grid, (i2, j2), &held_keys_set);
        let robot3: HashMap<char, u32> = explore(&grid, (i3, j3), &held_keys_set);
        let robot4: HashMap<char, u32> = explore(&grid, (i4, j4), &held_keys_set);

        for (next_key, add_steps) in robot1 {
            let mut held_keys2 = held_keys.clone();
            held_keys2.push(next_key);
            held_keys2.sort();

            let (i5, j5) = keys[&next_key];

            if steps + add_steps
                < *visited
                    .get(&(i5, j5, i2, j2, i3, j3, i4, j4, held_keys2.clone()))
                    .unwrap_or(&shortest)
            {
                visited.insert(
                    (i5, j5, i2, j2, i3, j3, i4, j4, held_keys2.clone()),
                    steps + add_steps,
                );
                nodes.push((
                    i5,
                    j5,
                    i2,
                    j2,
                    i3,
                    j3,
                    i4,
                    j4,
                    held_keys2,
                    steps + add_steps,
                ));
            }
        }

        for (next_key, add_steps) in robot2 {
            let mut held_keys2 = held_keys.clone();
            held_keys2.push(next_key);
            held_keys2.sort();

            let (i5, j5) = keys[&next_key];

            if steps + add_steps
                < *visited
                    .get(&(i, j, i5, j5, i3, j3, i4, j4, held_keys2.clone()))
                    .unwrap_or(&shortest)
            {
                visited.insert(
                    (i, j, i5, j5, i3, j3, i4, j4, held_keys2.clone()),
                    steps + add_steps,
                );
                nodes.push((i, j, i5, j5, i3, j3, i4, j4, held_keys2, steps + add_steps));
            }
        }

        for (next_key, add_steps) in robot3 {
            let mut held_keys2 = held_keys.clone();
            held_keys2.push(next_key);
            held_keys2.sort();

            let (i5, j5) = keys[&next_key];

            if steps + add_steps
                < *visited
                    .get(&(i, j, i2, j2, i5, j5, i4, j4, held_keys2.clone()))
                    .unwrap_or(&shortest)
            {
                visited.insert(
                    (i, j, i2, j2, i5, j5, i4, j4, held_keys2.clone()),
                    steps + add_steps,
                );
                nodes.push((i, j, i2, j2, i5, j5, i4, j4, held_keys2, steps + add_steps));
            }
        }

        for (next_key, add_steps) in robot4 {
            let mut held_keys2 = held_keys.clone();
            held_keys2.push(next_key);
            held_keys2.sort();

            let (i5, j5) = keys[&next_key];

            if steps + add_steps
                < *visited
                    .get(&(i, j, i2, j2, i3, j3, i5, j5, held_keys2.clone()))
                    .unwrap_or(&shortest)
            {
                visited.insert(
                    (i, j, i2, j2, i3, j3, i5, j5, held_keys2.clone()),
                    steps + add_steps,
                );
                nodes.push((i, j, i2, j2, i3, j3, i5, j5, held_keys2, steps + add_steps));
            }
        }
    }

    shortest
}
