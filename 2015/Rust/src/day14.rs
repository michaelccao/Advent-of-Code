use crate::helper::read_data;
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day14.txt");

    let reindeers: Vec<(u32, u32, u32)> = get_reindeers(&data);

    let p1: u32 = reindeers
        .iter()
        .map(|reindeer| calculate_distance(reindeer, 2503))
        .max()
        .unwrap();

    let p2: u32 = calculate_points(&reindeers, 2503);

    println!("{p1}\n{p2}");
}

fn get_reindeers(data: &String) -> Vec<(u32, u32, u32)> {
    data.lines()
        .map(|line| {
            let reindeer: Vec<&str> = line.split(' ').collect();
            let speed: u32 = reindeer[3].parse().unwrap();
            let fly_time: u32 = reindeer[6].parse().unwrap();
            let rest_time: u32 = reindeer[13].parse().unwrap();

            (speed, fly_time, rest_time)
        })
        .collect::<Vec<(u32, u32, u32)>>()
}

fn calculate_distance(reindeer: &(u32, u32, u32), time: u32) -> u32 {
    let (speed, fly, rest): (u32, u32, u32) = *reindeer;

    let cycles = time / (fly + rest);
    let remainder = time % (fly + rest);

    speed * cycles * fly + speed * if remainder > fly { fly } else { remainder }
}

fn calculate_points(reindeers: &Vec<(u32, u32, u32)>, time: u32) -> u32 {
    let mut points: Vec<u32> = vec![0; reindeers.len()];

    for t in 1..time + 1 {
        let max_dist: u32 = reindeers
            .iter()
            .map(|reindeer| calculate_distance(reindeer, t))
            .max()
            .unwrap();

        for i in 0..points.len() {
            if calculate_distance(&reindeers[i], t) == max_dist {
                points[i] += 1;
            }
        }
    }

    *points.iter().max().unwrap()
}
