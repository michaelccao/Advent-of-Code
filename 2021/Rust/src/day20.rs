use crate::helper::read_data;
use std::collections::HashSet;

pub fn main() {
    let data: String = read_data("../Data/Day20.txt");

    let (algo, image) = get_image_enhancement_algo(&data);
    // Puzzle input adds an additional challenge compared to test input
    // by starting algo with '#' and ending with '.'
    // This causes a flipping in the "default" value of the surroundings

    let p1: HashSet<(i32, i32)> = enhance_n(&image, &algo, 2);

    println!("{}", p1.len());

    let p2: HashSet<(i32, i32)> = enhance_n(&image, &algo, 50);

    println!("{}", p2.len());
}

fn get_image_enhancement_algo(data: &String) -> (Vec<char>, HashSet<(i32, i32)>) {
    let mut lines = data.lines();

    let algo: Vec<char> = lines.next().unwrap().chars().collect();

    lines.next();

    let mut image: HashSet<(i32, i32)> = HashSet::new();

    for (i, row) in lines.enumerate() {
        for (j, c) in row.chars().enumerate() {
            if c == '#' {
                image.insert((i as i32, j as i32));
            }
        }
    }

    (algo, image)
}

fn get_neighborhood_value(
    image: &HashSet<(i32, i32)>,
    i: i32,
    j: i32,
    i_min: i32,
    i_max: i32,
    j_min: i32,
    j_max: i32,
    default_value: bool,
) -> usize {
    let neighbors: [(i32, i32); 9] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 0),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let mut total: usize = 0;

    for (di, dj) in neighbors {
        total *= 2;

        let i2: i32 = i + di;
        let j2: i32 = j + dj;

        if image.contains(&(i2, j2)) {
            total += 1;
        } else if default_value && (i2 < i_min || i2 > i_max || j2 < j_min || j2 > j_max) {
            total += 1;
        }
    }

    total
}

fn enhance(
    image: &HashSet<(i32, i32)>,
    algo: &Vec<char>,
    default_value: bool,
) -> HashSet<(i32, i32)> {
    let mut image2: HashSet<(i32, i32)> = HashSet::new();

    let mut i_min: i32 = i32::MAX;
    let mut i_max: i32 = i32::MIN;

    let mut j_min: i32 = i32::MAX;
    let mut j_max: i32 = i32::MIN;

    for &(i, j) in image {
        i_min = i_min.min(i);
        i_max = i_max.max(i);

        j_min = j_min.min(j);
        j_max = j_max.max(j);
    }

    for i in i_min - 1..i_max + 2 {
        for j in j_min - 1..j_max + 2 {
            if algo[get_neighborhood_value(image, i, j, i_min, i_max, j_min, j_max, default_value)]
                == '#'
            {
                image2.insert((i, j));
            }
        }
    }

    image2
}

fn enhance_n(image: &HashSet<(i32, i32)>, algo: &Vec<char>, n: usize) -> HashSet<(i32, i32)> {
    let mut image2: HashSet<(i32, i32)> = image.clone();
    let mut default_value: bool = false;

    for _ in 0..n {
        image2 = enhance(&image2, algo, default_value);
        default_value = !default_value;
    }

    image2
}
