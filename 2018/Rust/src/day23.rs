use crate::helper::read_data;
use std::collections::{HashMap, HashSet};
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day23.txt");

    let (robots, strongest) = get_robots(&data);

    let p1 = robots_in_range(&robots, strongest);

    println!("{p1}");

    // Almost certain magic coordinate is at edge of a robot's range
    // But there are a ton of edge coordinates, scales with r^2
    // and our radii are large

    // Manhanttan distance sorta decouples x,y,z
    // Can we take advantage of this?

    // A point can be found in range of two robots
    // if their distance from each other is less than or
    // equal to the sum of their radii

    // In our target cluster, all pairs of robots are at
    // most the sum of their radii apart

    // Mathematically, what is
    // |x| + |y| + |z| s.t. x,y,z maximizes
    // Sum_i(Heaviside(ri - |x-xi| + |y-yi| + |z-zi|))

}

fn get_robots(data: &String) -> (Vec<(i32, i32, i32, i32)>, usize) {
    let mut robots: Vec<(i32, i32, i32, i32)> = Vec::new();

    let mut largest_radius: i32 = 0;
    let mut strongest_robot: usize = 0;

    for (i, line) in data.lines().enumerate() {
        let open_bracket = line.find("<").unwrap();
        let close_bracket = line.find(">").unwrap();

        let mut coords = line[open_bracket+1..close_bracket].split(",");
        let x: i32 = coords.next().unwrap().parse().unwrap();
        let y: i32 = coords.next().unwrap().parse().unwrap();
        let z: i32 = coords.next().unwrap().parse().unwrap();

        let radius:i32 = line.split("r=").last().unwrap().parse().unwrap();

        robots.push((x, y, z, radius));

        if radius > largest_radius {
            largest_radius = radius;
            strongest_robot = i;
        }

    }

    (robots, strongest_robot)
}

fn robots_in_range(robots: &Vec<(i32, i32, i32, i32)>, robot: usize) -> u32 {

    let (x0, y0, z0, r) = robots[robot];

    let mut in_radius: u32 = 0;

    for &(x, y, z, _) in robots {

        if x.abs_diff(x0) + y.abs_diff(y0) + z.abs_diff(z0) <= r as u32 {
            in_radius += 1;
        }
    }

    in_radius

}