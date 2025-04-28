use crate::helper::read_data;
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

pub fn main() {
    let data: String = read_data("../Data/Day10.txt");

    let asteroids: HashSet<(i32, i32)> = get_asteroids(&data);

    let (monitoring_station, p1) = best_monitoring_station(&asteroids);

    println!("{p1:?}");

    let p2: i32 = vaporize_asteroids(&asteroids, monitoring_station);

    println!("{p2}");
}

fn get_asteroids(data: &String) -> HashSet<(i32, i32)> {
    let mut asteroids: HashSet<(i32, i32)> = HashSet::new();

    let mut y: i32 = 0;

    for line in data.lines() {
        let mut x = 0;
        for c in line.chars() {
            if c == '#' {
                asteroids.insert((x, y));
            }
            x += 1
        }
        y += 1;
    }

    asteroids
}

fn vaporize_asteroids(asteroids: &HashSet<(i32, i32)>, station: (i32, i32)) -> i32 {
    let mut targets: HashMap<LaserAngle, Vec<(i32, i32)>> = HashMap::new();

    let (sx, sy) = station;

    for &(x, y) in asteroids {
        if x == sx && y == sy {
            continue;
        }
        let (dx, dy) = reduce((x - sx, y - sy));
        let laser_angle: LaserAngle = LaserAngle { x: dx, y: dy };

        if let Some(asteroids) = targets.get_mut(&laser_angle) {
            asteroids.push((x, y));
        } else {
            targets.insert(laser_angle, vec![(x, y)]);
        }
    }

    for asteroids in targets.values_mut() {
        asteroids.sort_by_key(|&(x, y)| -x.abs() + -y.abs())
    }

    let mut angles: Vec<LaserAngle> = targets.keys().cloned().collect();
    angles.sort();

    let mut target_angle: usize = 0;

    let mut destroyed: u32 = 0;

    while destroyed < 200 {
        let laser_angle = &angles[target_angle];

        if let Some(asteroid) = targets.get_mut(laser_angle).unwrap().pop() {
            destroyed += 1;

            if destroyed == 200 {
                return asteroid.0 * 100 + asteroid.1;
            }
        }

        target_angle += 1;
        target_angle %= angles.len();
    }

    0
}

fn best_monitoring_station(asteroids: &HashSet<(i32, i32)>) -> ((i32, i32), usize) {
    let mut los: HashMap<(i32, i32), HashSet<(i32, i32)>> = HashMap::new();

    for combo in asteroids.iter().combinations(2) {
        let &a1 = combo[0];
        let &a2 = combo[1];

        if has_line_of_sight(asteroids, a1, a2) {
            if let Some(visible) = los.get_mut(&a1) {
                visible.insert(a2);
            } else {
                los.insert(a1, HashSet::from([a2]));
            }

            if let Some(visible) = los.get_mut(&a2) {
                visible.insert(a1);
            } else {
                los.insert(a2, HashSet::from([a1]));
            }
        }
    }

    let monitoring_station = los
        .iter()
        .max_by_key(|&(_, visible)| visible.len())
        .unwrap();

    (*monitoring_station.0, monitoring_station.1.len())
}

fn has_line_of_sight(
    asteroids: &HashSet<(i32, i32)>,
    asteroid_1: (i32, i32),
    asteroid_2: (i32, i32),
) -> bool {
    let line: (i32, i32) = (asteroid_2.0 - asteroid_1.0, asteroid_2.1 - asteroid_1.1);
    let step: (i32, i32) = reduce(line);

    let num_steps: i32 = if step.0 != 0 {
        line.0 / step.0
    } else {
        line.1 / step.1
    };

    for i in 1..num_steps {
        let x2: i32 = asteroid_1.0 + i * step.0;
        let y2: i32 = asteroid_1.1 + i * step.1;

        if asteroids.contains(&(x2, y2)) {
            return false;
        }
    }

    true
}

fn reduce(line: (i32, i32)) -> (i32, i32) {
    let (x, y) = line;

    if x == 0 {
        return (0, y / y.abs());
    } else if y == 0 {
        return (x / x.abs(), 0);
    }

    for i in 0..x.abs() {
        let div: i32 = x.abs() - i;

        if x % div == 0 && y % div == 0 {
            return (x / div, y / div);
        }
    }

    (x, y)
}

#[derive(Debug, Hash, Clone)]
struct LaserAngle {
    x: i32,
    y: i32,
}

impl LaserAngle {
    fn quadrant(&self) -> u32 {
        match (self.x >= 0, self.y < 0) {
            (true, true) => 0,
            (true, false) => 1,
            (false, false) => 2,
            (false, true) => 3,
        }
    }
}

impl Ord for LaserAngle {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.quadrant() != other.quadrant() {
            return self.quadrant().cmp(&other.quadrant());
        } else {
            if self.y * other.x < self.x * other.y {
                return Ordering::Less;
            } else if self.y * other.x == self.x * other.y {
                return Ordering::Equal;
            } else {
                return Ordering::Greater;
            }
        }
    }
}

impl PartialOrd for LaserAngle {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for LaserAngle {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for LaserAngle {}
