use crate::helper::read_data;
use regex::Regex;
use std::collections::{HashSet, VecDeque};

pub fn main() {
    let data: String = read_data("../Data/Day22.txt");

    let steps: Vec<(bool, i64, i64, i64, i64, i64, i64)> = get_steps(&data);

    let p1: usize = execute_steps(&steps);

    println!("{p1}");

    let p2: i64 = execute_steps2(&steps);

    println!("{p2}");
}

fn get_steps(data: &String) -> Vec<(bool, i64, i64, i64, i64, i64, i64)> {
    let mut steps: Vec<(bool, i64, i64, i64, i64, i64, i64)> = Vec::new();

    let re: Regex =
        Regex::new(r"(on|off) x=(-?\d+)..(-?\d+),y=(-?\d+)..(-?\d+),z=(-?\d+)..(-?\d+)").unwrap();

    for line in data.lines() {
        for (_, [toggle, x0, x1, y0, y1, z0, z1]) in re.captures_iter(line).map(|c| c.extract()) {
            let toggle: bool = toggle == "on";
            let x0: i64 = x0.parse().unwrap();
            let x1: i64 = x1.parse().unwrap();
            let y0: i64 = y0.parse().unwrap();
            let y1: i64 = y1.parse().unwrap();
            let z0: i64 = z0.parse().unwrap();
            let z1: i64 = z1.parse().unwrap();

            steps.push((toggle, x0, x1, y0, y1, z0, z1));
        }
    }

    steps
}

fn execute_steps(steps: &Vec<(bool, i64, i64, i64, i64, i64, i64)>) -> usize {
    let mut cubes: HashSet<(i64, i64, i64)> = HashSet::new();

    for &(toggle, x0, x1, y0, y1, z0, z1) in steps {
        for x in x0.max(-50)..x1.min(50) + 1 {
            for y in y0.max(-50)..y1.min(50) + 1 {
                for z in z0.max(-50)..z1.min(50) + 1 {
                    if toggle {
                        cubes.insert((x, y, z));
                    } else {
                        cubes.remove(&(x, y, z));
                    }
                }
            }
        }
    }

    cubes.len()
}

fn execute_steps2(steps: &Vec<(bool, i64, i64, i64, i64, i64, i64)>) -> i64 {
    let mut cubes: HashSet<Cube> = HashSet::new();

    let mut to_insert: VecDeque<Cube> = VecDeque::new();

    let mut steps: VecDeque<(bool, i64, i64, i64, i64, i64, i64)> = steps.iter().cloned().collect();

    while steps.len() > 0 {
        let (toggle, x0, x1, y0, y1, z0, z1) = steps.pop_front().unwrap();

        let cube: Cube = Cube {
            x0,
            x1,
            y0,
            y1,
            z0,
            z1,
        };

        if toggle {
            to_insert.push_back(cube);
        } else {
            let mut cubes2: HashSet<Cube> = cubes.clone();

            for cube2 in &cubes {
                let inter_cube: Cube = cube2.intersection(&cube);
                if inter_cube.size() > 0 {
                    for sub_cube in cube2.remove_cube(&inter_cube) {
                        cubes2.insert(sub_cube);
                    }
                    cubes2.remove(cube2);
                }
            }

            cubes = cubes2;
        }

        while to_insert.len() > 0 {
            let cube: Cube = to_insert.pop_front().unwrap();

            let mut conflict: bool = false;

            for cube2 in &cubes {
                let inter_cube: Cube = cube.intersection(cube2);

                if inter_cube.size() > 0 {
                    conflict = true;
                    for sub_cube in cube.remove_cube(&inter_cube) {
                        to_insert.push_front(sub_cube);
                    }

                    break;
                }
            }

            if !conflict {
                cubes.insert(cube);
            }
        }
    }

    cubes.iter().fold(0, |acc, s| acc + s.size())
}

// Ok... technically not a cube
#[derive(Clone, Hash, Eq, PartialEq, Debug)]
struct Cube {
    x0: i64,
    x1: i64,
    y0: i64,
    y1: i64,
    z0: i64,
    z1: i64,
}

impl Cube {
    fn size(&self) -> i64 {
        (self.x1 - self.x0 + 1) * (self.y1 - self.y0 + 1) * (self.z1 - self.z0 + 1)
    }

    fn intersection(&self, other: &Cube) -> Cube {
        let mut c: Cube = Cube {
            x0: 0,
            x1: -1,
            y0: 0,
            y1: -1,
            z0: 0,
            z1: -1,
        };

        if self.x0 >= other.x0 && self.x0 <= other.x1 {
            c.x0 = self.x0;
        } else if other.x0 >= self.x0 && other.x0 <= self.x1 {
            c.x0 = other.x0
        }

        if self.x1 >= other.x0 && self.x1 <= other.x1 {
            c.x1 = self.x1;
        } else if other.x1 >= self.x0 && other.x1 <= self.x1 {
            c.x1 = other.x1
        }

        if self.y0 >= other.y0 && self.y0 <= other.y1 {
            c.y0 = self.y0;
        } else if other.y0 >= self.y0 && other.y0 <= self.y1 {
            c.y0 = other.y0
        }

        if self.y1 >= other.y0 && self.y1 <= other.y1 {
            c.y1 = self.y1;
        } else if other.y1 >= self.y0 && other.y1 <= self.y1 {
            c.y1 = other.y1
        }

        if self.z0 >= other.z0 && self.z0 <= other.z1 {
            c.z0 = self.z0;
        } else if other.z0 >= self.z0 && other.z0 <= self.z1 {
            c.z0 = other.z0
        }

        if self.z1 >= other.z0 && self.z1 <= other.z1 {
            c.z1 = self.z1;
        } else if other.z1 >= self.z0 && other.z1 <= self.z1 {
            c.z1 = other.z1
        }

        c
    }

    // Other cube is within this cube
    fn remove_cube(&self, other: &Cube) -> Vec<Cube> {
        let mut cubes: Vec<Cube> = Vec::new();

        if self.x0 < other.x0 {
            let mut cube: Cube = self.clone();
            cube.x1 = other.x0 - 1;

            cubes.push(cube);
        }

        if self.x1 > other.x1 {
            let mut cube: Cube = self.clone();
            cube.x0 = other.x1 + 1;

            cubes.push(cube);
        }

        if self.y0 < other.y0 {
            let mut cube: Cube = self.clone();
            cube.x0 = other.x0;
            cube.x1 = other.x1;
            cube.y1 = other.y0 - 1;

            cubes.push(cube);
        }

        if self.y1 > other.y1 {
            let mut cube: Cube = self.clone();
            cube.x0 = other.x0;
            cube.x1 = other.x1;
            cube.y0 = other.y1 + 1;

            cubes.push(cube);
        }

        if self.z0 < other.z0 {
            let mut cube: Cube = other.clone();
            cube.z0 = self.z0;
            cube.z1 = other.z0 - 1;

            cubes.push(cube);
        }

        if self.z1 > other.z1 {
            let mut cube: Cube = other.clone();
            cube.z0 = other.z1 + 1;
            cube.z1 = self.z1;

            cubes.push(cube);
        }

        cubes
    }
}
