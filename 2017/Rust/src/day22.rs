use crate::helper::read_data;
use std::collections::HashMap;

pub fn main() {
    let data: String = read_data("../Data/Day22.txt");

    let (infected, i_start, j_start) = get_grid(&data);

    let p1: u32 = count_infections(&infected, i_start, j_start);
    let p2: u32 = count_infections2(&infected, i_start, j_start);

    println!("{p1}\n{p2}");
}

fn get_grid(data: &String) -> (HashMap<(i32, i32), bool>, i32, i32) {
    let mut grid: HashMap<(i32, i32), bool> = HashMap::new();

    let mut i: i32 = 0;
    let mut j: i32 = 0;

    for line in data.lines() {
        j = 0;
        for c in line.trim().chars() {
            grid.insert((i, j), c == '#');
            j += 1;
        }
        i += 1;
    }

    (grid, i / 2, j / 2)
}

fn count_infections(infected: &HashMap<(i32, i32), bool>, mut i: i32, mut j: i32) -> u32 {
    let mut infected: HashMap<(i32, i32), bool> = infected.clone();

    let mut infections: u32 = 0;

    let heading: [(i32, i32); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];

    let mut current_heading: usize = 0;

    for _ in 0..10000 {
        if *infected.get(&(i, j)).unwrap_or(&false) {
            current_heading += 3;

            infected.insert((i, j), false);
        } else {
            current_heading += 1;

            infected.insert((i, j), true);
            infections += 1;
        }

        current_heading %= 4;

        let (di, dj) = heading[current_heading];

        i += di;
        j += dj;
    }

    infections
}

fn count_infections2(infected: &HashMap<(i32, i32), bool>, mut i: i32, mut j: i32) -> u32 {
    let mut nodes: HashMap<(i32, i32), u8> = HashMap::new();

    for (&key, &is_infected) in infected.iter() {
        nodes.insert(key, if is_infected { 2 } else { 0 });
    }

    let mut infections: u32 = 0;

    let heading: [(i32, i32); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];

    let mut current_heading: usize = 0;

    for _ in 0..10000000 {
        let node_state: u8 = *nodes.get(&(i, j)).unwrap_or(&0);

        match node_state {
            0 => current_heading += 1,
            2 => current_heading += 3,
            3 => current_heading += 2,
            _ => {}
        }

        nodes.insert((i, j), (node_state + 1) % 4);

        if node_state == 1 {
            infections += 1;
        }

        current_heading %= 4;

        let (di, dj) = heading[current_heading];

        i += di;
        j += dj;
    }

    infections
}
