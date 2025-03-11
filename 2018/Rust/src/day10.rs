use crate::helper::read_data;
use regex::Regex;
use std::collections::HashMap;
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day10.txt");

    let points: Vec<(i32, i32, i32, i32)> = get_points(&data);

    let (p1, p2) = get_message(&points);

    println!("{p1}\n{p2}");
}

fn get_points(data: &String) -> Vec<(i32, i32, i32, i32)> {
    let mut points: Vec<(i32, i32, i32, i32)> = Vec::new();
    let re: Regex =
        Regex::new(r"position=<([- ]?\d+), ([- ]?\d+)> velocity=<([- ]?\d+), ([- ]?\d+)>").unwrap();

    for line in data.lines() {
        let (_, [x, y, vx, vy]) = re.captures(line.trim()).unwrap().extract();

        let x: i32 = x.trim().parse().unwrap();
        let y: i32 = y.trim().parse().unwrap();
        let vx: i32 = vx.trim().parse().unwrap();
        let vy: i32 = vy.trim().parse().unwrap();

        points.push((x, y, vx, vy));
    }

    points
}

fn calculate_points(points: &Vec<(i32, i32, i32, i32)>, t: i32) -> Vec<(i32, i32)> {
    points
        .iter()
        .map(|&(x, y, vx, vy)| (x + vx * t, y + vy * t))
        .collect()
}

fn get_message(points: &Vec<(i32, i32, i32, i32)>) -> (String, i32) {
    let (_, t, _) = calculate_x_intercepts(points);
    let mut points: Vec<(i32, i32)> = calculate_points(points, t);

    let min_x: i32 = points.iter().map(|&xy| xy.0).min().unwrap();
    let min_y: i32 = points.iter().map(|&xy| xy.1).min().unwrap();

    points = points
        .iter()
        .map(|&(x, y)| (x - min_x, y - min_y))
        .collect();

    let max_x: i32 = points.iter().map(|&xy| xy.0).max().unwrap();
    let max_y: i32 = points.iter().map(|&xy| xy.1).max().unwrap();

    let mut message_arr: Vec<Vec<char>> = vec![vec![' '; max_x as usize + 1]; max_y as usize + 1];

    for (x, y) in points {
        message_arr[y as usize][x as usize] = '#';
    }

    let mut message: String = String::new();

    for line in message_arr {
        let line: String = line.iter().collect();
        message = format!("{}{}\n", message, line);
    }

    (message, t)
}

fn calculate_x_intercepts(points: &Vec<(i32, i32, i32, i32)>) -> (i32, i32, u32) {
    let mut x_intercepts: HashMap<(i32, i32), u32> = HashMap::new();

    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let (x1, _, vx1, _) = points[i];
            let (x2, _, vx2, _) = points[j];

            if vx1 != vx2 && (x2 - x1) % (vx1 - vx2) == 0 {
                let t: i32 = (x2 - x1) / (vx1 - vx2);
                let x_int: i32 = x1 + vx1 * t;

                if let Some(count) = x_intercepts.get_mut(&(x_int, t)) {
                    *count += 1
                } else {
                    x_intercepts.insert((x_int, t), 1);
                }
            }
        }
    }

    let (&(x, t), &count) = x_intercepts.iter().max_by_key(|&(_, v)| v).unwrap();

    (x, t, count)
}
