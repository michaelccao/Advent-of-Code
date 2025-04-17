use crate::helper::read_data;
use std::collections::HashSet;
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day25.txt");

    let p1: u32 = num_constellations(&data);

    println!("{p1}");
}

fn num_constellations(data: &String) -> u32 {
    let coords: Vec<Vec<i32>> = data
        .lines()
        .map(|line| {
            line.trim()
                .split(",")
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();
    let mut links: Vec<Vec<usize>> = vec![vec![]; coords.len()];

    for i in 0..coords.len() {
        for j in i + 1..coords.len() {
            let (a, b, c, d): (i32, i32, i32, i32) =
                (coords[i][0], coords[i][1], coords[i][2], coords[i][3]);
            let (a2, b2, c2, d2): (i32, i32, i32, i32) =
                (coords[j][0], coords[j][1], coords[j][2], coords[j][3]);

            if a.abs_diff(a2) + b.abs_diff(b2) + c.abs_diff(c2) + d.abs_diff(d2) <= 3 {
                links[i].push(j);
                links[j].push(i);
            }
        }
    }

    let mut unvisited: HashSet<usize> = (0..coords.len()).collect();
    let mut nodes: Vec<usize> = Vec::new();

    let mut constellations: u32 = 0;

    while unvisited.len() > 0 {
        let node: usize = if let Some(node) = nodes.pop() {
            node
        } else {
            constellations += 1;
            *unvisited.iter().next().unwrap()
        };

        unvisited.remove(&node);

        for link in &links[node] {
            if unvisited.contains(link) {
                unvisited.remove(link);
                nodes.push(*link);
            }
        }
    }

    constellations
}
