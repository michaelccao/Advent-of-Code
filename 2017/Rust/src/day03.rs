use crate::helper::read_data;
use std::collections::HashMap;

pub fn main() {
    let data: String = read_data("../Data/Day03.txt");
    let total_steps: i32 = data.parse::<i32>().unwrap() - 1;

    let p1: i32 = spiral(total_steps);
    let p2: i32 = spiral2(total_steps + 1);

    println!("{p1}\n{p2}");
}

fn spiral(total_steps: i32) -> i32 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;

    let v: [(i32, i32); 4] = [(0, 1), (-1, 0), (0, -1), (1, 0)];

    let mut heading: usize = 0;
    let mut steps: i32 = 1;
    let mut steps_left: i32 = 1;

    for _ in 0..total_steps {
        let (di, dj) = v[heading];

        i += di;
        j += dj;

        steps_left -= 1;

        if steps_left == 0 {
            heading = (heading + 1) % v.len();
            if heading % 2 == 0 {
                steps += 1;
            }
            steps_left = steps;
        }
    }

    i.abs() + j.abs()
}

fn spiral2(threshold: i32) -> i32 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;

    let v: [(i32, i32); 4] = [(0, 1), (-1, 0), (0, -1), (1, 0)];

    let neighbors: [(i32, i32); 8] = [
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let mut spiral_vals: HashMap<(i32, i32), i32> = HashMap::new();
    spiral_vals.insert((0, 0), 1);

    let mut heading: usize = 0;
    let mut steps: i32 = 1;
    let mut steps_left: i32 = 1;

    loop {
        let (di, dj) = v[heading];

        i += di;
        j += dj;

        let mut val = 0;

        for (di, dj) in neighbors {
            if let Some(val2) = spiral_vals.get(&(i + di, j + dj)) {
                val += val2;
            }
        }

        if val > threshold {
            return val;
        }

        spiral_vals.insert((i, j), val);

        steps_left -= 1;

        if steps_left == 0 {
            heading = (heading + 1) % v.len();
            if heading % 2 == 0 {
                steps += 1;
            }
            steps_left = steps;
        }
    }
}
