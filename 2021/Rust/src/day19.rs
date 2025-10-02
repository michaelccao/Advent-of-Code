use crate::helper::read_data;
use std::collections::HashSet;

pub fn main() {
    let data: String = read_data("../Data/Day19.txt");

    let beacons: Vec<Vec<Vec<i32>>> = get_beacons(&data);

    let (beacons, p2) = find_beacons(&beacons);

    let p1: usize = beacons.len();

    println!("{p1}\n{p2}")
}

fn get_beacons(data: &String) -> Vec<Vec<Vec<i32>>> {
    let mut beacons: Vec<Vec<Vec<i32>>> = Vec::new();
    let mut row: Vec<Vec<i32>> = Vec::new();

    for line in data.lines() {
        if line.starts_with("---") {
            row = Vec::new();
        } else if line.len() == 0 {
            beacons.push(row.clone());
        } else {
            let coords: Vec<i32> = line.split(",").map(|num| num.parse().unwrap()).collect();
            row.push(coords);
        }
    }

    beacons.push(row);

    beacons
}

fn match_beacons(b1: &Vec<Vec<i32>>, b2: &Vec<Vec<i32>>) -> (Vec<Vec<i32>>, (i32, i32, i32)) {
    let b1_set: HashSet<Vec<i32>> = b1.iter().cloned().collect();

    for i in 0..b1.len() {
        let (x1, y1, z1) = (b1[i][0], b1[i][1], b1[i][2]);
        for j in 0..b2.len() {
            let (x2, y2, z2) = (b2[j][0], b2[j][1], b2[j][2]);

            let (dx, dy, dz) = (x2 - x1, y2 - y1, z2 - z1);

            let b2_set: HashSet<Vec<i32>> = b2
                .iter()
                .map(|beacon| vec![beacon[0] - dx, beacon[1] - dy, beacon[2] - dz])
                .collect();

            let beacons: HashSet<Vec<i32>> = b1_set.intersection(&b2_set).cloned().collect();

            if beacons.len() >= 12 {
                let b2_true: Vec<Vec<i32>> = b2_set.iter().cloned().collect();
                return (b2_true, (dx, dy, dz));
            }
        }
    }

    return (Vec::new(), (0, 0, 0));
}

fn transform_beacons(beacons: &Vec<Vec<i32>>) -> Vec<Vec<Vec<i32>>> {
    let axes: [(i32, i32, i32); 24] = [
        (1, 2, 3),
        (1, -2, -3),
        (1, 3, -2),
        (1, -3, 2),
        (-1, 2, -3),
        (-1, -2, 3),
        (-1, 3, 2),
        (-1, -3, -2),
        (2, 1, -3),
        (2, -1, 3),
        (2, 3, 1),
        (2, -3, -1),
        (-2, 1, 3),
        (-2, -1, -3),
        (-2, 3, -1),
        (-2, -3, 1),
        (3, 1, 2),
        (3, -1, -2),
        (3, 2, -1),
        (3, -2, 1),
        (-3, 1, -2),
        (-3, -1, 2),
        (-3, 2, 1),
        (-3, -2, -1),
    ];

    let mut transforms: Vec<Vec<Vec<i32>>> = Vec::new();

    for (i, j, k) in axes {
        let i_axis: usize = i.abs() as usize - 1;
        let j_axis: usize = j.abs() as usize - 1;
        let k_axis: usize = k.abs() as usize - 1;

        let i_sign: i32 = i.signum();
        let j_sign: i32 = j.signum();
        let k_sign: i32 = k.signum();

        let mut beacons2: Vec<Vec<i32>> = Vec::new();

        for beacon in beacons {
            beacons2.push(vec![
                i_sign * beacon[i_axis],
                j_sign * beacon[j_axis],
                k_sign * beacon[k_axis],
            ]);
        }

        transforms.push(beacons2);
    }

    transforms
}

fn find_beacons(beacons: &Vec<Vec<Vec<i32>>>) -> (HashSet<Vec<i32>>, i32) {
    let mut true_beacons: Vec<Vec<Vec<i32>>> = Vec::new();
    let mut unresolved: Vec<Vec<Vec<i32>>> = beacons.clone();

    let mut scanners: Vec<Vec<i32>> = vec![vec![0, 0, 0]];

    true_beacons.push(beacons[0].clone());
    unresolved.remove(0);

    for i in 0..beacons.len() {
        let b1: Vec<Vec<i32>> = true_beacons[i].clone();

        let mut unresolved2: Vec<Vec<Vec<i32>>> = Vec::new();

        for j in 0..unresolved.len() {
            let b2: &Vec<Vec<i32>> = &unresolved[j];

            let mut found_match: bool = false;

            for b2_t in transform_beacons(b2) {
                let (b2_true, (dx, dy, dz)) = match_beacons(&b1, &b2_t);

                if b2_true.len() > 0 {
                    found_match = true;
                    true_beacons.push(b2_true);

                    scanners.push(vec![dx, dy, dz]);
                    break;
                }
            }

            if !found_match {
                unresolved2.push(b2.clone());
            }
        }

        unresolved = unresolved2;
    }

    let mut max_dist: i32 = 0;

    for i in 0..scanners.len() {
        let s0: &Vec<i32> = &scanners[i];
        for j in i + 1..scanners.len() {
            let s1: &Vec<i32> = &scanners[j];

            max_dist =
                max_dist.max((s0[0] - s1[0]).abs() + (s0[1] - s1[1]).abs() + (s0[2] - s1[2]).abs());
        }
    }

    (true_beacons.iter().flatten().cloned().collect(), max_dist)
}
