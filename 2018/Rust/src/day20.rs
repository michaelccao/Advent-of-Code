use crate::helper::read_data;
use std::collections::{HashMap, HashSet};
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day20.txt");

    let doors: HashMap<(i32, i32), HashSet<(i32, i32)>> = generate_map(&data);

    let (p1, p2) = furthest_room(&doors);

    println!("{p1}\n{p2}");
}

fn generate_map(data: &String) -> HashMap<(i32, i32), HashSet<(i32, i32)>> {
    let data: Vec<char> = data[1..data.len() - 1].chars().collect();

    let mut paths: Vec<((i32, i32), usize, usize)> = vec![((0, 0), 0, data.len())];

    let mut doors: HashMap<(i32, i32), HashSet<(i32, i32)>> = HashMap::new();
    let moves: HashMap<char, (i32, i32)> =
        HashMap::from([('N', (-1, 0)), ('W', (0, -1)), ('S', (1, 0)), ('E', (0, 1))]);

    let mut visited: HashSet<((i32, i32), usize)> = HashSet::new();

    while paths.len() > 0 {
        let (current_room, pointer, mut exit) = paths.pop().unwrap();
        if pointer >= data.len() {
            continue;
        }

        let c: char = data[pointer];

        if c == '|' {
            if visited.insert((current_room, exit)) {
                paths.push((current_room, exit, data.len()));
            }
        } else if c == '(' {
            let mut splits: Vec<usize> = vec![pointer + 1];

            let mut buffer: i32 = -1;
            for j in pointer + 1..data.len() {
                let c2: char = data[j];
                if c2 == '|' && buffer == -1 {
                    splits.push(j + 1);
                } else if c2 == '(' {
                    buffer -= 1;
                } else if c2 == ')' {
                    buffer += 1;
                }
                if buffer == 0 {
                    exit = j;

                    for &split in &splits {
                        if visited.insert((current_room, split)) {
                            paths.push((current_room, split, exit));
                        }
                    }
                    break;
                }
            }
        } else if c == ')' {
            if visited.insert((current_room, pointer + 1)) {
                paths.push((current_room, pointer + 1, data.len()));
            }
        } else {
            let (di, dj) = moves[&c];
            let next_room = (current_room.0 + di, current_room.1 + dj);

            if let Some(rooms) = doors.get_mut(&current_room) {
                rooms.insert(next_room);
            } else {
                doors.insert(current_room, HashSet::from([next_room]));
            }

            if let Some(rooms) = doors.get_mut(&next_room) {
                rooms.insert(current_room);
            } else {
                doors.insert(next_room, HashSet::from([current_room]));
            }

            if visited.insert((next_room, pointer + 1)) {
                paths.push((next_room, pointer + 1, exit));
            }
        }
    }

    doors
}

fn furthest_room(doors: &HashMap<(i32, i32), HashSet<(i32, i32)>>) -> (u32, usize) {
    let mut visited: HashMap<(i32, i32), u32> = HashMap::from([((0, 0), 0)]);

    let mut nodes: Vec<((i32, i32), u32)> = vec![((0, 0), 0)];

    while nodes.len() > 0 {
        let (current_room, steps) = nodes.pop().unwrap();

        for &next_room in &doors[&current_room] {
            if !visited.contains_key(&next_room) || steps + 1 < visited[&next_room] {
                visited.insert(next_room, steps + 1);
                nodes.push((next_room, steps + 1));
            }
        }
    }

    (
        *visited.values().max().unwrap(),
        visited.values().filter(|&&dist| dist >= 1000).count(),
    )
}
