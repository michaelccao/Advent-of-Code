use crate::helper::read_data;
use num::integer::lcm;

pub fn main() {
    let data: String = read_data("../Data/Day12.txt");

    let moons: Vec<Moon> = get_moons(&data);

    let (p1, p2) = simulate_moons(&moons);

    println!("{p1}");
    println!("{p2}");
}

fn simulate_moons(moons: &Vec<Moon>) -> (i64, i64) {
    let mut moons: Vec<Moon> = moons.clone();

    let original_moons: Vec<Moon> = moons.clone();

    let mut p1: i64 = 0;
    let mut x_cycle: i64 = 0;
    let mut y_cycle: i64 = 0;
    let mut z_cycle: i64 = 0;

    for t in 1..1000000 {
        for i in 0..moons.len() {
            for j in i + 1..moons.len() {
                let (left, right) = moons.split_at_mut(j);

                left[i].apply_gravity(&mut right[0]);
            }
        }

        for i in 0..moons.len() {
            moons[i].update_position();
        }

        if t == 1000 {
            p1 = moons.iter().map(|moon| moon.energy()).sum();
        }

        let mut x_match: bool = true;
        let mut y_match: bool = true;
        let mut z_match: bool = true;

        // X, Y, Z are decoupled simulations, find where they each cycle and then find LCM
        for i in 0..moons.len() {
            if moons[i].x != original_moons[i].x || moons[i].vx != 0 {
                x_match = false;
            }

            if moons[i].y != original_moons[i].y || moons[i].vy != 0 {
                y_match = false;
            }

            if moons[i].z != original_moons[i].z || moons[i].vz != 0 {
                z_match = false;
            }
        }

        if x_match && x_cycle == 0 {
            x_cycle = t
        }
        if y_match && y_cycle == 0 {
            y_cycle = t
        }
        if z_match && z_cycle == 0 {
            z_cycle = t
        }
    }

    let full_cycle: i64 = lcm(lcm(x_cycle, y_cycle), z_cycle);

    (p1, full_cycle)
}

fn get_moons(data: &String) -> Vec<Moon> {
    let mut moons: Vec<Moon> = Vec::new();

    for line in data.lines() {
        let coords: Vec<i64> = line
            .trim_end_matches(">")
            .split(", ")
            .map(|coord| coord.split("=").last().unwrap().parse().unwrap())
            .collect();
        let moon: Moon = Moon {
            x: coords[0],
            y: coords[1],
            z: coords[2],
            vx: 0,
            vy: 0,
            vz: 0,
        };

        moons.push(moon);
    }

    moons
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Moon {
    x: i64,
    y: i64,
    z: i64,
    vx: i64,
    vy: i64,
    vz: i64,
}

impl Moon {
    fn apply_gravity(&mut self, other: &mut Self) {
        if self.x < other.x {
            self.vx += 1;
            other.vx -= 1;
        } else if self.x > other.x {
            self.vx -= 1;
            other.vx += 1;
        }

        if self.y < other.y {
            self.vy += 1;
            other.vy -= 1;
        } else if self.y > other.y {
            self.vy -= 1;
            other.vy += 1;
        }

        if self.z < other.z {
            self.vz += 1;
            other.vz -= 1;
        } else if self.z > other.z {
            self.vz -= 1;
            other.vz += 1;
        }
    }

    fn update_position(&mut self) {
        self.x += self.vx;
        self.y += self.vy;
        self.z += self.vz;
    }

    fn energy(&self) -> i64 {
        let potential_energy: i64 = self.x.abs() + self.y.abs() + self.z.abs();
        let kinetic_energy: i64 = self.vx.abs() + self.vy.abs() + self.vz.abs();

        potential_energy * kinetic_energy
    }
}
