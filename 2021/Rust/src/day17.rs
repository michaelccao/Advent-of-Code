use crate::helper::read_data;
use std::collections::{HashMap, HashSet};

pub fn main() {
    let data: String = read_data("../Data/Day17.txt");

    let (x_min, x_max, y_min, y_max) = get_target(&data);

    // Key idea: triangle numbers
    // x position is something like 3 + 2 + 1 + 0
    // y position is something like 3 + 2 + 1 + 0 - 1 - 2 - 3 - ....
    // y always returns to 0 if starting positive

    // Using a vx that stalls at target is optimal
    // because it gives "infinite" air time

    // So we just find vx*(vx+1) / 2 that's in target
    // Max vy is |y_min| - 1, any faster and we overshoot
    // Max height is just vy(vy+1)/2
    // Caveat: We assume that x has reached destination in vy time
    // So x_min >= |y_min|

    let p1 = (y_min.abs() - 1) * (y_min.abs()) / 2;

    println!("{p1}");

    // vx_min*(vx_min+1)/2 >= x_min
    // vx_max = x_max
    // y_max = |y_min| - 1
    // y_min = y_min

    let p2 = calculate_valid_velocities(x_min, x_max, y_min, y_max);

    println!("{p2}");

    // In retrospect, this problem is easily brute-forceable
}

fn get_target(data: &String) -> (i32, i32, i32, i32) {
    let mut xy = data.split(": ").last().unwrap().split(", ");

    let mut x = xy.next().unwrap().split("=").last().unwrap().split("..");
    let mut y = xy.next().unwrap().split("=").last().unwrap().split("..");

    let x_min: i32 = x.next().unwrap().parse().unwrap();
    let x_max: i32 = x.next().unwrap().parse().unwrap();

    let y_min: i32 = y.next().unwrap().parse().unwrap();
    let y_max: i32 = y.next().unwrap().parse().unwrap();

    (x_min, x_max, y_min, y_max)
}

fn calculate_valid_velocities(x_min: i32, x_max: i32, y_min: i32, y_max: i32) -> usize {
    let mut vx_min: i32 = 0;

    while vx_min * (vx_min + 1) / 2 < x_min {
        vx_min += 1;
    }

    let vx_max: i32 = x_max;

    let vy_min: i32 = y_min;
    let vy_max: i32 = y_min.abs() - 1;

    let mut valid_times: HashMap<i32, HashSet<i32>> = HashMap::new();

    for vy0 in vy_min..vy_max + 1 {
        // Jump to y = 0
        let mut t: i32 = if vy0 > 0 { 2 * vy0 + 1 } else { 0 };
        let mut vy: i32 = if vy0 > 0 { -vy0 - 1 } else { vy0 };
        let mut y: i32 = 0;

        while y >= y_min {
            t += 1;
            y += vy;
            vy -= 1;

            if y >= y_min && y <= y_max {
                if let Some(valid_vy) = valid_times.get_mut(&t) {
                    valid_vy.insert(vy0);
                } else {
                    valid_times.insert(t, HashSet::from([vy0]));
                }
            }
        }
    }

    let mut valid_v: HashSet<(i32, i32)> = HashSet::new();

    for vx in vx_min..vx_max + 1 {
        for (&t, vys) in &valid_times {
            let t: i32 = t.min(vx);
            let distance: i32 = vx * t - t * (t - 1) / 2;
            if distance >= x_min && distance <= x_max {
                for &vy in vys {
                    valid_v.insert((vx, vy));
                }
            }
        }
    }

    valid_v.len()
}
