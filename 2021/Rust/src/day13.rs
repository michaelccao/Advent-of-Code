use crate::helper::read_data;
use std::collections::HashSet;

pub fn main() {
    let data: String = read_data("../Data/Day13.txt");

    let (dots, folds) = get_instructions(&data);

    let p1: usize = perform_fold(&dots, &folds[0]).len();

    println!("{p1}");

    let p2: String = perform_folds(&dots, &folds);

    println!("{p2}");
}

fn get_instructions(data: &String) -> (HashSet<(usize, usize)>, Vec<(char, usize)>) {
    let mut dots: HashSet<(usize, usize)> = HashSet::new();
    let mut folds: Vec<(char, usize)> = Vec::new();

    for line in data.lines() {
        if line.starts_with("fold") {
            let mut line = line.split("=");
            let axis = line.next().unwrap().chars().last().unwrap();
            let coord: usize = line.next().unwrap().parse().unwrap();

            folds.push((axis, coord));
        } else if line.len() > 0 {
            let mut line = line.split(",");
            let x: usize = line.next().unwrap().parse().unwrap();
            let y: usize = line.next().unwrap().parse().unwrap();

            dots.insert((x, y));
        }
    }

    (dots, folds)
}

fn perform_fold(dots: &HashSet<(usize, usize)>, fold: &(char, usize)) -> HashSet<(usize, usize)> {
    let mut dots2: HashSet<(usize, usize)> = HashSet::new();

    let &(axis, coord) = fold;

    for &(x, y) in dots {
        if axis == 'x' {
            if x < coord {
                dots2.insert((x, y));
            } else if x > coord {
                dots2.insert((2 * coord - x, y));
            }
        } else if axis == 'y' {
            if y < coord {
                dots2.insert((x, y));
            } else if y > coord {
                dots2.insert((x, 2 * coord - y));
            }
        }
    }

    dots2
}

fn perform_folds(dots: &HashSet<(usize, usize)>, folds: &Vec<(char, usize)>) -> String {
    let mut dots2: HashSet<(usize, usize)> = dots.clone();

    for fold in folds {
        dots2 = perform_fold(&dots2, fold);
    }

    let mut x_min: usize = usize::MAX;
    let mut y_min: usize = usize::MAX;

    let mut x_max: usize = 0;
    let mut y_max: usize = 0;

    for &(x, y) in &dots2 {
        x_min = x_min.min(x);
        y_min = y_min.max(y);

        x_max = x_max.max(x);
        y_max = y_max.max(y);
    }

    let mut code: Vec<Vec<char>> = vec![vec![' '; x_max - x_min + 1]; y_max - y_min + 1];

    for &(x, y) in &dots2 {
        code[y - y_min][x - x_min] = '#';
    }

    let mut code_str: String = String::new();

    for row in code {
        code_str.push_str(&row.iter().collect::<String>());
        code_str.push('\n');
    }

    code_str
}
