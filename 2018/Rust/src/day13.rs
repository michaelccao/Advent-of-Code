use crate::helper::read_data;
use std::collections::{HashMap, HashSet};
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day13.txt");

    let (tracks, carts) = get_track(&data);

    let p1: (usize, usize) = run_carts(&tracks, &carts, true);
    let p2: (usize, usize) = run_carts(&tracks, &carts, false);

    println!("{p1:?}\n{p2:?}");
}

fn get_track(data: &String) -> (Vec<Vec<char>>, HashMap<(usize, usize), (u8, u8)>) {
    let mut tracks: Vec<Vec<char>> = Vec::new();
    let mut carts: HashMap<(usize, usize), (u8, u8)> = HashMap::new();

    for (i, line) in data.lines().enumerate() {
        let mut row: Vec<char> = Vec::new();

        for (j, c) in line.chars().enumerate() {
            if c == '>' {
                row.push('-');
                carts.insert((i, j), (0, 0));
            } else if c == 'v' {
                row.push('|');
                carts.insert((i, j), (1, 0));
            } else if c == '<' {
                row.push('-');
                carts.insert((i, j), (2, 0));
            } else if c == '^' {
                row.push('|');
                carts.insert((i, j), (3, 0));
            } else {
                row.push(c);
            }
        }

        tracks.push(row);
    }

    (tracks, carts)
}

fn run_carts(
    tracks: &Vec<Vec<char>>,
    carts: &HashMap<(usize, usize), (u8, u8)>,
    part1: bool,
) -> (usize, usize) {
    let mut carts: HashMap<(usize, usize), (u8, u8)> = carts.clone();

    let mut cart_coords: Vec<&(usize, usize)> = carts.keys().collect();
    cart_coords.sort();

    while cart_coords.len() > 1 {
        let mut collisions: HashSet<(usize, usize)> = HashSet::new();

        let mut carts2: HashMap<(usize, usize), (u8, u8)> = HashMap::new();

        let mut dests: HashMap<(usize, usize), u8> = HashMap::new();

        for &(i, j) in cart_coords {
            if collisions.contains(&(i, j)) {
                continue;
            }

            let &(heading, inter) = carts.get(&(i, j)).unwrap();

            let (i2, j2, heading2, inter2) = move_cart(tracks, i, j, heading, inter);

            if let Some(count) = dests.get_mut(&(i2, j2)) {
                *count += 1;
            } else {
                dests.insert((i2, j2), 1);
            }

            if carts.contains_key(&(i2, j2)) && (i, j) < (i2, j2) {
                if part1 {
                    return (j2, i2);
                }
                collisions.insert((i2, j2));
                continue;
            }

            if carts2.insert((i2, j2), (heading2, inter2)).is_some() {
                if part1 {
                    return (j2, i2);
                }
                carts2.remove(&(i2, j2));
            }
        }

        carts = carts2;
        cart_coords = carts.keys().collect();
        cart_coords.sort();
    }

    (cart_coords[0].1, cart_coords[0].0)
}

fn move_cart(
    tracks: &Vec<Vec<char>>,
    i: usize,
    j: usize,
    heading: u8,
    inter: u8,
) -> (usize, usize, u8, u8) {
    let mut i2: usize = i;
    let mut j2: usize = j;

    match heading {
        0 => j2 += 1,
        1 => i2 += 1,
        2 => j2 -= 1,
        3 => i2 -= 1,
        _ => {}
    }

    let mut heading2: u8 = heading;
    let mut inter2: u8 = inter;

    match (heading, tracks[i2][j2]) {
        (0, '\\') => heading2 = 1,
        (0, '/') => heading2 = 3,
        (1, '\\') => heading2 = 0,
        (1, '/') => heading2 = 2,
        (2, '\\') => heading2 = 3,
        (2, '/') => heading2 = 1,
        (3, '\\') => heading2 = 2,
        (3, '/') => heading2 = 0,
        (_, '+') => {
            heading2 = (heading + 3 + inter) % 4;
            inter2 = (inter + 1) % 3
        }
        _ => {}
    }

    (i2, j2, heading2, inter2)
}
