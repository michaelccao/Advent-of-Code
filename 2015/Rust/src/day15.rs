use crate::helper::read_data;
use itertools::Itertools;
use regex::Regex;
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day15.txt");

    let ingredients: Vec<Vec<i32>> = get_ingredients(&data);

    let recipes = get_recipes(ingredients.len() as i32, 100);

    let p1: i32 = recipes
        .iter()
        .map(|recipe| rate_recipe(recipe, &ingredients, None))
        .max()
        .unwrap();
    let p2: i32 = recipes
        .iter()
        .map(|recipe| rate_recipe(recipe, &ingredients, Some(500)))
        .max()
        .unwrap();

    println!("{p1}\n{p2}");
}

fn get_ingredients(data: &String) -> Vec<Vec<i32>> {
    let re = Regex::new(r"-?\d+").unwrap();

    data.lines()
        .map(|line| {
            re.find_iter(line)
                .map(|m| m.as_str().parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>()
}

fn rate_recipe(amounts: &Vec<i32>, ingredients: &Vec<Vec<i32>>, calories: Option<i32>) -> i32 {
    let mut categories: Vec<i32> = vec![0; ingredients[0].len()];

    for i in 0..amounts.len() {
        for j in 0..categories.len() {
            categories[j] += amounts[i] * ingredients[i][j];
        }
    }

    match calories {
        Some(cal) => {
            if categories[categories.len() - 1] != cal {
                return 0;
            }
        }
        None => {}
    }

    for category in &categories {
        if *category < 0 {
            return 0;
        }
    }

    categories.iter().product::<i32>() / categories[categories.len() - 1]
}

fn get_recipes(num_ingredients: i32, total: i32) -> Vec<Vec<i32>> {
    let mut recipes: Vec<Vec<i32>> = Vec::new();

    for combo in (0..num_ingredients + total - 1).combinations(num_ingredients as usize - 1) {
        let mut recipe: Vec<i32> = Vec::new();
        let mut prev: i32 = -1;

        for i in combo {
            recipe.push(i - prev - 1);
            prev = i;
        }
        recipe.push(num_ingredients + total - prev - 2);
        recipes.push(recipe);
    }

    recipes
}
