use crate::helper::read_data;
use regex::Regex;
use std::collections::HashMap;
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day20.txt");

    let particles: Vec<Vec<i64>> = get_particles(&data);

    let (p1, _): (usize, &Vec<i64>) = particles
        .iter()
        .enumerate()
        .min_by_key(|(_, &ref particle)| particle[6].abs() + particle[7].abs() + particle[8].abs())
        .unwrap();

    let p2: usize = simulate_particles(&particles);

    println!("{p1}\n{p2}");
}

fn get_particles(data: &String) -> Vec<Vec<i64>> {
    let mut particles: Vec<Vec<i64>> = Vec::new();

    let re: Regex = Regex::new(
        r"p=<(-?\d+),(-?\d+),(-?\d+)>, v=<(-?\d+),(-?\d+),(-?\d+)>, a=<(-?\d+),(-?\d+),(-?\d+)>",
    )
    .unwrap();

    for line in data.lines() {
        let (_, nums): (&str, [&str; 9]) = re.captures(line.trim()).unwrap().extract();
        let nums: Vec<i64> = nums.iter().map(|&num| num.parse().unwrap()).collect();

        particles.push(nums);
    }

    particles
}

fn calculate_particle(particle: &Vec<i64>, t: i64) -> Vec<i64> {
    let mut position: Vec<i64> = vec![particle[0], particle[1], particle[2]];

    for i in 0..3 {
        position[i] += particle[3 + i] * t + t * (t + 1) / 2 * particle[6 + i];
    }

    position
}

fn simulate_particles(particles: &Vec<Vec<i64>>) -> usize {
    let mut active: Vec<bool> = vec![true; particles.len()];

    // Lucky guess on reasonable time limit
    for t in 0..1000 {
        let mut positions: HashMap<Vec<i64>, usize> = HashMap::new();

        for i in 0..particles.len() {
            if !active[i] {
                continue;
            }

            let xyz: Vec<i64> = calculate_particle(&particles[i], t);

            if let Some(ind) = positions.insert(xyz, i) {
                active[ind] = false;
                active[i] = false;
            }
        }
    }

    active.iter().filter(|&&b| b).count()
}
