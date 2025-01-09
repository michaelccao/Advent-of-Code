use crate::helper::read_data;
use md5::compute;
use std::collections::{HashMap, HashSet};
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day17.txt");

    let p1 = shortest_path(&data);
    let p2 = longest_path(&data).len();

    println!("{p1}\n{p2}");
}

fn open_doors(s: &String) -> Vec<bool> {
    let mut doors: Vec<bool> = Vec::new();

    for c in format!("{:x}", compute(s)).chars() {
        match c {
            'b' | 'c' | 'd' | 'e' | 'f' => doors.push(true),
            _ => doors.push(false),
        }

        if doors.len() == 4 {
            return doors;
        }
    }

    doors
}

fn shortest_path(pass: &String) -> String {
    let mut nodes: Vec<(i32, i32, String, Vec<bool>)> = Vec::new();

    nodes.push((0, 0, pass.clone(), open_doors(pass)));

    let mut visited: HashMap<(i32, i32, Vec<bool>), usize> = HashMap::new();

    let mut shortest: Option<String> = None;

    let moves: [(char, i32, i32); 4] = [('U', -1, 0), ('D', 1, 0), ('L', 0, -1), ('R', 0, 1)];

    while nodes.len() > 0 {
        let (i, j, pass, doors) = nodes.pop().unwrap();

        visited.insert((i, j, doors.clone()), pass.len());

        if i == 3 && j == 3 {
            shortest = Some(pass);
            continue;
        }

        if shortest.is_some() && pass.len() >= shortest.clone().unwrap().len() {
            continue;
        }

        for k in 0..moves.len() {
            let (m, di, dj) = moves[k];
            let pass2 = format!("{pass}{m}");
            let doors2 = open_doors(&pass2);

            if doors[k]
                && i + di >= 0
                && i + di < 4
                && j + dj >= 0
                && j + dj < 4
                && (!visited.contains_key(&(i + di, j + dj, doors2.clone()))
                    || visited[&(i + di, j + dj, doors2.clone())] > pass2.len())
            {
                nodes.push((i + di, j + dj, pass2, doors2));
            }
        }
    }

    if shortest.is_some() {
        return shortest.unwrap()[pass.len()..].to_string();
    } else {
        return "".to_string();
    }
}

fn longest_path(pass: &String) -> String {
    let mut nodes: Vec<(i32, i32, String, Vec<bool>)> = Vec::new();

    nodes.push((0, 0, pass.clone(), open_doors(pass)));

    let mut visited: HashSet<(i32, i32, String)> = HashSet::new();

    let mut longest: Option<String> = None;

    let moves: [(char, i32, i32); 4] = [('U', -1, 0), ('D', 1, 0), ('L', 0, -1), ('R', 0, 1)];

    while nodes.len() > 0 {
        let (i, j, pass, doors) = nodes.pop().unwrap();

        visited.insert((i, j, pass.clone()));

        if i == 3 && j == 3 {
            if longest.is_none() {
                longest = Some(pass);
            } else if pass.len() > longest.clone().unwrap().len() {
                longest = Some(pass);
            }
            continue;
        }

        for k in 0..moves.len() {
            let (m, di, dj) = moves[k];
            let pass2 = format!("{pass}{m}");
            let doors2 = open_doors(&pass2);

            if doors[k]
                && i + di >= 0
                && i + di < 4
                && j + dj >= 0
                && j + dj < 4
                && !visited.contains(&(i + di, j + dj, pass2.clone()))
            {
                nodes.push((i + di, j + dj, pass2, doors2));
            }
        }
    }

    if longest.is_some() {
        return longest.unwrap()[pass.len()..].to_string();
    } else {
        return "".to_string();
    }
}
