use crate::helper::read_data;
use std::collections::HashMap;

pub fn main() {
    let data: String = read_data("../Data/Day06.txt");
    let fish: Vec<u64> = data.split(",").map(|n| n.parse().unwrap()).collect();

    let p1: u64 = total_fish(&fish, 80);

    println!("{p1}");

    let p2: u64 = total_fish(&fish, 256);

    println!("{p2}");
}

fn generate_fish(
    days_left: u64,
    days: u64,
    cache: HashMap<(u64, u64), u64>,
) -> (u64, HashMap<(u64, u64), u64>) {
    if let Some(&fish) = cache.get(&(days_left, days)) {
        (fish, cache)
    } else if days <= days_left {
        (1, cache)
    } else {
        let (res1, cache) = generate_fish(6, days - days_left - 1, cache);
        let (res2, mut cache) = generate_fish(8, days - days_left - 1, cache);

        cache.insert((days_left, days), res1 + res2);

        return (res1 + res2, cache);
    }
}

fn total_fish(fish: &Vec<u64>, days: u64) -> u64 {
    let mut cache: HashMap<(u64, u64), u64> = HashMap::new();

    let mut total: u64 = 0;

    for &f in fish {
        let res = generate_fish(f, days, cache);
        cache = res.1;
        total += res.0;
    }

    total
}
