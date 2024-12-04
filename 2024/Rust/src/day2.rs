use crate::helper::read_data;
use std::vec::Vec;

pub fn main() {

    let data = read_data("../Data/Day2.txt");

    let levels = get_levels(&data);

    let p1:i32 = levels.clone().into_iter().map(|level| is_safe(&level)).sum();

    println!("{p1}");

    let p2:i32 = levels.clone().into_iter().map(|level| is_sorta_safe(&level)).sum();

    println!("{p2}");

}

fn get_levels(data: &String) -> Vec<Vec<i32>> {

    let mut levels:Vec<Vec<i32>> = Vec::new();

    for line in data.split("\n") {
        let mut levels_line:Vec<i32> = Vec::new();
        for num in line.split(" ") {
            levels_line.push(num.trim().parse::<i32>().unwrap());
        }
        levels.push(levels_line)
    }

    levels

}

fn is_safe(levels: &Vec<i32>) -> i32 {
    let inc:bool = levels[1] - levels[0] > 0;

    for i in 0..levels.len()-1 {
        let a = levels[i];
        let b = levels[i+1];

        let diff = if inc { b-a } else { a-b };

        if diff < 1 || diff > 3 {
            return 0
        }
        
    }

    return 1

}

fn is_sorta_safe(levels: &Vec<i32>) -> i32 {

    for i in 0..levels.len() {
        let mut level2 = levels.clone();
        level2.remove(i);

        if is_safe(&level2) == 1 {
            return 1
        }
    }

    return 0

}