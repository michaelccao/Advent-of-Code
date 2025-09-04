use crate::helper::read_data;
use std::collections::HashMap;

pub fn main() {
    let data: String = read_data("../Data/Day05.txt");

    let lines: Vec<(i32, i32, i32, i32)> = get_lines(&data);

    let p1: usize = count_intersections(&lines, false);

    println!("{p1}");

    let p2: usize = count_intersections(&lines, true);

    println!("{p2}");
}

fn get_lines(data: &String) -> Vec<(i32, i32, i32, i32)> {
    let mut lines: Vec<(i32, i32, i32, i32)> = Vec::new();

    for line in data.lines() {
        let mut line = line.split(" -> ");
        let start: Vec<&str> = line.next().unwrap().split(",").collect();
        let x0: i32 = start[0].parse().unwrap();
        let y0: i32 = start[1].parse().unwrap();

        let end: Vec<&str> = line.next().unwrap().split(",").collect();
        let x1: i32 = end[0].parse().unwrap();
        let y1: i32 = end[1].parse().unwrap();

        lines.push((x0, y0, x1, y1));
    }

    lines
}

fn count_intersections(lines: &Vec<(i32, i32, i32, i32)>, part2: bool) -> usize {
    let mut points: HashMap<(i32, i32), i32> = HashMap::new();

    for &(x0, y0, x1, y1) in lines {
        if x0 == x1 {
            let (y0, y1) = if y0 > y1 { (y1, y0) } else { (y0, y1) };

            for y in y0..y1 + 1 {
                if let Some(count) = points.get_mut(&(x0, y)) {
                    *count += 1;
                } else {
                    points.insert((x0, y), 1);
                }
            }
        } else if y0 == y1 {
            let (x0, x1) = if x0 > x1 { (x1, x0) } else { (x0, x1) };

            for x in x0..x1 + 1 {
                if let Some(count) = points.get_mut(&(x, y0)) {
                    *count += 1;
                } else {
                    points.insert((x, y0), 1);
                }
            }
        } else if part2 {
            let steps: u32 = i32::abs_diff(x1, x0) + 1;
            let dx: i32 = if x1 > x0 { 1 } else { -1 };
            let dy: i32 = if y1 > y0 { 1 } else { -1 };

            let mut x: i32 = x0;
            let mut y: i32 = y0;

            for _ in 0..steps {
                if let Some(count) = points.get_mut(&(x, y)) {
                    *count += 1;
                } else {
                    points.insert((x, y), 1);
                }

                x += dx;
                y += dy;
            }
        }
    }

    points.values().filter(|&&v| v > 1).count()
}
