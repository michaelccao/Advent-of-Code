use crate::helper::read_data;
use std::collections::{HashMap, HashSet};
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day03.txt");

    let triangles: Vec<Vec<u32>> = data
        .lines()
        .map(|line| {
            line.trim()
                .split_whitespace()
                .map(|num| num.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect();

    let p1: usize = triangles
        .iter()
        .filter(|sides| {
            let mut sides: Vec<u32> = sides.to_owned().clone();
            sides.sort();

            sides[0] + sides[1] > sides[2]
        })
        .cloned()
        .collect::<Vec<_>>()
        .len();

    let mut triangles2: Vec<Vec<u32>> = Vec::new();

    for i in 0..triangles.len() / 3 {
        for j in 0..3 {
            let triangle: Vec<u32> = vec![
                triangles[3 * i][j],
                triangles[3 * i + 1][j],
                triangles[3 * i + 2][j],
            ];
            triangles2.push(triangle)
        }
    }

    let p2: usize = triangles2
        .iter()
        .filter(|sides| {
            let mut sides: Vec<u32> = sides.to_owned().clone();
            sides.sort();

            sides[0] + sides[1] > sides[2]
        })
        .cloned()
        .collect::<Vec<_>>()
        .len();

    println!("{p1}\n{p2}");
}
