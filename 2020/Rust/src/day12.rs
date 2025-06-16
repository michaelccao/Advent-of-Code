use crate::helper::read_data;

pub fn main() {
    let data: String = read_data("../Data/Day12.txt");

    let directions: Vec<(char, i32)> = get_directions(&data);

    let p1: i32 = follow_directions(&directions);

    println!("{p1}");

    let p2: i32 = follow_directions2(&directions);

    println!("{p2}");
}

fn get_directions(data: &String) -> Vec<(char, i32)> {
    let mut directions: Vec<(char, i32)> = Vec::new();

    for line in data.lines() {
        let direction = line[0..1].chars().next().unwrap();
        let amount: i32 = line[1..].parse().unwrap();

        directions.push((direction, amount));
    }

    directions
}

fn follow_directions(directions: &Vec<(char, i32)>) -> i32 {
    let mut x: i32 = 0;
    let mut y: i32 = 0;

    let headings: [(i32, i32); 4] = [(1, 0), (0, -1), (-1, 0), (0, 1)];

    let mut heading: usize = 0;

    for &(direction, amount) in directions {
        match direction {
            'E' => x += amount,
            'W' => x -= amount,
            'N' => y += amount,
            'S' => y -= amount,
            'R' => {
                heading += (amount / 90) as usize;
                heading %= 4;
            }
            'L' => {
                heading += 4;
                heading -= (amount / 90) as usize;
                heading %= 4;
            }
            'F' => {
                let (dx, dy) = headings[heading];
                x += dx * amount;
                y += dy * amount;
            }
            _ => {}
        }
    }

    x.abs() + y.abs()
}

fn follow_directions2(directions: &Vec<(char, i32)>) -> i32 {
    let mut x: i32 = 0;
    let mut y: i32 = 0;

    let mut wx: i32 = 10;
    let mut wy: i32 = 1;

    for &(direction, amount) in directions {
        match direction {
            'E' => wx += amount,
            'W' => wx -= amount,
            'N' => wy += amount,
            'S' => wy -= amount,
            'R' => {
                for _ in 0..amount / 90 {
                    (wx, wy) = (wy, -wx);
                }
            }
            'L' => {
                for _ in 0..amount / 90 {
                    (wx, wy) = (-wy, wx);
                }
            }
            'F' => {
                x += wx * amount;
                y += wy * amount;
            }
            _ => {}
        }
    }

    x.abs() + y.abs()
}
