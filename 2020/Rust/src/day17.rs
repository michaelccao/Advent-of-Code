use crate::helper::read_data;
use std::collections::HashMap;

pub fn main() {
    let data: String = read_data("../Data/Day17.txt");

    let cubes: Cubes = initialize_cubes(&data);
    let hypercubes: HyperCubes = initialize_hypercubes(&data);

    let p1: usize = cubes
        .mutate_n(6)
        .state
        .values()
        .filter(|&active| *active)
        .count();

    println!("{p1}");

    let p2: usize = hypercubes
        .mutate_n(6)
        .state
        .values()
        .filter(|&active| *active)
        .count();

    println!("{p2}");
}

fn initialize_cubes(data: &String) -> Cubes {
    let mut cubes: HashMap<(i32, i32, i32), bool> = HashMap::new();

    for (y, line) in data.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            cubes.insert((x as i32, y as i32, 0), c == '#');
        }
    }

    let cubes: Cubes = Cubes { state: cubes };

    cubes
}

fn initialize_hypercubes(data: &String) -> HyperCubes {
    let mut cubes: HashMap<(i32, i32, i32, i32), bool> = HashMap::new();

    for (y, line) in data.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            cubes.insert((x as i32, y as i32, 0, 0), c == '#');
        }
    }

    let cubes: HyperCubes = HyperCubes { state: cubes };

    cubes
}

#[derive(Clone)]
struct Cubes {
    state: HashMap<(i32, i32, i32), bool>,
}

impl Cubes {
    fn mutate(&self) -> Cubes {
        let mut active_neighbor_count: HashMap<(i32, i32, i32), u32> = HashMap::new();
        let mut state2: HashMap<(i32, i32, i32), bool> = self.state.clone();

        for (&(x, y, z), &active) in &self.state {
            if active {
                for x2 in x - 1..x + 2 {
                    for y2 in y - 1..y + 2 {
                        for z2 in z - 1..z + 2 {
                            if (x2, y2, z2) == (x, y, z) {
                                continue;
                            }

                            if let Some(count) = active_neighbor_count.get_mut(&(x2, y2, z2)) {
                                *count += 1;
                            } else {
                                active_neighbor_count.insert((x2, y2, z2), 1);
                            }
                        }
                    }
                }
            }
        }

        for (&coords, &count) in &active_neighbor_count {
            let active: bool = *self.state.get(&coords).unwrap_or(&false);

            if active && count != 2 && count != 3 {
                state2.insert(coords, false);
            } else if !active && count == 3 {
                state2.insert(coords, true);
            }
        }

        for (coords, &active) in &self.state {
            if active {
                let count: u32 = *active_neighbor_count.get(coords).unwrap_or(&0);
                if count != 2 && count != 3 {
                    state2.insert(*coords, false);
                }
            }
        }

        Cubes { state: state2 }
    }

    fn mutate_n(&self, n: u32) -> Cubes {
        let mut cubes: Cubes = self.clone();

        for _ in 0..n {
            cubes = cubes.mutate();
        }

        cubes
    }
}

#[derive(Clone)]
struct HyperCubes {
    state: HashMap<(i32, i32, i32, i32), bool>,
}

impl HyperCubes {
    fn mutate(&self) -> HyperCubes {
        let mut active_neighbor_count: HashMap<(i32, i32, i32, i32), u32> = HashMap::new();
        let mut state2: HashMap<(i32, i32, i32, i32), bool> = self.state.clone();

        for (&(x, y, z, w), &active) in &self.state {
            if active {
                for x2 in x - 1..x + 2 {
                    for y2 in y - 1..y + 2 {
                        for z2 in z - 1..z + 2 {
                            for w2 in w - 1..w + 2 {
                                if (x2, y2, z2, w2) == (x, y, z, w) {
                                    continue;
                                }

                                if let Some(count) =
                                    active_neighbor_count.get_mut(&(x2, y2, z2, w2))
                                {
                                    *count += 1;
                                } else {
                                    active_neighbor_count.insert((x2, y2, z2, w2), 1);
                                }
                            }
                        }
                    }
                }
            }
        }

        for (&coords, &count) in &active_neighbor_count {
            let active: bool = *self.state.get(&coords).unwrap_or(&false);

            if active && count != 2 && count != 3 {
                state2.insert(coords, false);
            } else if !active && count == 3 {
                state2.insert(coords, true);
            }
        }

        for (coords, &active) in &self.state {
            if active {
                let count: u32 = *active_neighbor_count.get(coords).unwrap_or(&0);
                if count != 2 && count != 3 {
                    state2.insert(*coords, false);
                }
            }
        }

        HyperCubes { state: state2 }
    }

    fn mutate_n(&self, n: u32) -> HyperCubes {
        let mut cubes: HyperCubes = self.clone();

        for _ in 0..n {
            cubes = cubes.mutate();
        }

        cubes
    }
}
