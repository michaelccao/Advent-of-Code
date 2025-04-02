use crate::helper::read_data;
use std::collections::{HashMap, HashSet, VecDeque};

pub fn main() {
    let data: String = read_data("../Data/Day15.txt");

    let (grid, units) = get_grid(&data);

    let p1: i32 = resolve_fight(&grid, &units);

    println!("{p1}");

}

fn get_grid(data: &String) -> (Vec<Vec<(char, i32)>>, HashSet<(usize, usize)>) {
    let mut grid: Vec<Vec<(char, i32)>> = Vec::new();
    let mut units: HashSet<(usize, usize)> = HashSet::new();

    for (i, line) in data.lines().enumerate() {
        let line = line.split_whitespace().next().unwrap();
        let mut row: Vec<(char, i32)> = Vec::new();

        for (j, c) in line.chars().enumerate() {
            if c == 'G' || c == 'E' {
                row.push((c, 200));
                units.insert((i, j));
            } else {
                row.push((c, 0));
            }
        }

        grid.push(row);
    }

    (grid, units)
}

fn resolve_fight(grid: &Vec<Vec<(char, i32)>>, units: &HashSet<(usize, usize)>) -> i32 {
    let mut grid: Vec<Vec<(char, i32)>> = grid.clone();
    let mut units: HashSet<(usize, usize)> = units.clone();

    let mut elves: u8 = 0;
    let mut goblins: u8 = 0;

    for &(i, j) in &units {
        if grid[i][j].0 == 'G' {
            goblins += 1;
        } else if grid[i][j].0 == 'E' {
            elves += 1;
        }
    }

    let mut rounds: i32 = 0;

    'outer: loop {
        let mut turn_order: Vec<(usize, usize)> = units.iter().cloned().collect();

        turn_order.sort();

        // println!("{turn_order:?}");

        // println!("start round {rounds}");

        print_grid(&grid);

        for (i, j) in turn_order {
            let (c, h) = grid[i][j];

            // Unit may already be dead from previous units' actions
            if c == '.' {
                continue;
            }

            if (c == 'G' && elves == 0) || (c == 'E' && goblins == 0) {
                break 'outer;
            }

            let enemy: char = if c == 'E' { 'G' } else { 'E' };

            // Move
            let (i2, j2) = find_best_move(&grid, i, j, enemy);

            grid[i][j] = ('.', 0);
            grid[i2][j2] = (c, h);

            units.remove(&(i, j));
            units.insert((i2, j2));

            // Attack
            let mut weakest_enemy: (i32, usize, usize) = (201, i2, j2);

            for (ei, ej) in [(i2 - 1, j2), (i2, j2 - 1), (i2, j2 + 1), (i2 + 1, j2)] {
                let (e, h): (char, i32) = grid[ei][ej];

                if e == enemy && (h, ei, ej) < weakest_enemy {
                    weakest_enemy = (h, ei, ej);
                }
            }

            if weakest_enemy.0 <= 200 {
                let (_, ei, ej): (i32, usize, usize) = weakest_enemy;

                grid[ei][ej].1 -= 3;

                // println!("{enemy} at {ei}, {ej} takes 3 damage from {c} at {i2}, {j2}. It has {} health remaining", grid[ei][ej].1);

                if grid[ei][ej].1 <= 0 {
                    units.remove(&(ei, ej));

                    if grid[ei][ej].0 == 'E' {
                        elves -= 1;
                    } else if grid[ei][ej].0 == 'G' {
                        goblins -= 1;
                    }

                    grid[ei][ej] = ('.', 0);

                    // println!("{enemy} died during round {}", rounds);
                }
            }
        }

        rounds += 1;

        // println!("end round {rounds}");
    }

    print_grid(&grid);

    let score: i32 = units.iter().map(|&(i, j)| grid[i][j].1).sum::<i32>() * rounds;

    score
}

fn find_best_move(
    grid: &Vec<Vec<(char, i32)>>,
    i: usize,
    j: usize,
    target: char,
) -> (usize, usize) {
    let mut nodes: VecDeque<(u32, usize, usize, usize, usize)> = VecDeque::new();

    for (start_i, start_j) in [(i - 1, j), (i, j - 1), (i, j + 1), (i + 1, j)] {
        if grid[start_i][start_j].0 == target {
            return (i, j);
        } else if grid[start_i][start_j].0 == '.' {
            nodes.push_back((1, start_i, start_j, start_i, start_j));
        }
    }

    let mut best_move: (u32, usize, usize, usize, usize) =
        ((grid.len() * grid[0].len()) as u32, i, j, i, j);

    let mut visited: HashMap<(usize, usize), (u32, usize, usize)> = HashMap::new();

    while nodes.len() > 0 {
        let (steps, i, j, start_i, start_j) = nodes.pop_back().unwrap();

        visited.insert((i, j), (steps, start_i, start_j));

        if steps > best_move.0 {
            continue;
        }

        for (i2, j2) in [(i - 1, j), (i, j - 1), (i, j + 1), (i + 1, j)] {
            let steps2: u32 = steps + 1;

            if grid[i2][j2].0 == '.'
                && (!visited.contains_key(&(i2, j2))
                    || (steps2, start_i, start_j) < visited[&(i2, j2)])
            {
                visited.insert((i2, j2), (steps2, start_i, start_j));
                nodes.push_back((steps2, i2, j2, start_i, start_j));
            } else if grid[i2][j2].0 == target {
                if (steps, i, j, start_i, start_j) < best_move {
                    // println!("{:?} is better than {:?}", (steps, i, j, start_i, start_j), best_move);
                    best_move = (steps, i, j, start_i, start_j);
                }
            }
        }
    }

    (best_move.3, best_move.4)
}

fn print_grid(grid: &Vec<Vec<(char, i32)>>) {
    let mut output_string: String = String::new();

    for row in grid {
        let mut suffix: String = String::new();
        for &(c, h) in row {
            output_string.push(c);

            if c == 'E' || c == 'G' {
                suffix = format!("{suffix} {c}({h}),");
            }
        }
        output_string = format!("{output_string}    {suffix}\n");
    }

    println!("{output_string}");
}
