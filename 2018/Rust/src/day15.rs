use crate::helper::read_data;
use std::collections::{HashMap, VecDeque};

pub fn main() {
    let data: String = read_data("../Data/Day15.txt");

    let (grid, units) = get_grid(&data);

    let p1: i32 = resolve_fight(&grid, &units);

    println!("{p1}");

    let p2: i32 = find_elf_win(&grid, &units);

    println!("{p2}");
}

fn get_grid(data: &String) -> (Vec<Vec<String>>, HashMap<String, (usize, usize, i32)>) {
    let mut grid: Vec<Vec<String>> = Vec::new();
    let mut units: HashMap<String, (usize, usize, i32)> = HashMap::new();

    let mut unit_count: HashMap<char, u8> = HashMap::from([('E', 0), ('G', 0)]);

    for (i, line) in data.lines().enumerate() {
        let line = line.split_whitespace().next().unwrap();
        let mut row: Vec<String> = Vec::new();

        for (j, c) in line.chars().enumerate() {
            if c == 'G' || c == 'E' {
                let callsign: String = format!("{c}{}", unit_count[&c]);
                row.push(callsign.clone());
                units.insert(callsign, (i, j, 200));

                *unit_count.get_mut(&c).unwrap() += 1;
            } else {
                row.push(format!("{c}"));
            }
        }

        grid.push(row);
    }

    (grid, units)
}

fn resolve_fight(grid: &Vec<Vec<String>>, units: &HashMap<String, (usize, usize, i32)>) -> i32 {
    let mut grid: Vec<Vec<String>> = grid.clone();
    let mut units: HashMap<String, (usize, usize, i32)> = units.clone();

    let mut elves: u8 = 0;
    let mut goblins: u8 = 0;

    for callsign in units.keys() {
        let unit_type: char = callsign.chars().next().unwrap();

        if unit_type == 'E' {
            elves += 1;
        } else if unit_type == 'G' {
            goblins += 1;
        }
    }

    let mut rounds: i32 = 0;

    'outer: loop {
        let mut turn_order: Vec<String> = units.keys().cloned().collect();

        turn_order.sort_by_key(|callsign| units[callsign]);

        // println!("start round {rounds}");

        for callsign in turn_order {
            let c: char = callsign.chars().next().unwrap();

            // Unit may be dead from previous units' actions
            if !units.contains_key(&callsign) {
                continue;
            }

            let (i, j, hp) = units[&callsign];

            // This is where our bug occurred. A unit can kill a unit that hasn't moved
            // A later unit can then move into dead unit's space
            // It then moves again due to being mistaken as the dead unit
            // The occasional extra turn speeds up combat, creating our off-by-one error

            // Main reason this bug was so hard to uncover was using a bad data structure
            // to keep track of our units

            // BAD CODE HERE
            // if c == '.' {
            //     continue;
            // }

            if (c == 'G' && elves == 0) || (c == 'E' && goblins == 0) {
                break 'outer;
            }

            let enemy: char = if c == 'E' { 'G' } else { 'E' };

            // Move
            let (i2, j2) = find_best_move(&grid, i, j, enemy);

            grid[i][j] = ".".to_string();
            grid[i2][j2] = callsign.clone();

            units.insert(callsign.clone(), (i2, j2, hp));

            // Attack
            let mut weakest_enemy: (i32, usize, usize, String) =
                (201, i2, j2, grid[i2][j2].clone());

            for (ei, ej) in [(i2 - 1, j2), (i2, j2 - 1), (i2, j2 + 1), (i2 + 1, j2)] {
                let callsign: String = grid[ei][ej].clone();
                let unit_type = callsign.chars().next().unwrap();

                if unit_type == enemy {
                    let (_, _, ehp) = units[&callsign];

                    if (ehp, ei, ej, callsign.clone()) < weakest_enemy {
                        weakest_enemy = (ehp, ei, ej, callsign.clone());
                    }
                }
            }

            if weakest_enemy.0 <= 200 {
                let (ei, ej, ehp) = units[&weakest_enemy.3];

                if ehp > 3 {
                    units.insert(weakest_enemy.3, (ei, ej, ehp - 3));
                } else {
                    units.remove(&weakest_enemy.3);
                    grid[ei][ej] = ".".to_string();

                    if enemy == 'E' {
                        elves -= 1;
                    } else if enemy == 'G' {
                        goblins -= 1;
                    }
                }
            }
        }

        rounds += 1;
    }

    let score: i32 = units.values().map(|&(_, _, hp)| hp).sum::<i32>() * rounds;

    score
}

