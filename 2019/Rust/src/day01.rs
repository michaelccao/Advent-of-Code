use crate::helper::read_data;

pub fn main() {
    let data: String = read_data("../Data/Day01.txt");
    let modules: Vec<i32> = data
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect();

    let p1: i32 = modules.iter().map(|&num| num / 3 - 2).sum();

    println!("{p1}");

    let p2: i32 = modules.iter().map(|&num| fuel_cost(num)).sum();

    println!("{p2}");
}

fn fuel_cost(mut mass: i32) -> i32 {
    let mut fuel: i32 = 0;

    while mass > 6 {
        let fuel_cost = mass / 3 - 2;
        fuel += fuel_cost;
        mass = fuel_cost;
    }

    fuel
}
