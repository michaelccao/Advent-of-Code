use crate::helper::read_data;
use std::collections::VecDeque;

pub fn main() {
    let data: String = read_data("../Data/Day14.txt");

    let num_recipes: usize = data.parse().unwrap();

    let p1: String = get_recipes(num_recipes);
    let p2: usize = get_recipes2(&data);

    println!("{p1}\n{p2}");
}

fn get_recipes(num_recipes: usize) -> String {
    let mut recipes: Vec<u8> = vec![3, 7];
    let mut i: usize = 0;
    let mut j: usize = 1;

    while recipes.len() < num_recipes + 10 {
        let new_recipes = recipes[i] + recipes[j];
        if new_recipes >= 10 {
            recipes.push(1);
            recipes.push(new_recipes - 10);
        } else {
            recipes.push(new_recipes);
        }

        i += 1 + recipes[i] as usize;
        j += 1 + recipes[j] as usize;

        i %= recipes.len();
        j %= recipes.len();
    }

    let mut num_string: String = String::new();

    for &score in &recipes[num_recipes..num_recipes + 10] {
        let score: char = char::from_digit(score as u32, 10).unwrap();
        num_string.push(score);
    }
    num_string
}

fn get_recipes2(target: &String) -> usize {
    let mut recipes: VecDeque<u8> = VecDeque::from([3, 7]);
    let mut i: usize = 0;
    let mut j: usize = 1;

    let target: VecDeque<u8> = target
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();
    let mut buffer: VecDeque<u8> = recipes.clone();

    while buffer != target {
        let new_recipes = recipes[i] + recipes[j];
        if new_recipes >= 10 {
            recipes.push_back(1);

            buffer.push_back(1);
            if buffer == target {
                break;
            } else if buffer.len() > target.len() {
                buffer.pop_front();
                if buffer == target {
                    break;
                }
            }

            recipes.push_back(new_recipes - 10);
            buffer.push_back(new_recipes - 10);
        } else {
            recipes.push_back(new_recipes);
            buffer.push_back(new_recipes);
        }

        if buffer.len() > target.len() {
            buffer.pop_front();
        }

        i += 1 + recipes[i] as usize;
        j += 1 + recipes[j] as usize;

        i %= recipes.len();
        j %= recipes.len();
    }

    recipes.len() - buffer.len()
}