fn find_elf_win(grid: &Vec<Vec<String>>, units: &HashMap<String, (usize, usize, i32)>) -> i32 {
    let mut goblins: u8 = 0;

    let mut elf_power: i32 = 3;

    for callsign in units.keys() {
        let unit_type: char = callsign.chars().next().unwrap();

        if unit_type == 'G' {
            goblins += 1;
        }
    }

    'elf_power: loop {
        let mut rounds: i32 = 0;

        let mut grid2: Vec<Vec<String>> = grid.clone();
        let mut units2: HashMap<String, (usize, usize, i32)> = units.clone();

        let mut goblins2: u8 = goblins;

        elf_power += 1;

        'outer: loop {
            let mut turn_order: Vec<String> = units2.keys().cloned().collect();

            turn_order.sort_by_key(|callsign| units2[callsign]);

            for callsign in turn_order {
                let c: char = callsign.chars().next().unwrap();

                if !units2.contains_key(&callsign) {
                    continue;
                }

                let (i, j, hp) = units2[&callsign];

                if c == 'E' && goblins2 == 0 {
                    break 'outer;
                }

                let enemy: char = if c == 'E' { 'G' } else { 'E' };

                // Move
                let (i2, j2) = find_best_move(&grid2, i, j, enemy);

                grid2[i][j] = ".".to_string();
                grid2[i2][j2] = callsign.clone();

                units2.insert(callsign.clone(), (i2, j2, hp));

                // Attack
                let mut weakest_enemy: (i32, usize, usize, String) =
                    (201, i2, j2, grid2[i2][j2].clone());

                for (ei, ej) in [(i2 - 1, j2), (i2, j2 - 1), (i2, j2 + 1), (i2 + 1, j2)] {
                    let callsign: String = grid2[ei][ej].clone();
                    let unit_type = callsign.chars().next().unwrap();

                    if unit_type == enemy {
                        let (_, _, ehp) = units2[&callsign];

                        if (ehp, ei, ej, callsign.clone()) < weakest_enemy {
                            weakest_enemy = (ehp, ei, ej, callsign.clone());
                        }
                    }
                }

                if weakest_enemy.0 <= 200 {
                    let (ei, ej, ehp) = units2[&weakest_enemy.3];

                    let power: i32 = if enemy == 'E' {3} else {elf_power};

                    if ehp > power {
                        units2.insert(weakest_enemy.3, (ei, ej, ehp - power));
                    } else {
                        units2.remove(&weakest_enemy.3);
                        grid2[ei][ej] = ".".to_string();

                        if enemy == 'E' {
                            continue 'elf_power;
                        } else if enemy == 'G' {
                            goblins2 -= 1;
                        }
                    }
                }
            }

            rounds += 1;
        }
        return units2.values().map(|&(_, _, hp)| hp).sum::<i32>() * rounds;

    }
    
}

fn find_best_move(grid: &Vec<Vec<String>>, i: usize, j: usize, target: char) -> (usize, usize) {
    let mut nodes: VecDeque<(u32, usize, usize, usize, usize)> = VecDeque::new();

    for (start_i, start_j) in [(i - 1, j), (i, j - 1), (i, j + 1), (i + 1, j)] {
        if grid[start_i][start_j].chars().next().unwrap() == target {
            return (i, j);
        } else if grid[start_i][start_j].chars().next().unwrap() == '.' {
            nodes.push_back((1, start_i, start_j, start_i, start_j));
        }
    }

    let mut best_move: (u32, usize, usize, usize, usize) =
        ((grid.len() * grid[0].len()) as u32, i, j, i, j);

    let mut visited: HashMap<(usize, usize), (u32, usize, usize)> = HashMap::new();

    while nodes.len() > 0 {
        let (steps, i, j, start_i, start_j) = nodes.pop_front().unwrap();

        visited.insert((i, j), (steps, start_i, start_j));

        if steps > best_move.0 {
            continue;
        }

        for (i2, j2) in [(i - 1, j), (i, j - 1), (i, j + 1), (i + 1, j)] {
            let steps2: u32 = steps + 1;

            if grid[i2][j2].chars().next().unwrap() == '.'
                && (!visited.contains_key(&(i2, j2))
                    || (steps2, start_i, start_j) < visited[&(i2, j2)])
            {
                visited.insert((i2, j2), (steps2, start_i, start_j));
                nodes.push_back((steps2, i2, j2, start_i, start_j));
            } else if grid[i2][j2].chars().next().unwrap() == target {
                if (steps, i, j, start_i, start_j) < best_move {
                    // println!("{:?} is better than {:?}", (steps, i, j, start_i, start_j), best_move);
                    best_move = (steps, i, j, start_i, start_j);
                }
            }
        }
    }

    (best_move.3, best_move.4)
}

fn print_grid(grid: &Vec<Vec<String>>, units: &HashMap<String, (usize, usize, i32)>) {
    let mut output_string: String = String::new();

    for row in grid {
        let mut suffix: String = String::new();
        for s in row {
            let c = s.chars().next().unwrap();

            output_string.push(c);

            if c == 'E' || c == 'G' {
                let h = units[s].2;
                suffix = format!("{suffix} {s}({h}),");
            }
        }
        output_string = format!("{output_string}    {suffix}\n");
    }
    println!("\n");
    println!("{output_string}");
}
