use crate::helper::read_data;
use std::collections::HashSet;

pub fn main() {
    let data: String = read_data("../Data/Day02.txt");

    let p1: u64 = get_invalid_ids(&data);

    println!("{p1}");

    let p2: u64 = get_invalid_ids2(&data);

    println!("{p2}");
}

fn get_invalid_ids(data: &String) -> u64 {
    let mut total: u64 = 0;

    for rg in data.split(",") {
        let mut rg = rg.split("-");
        let start: &str = rg.next().unwrap();
        let start_val: u64 = start.parse().unwrap();
        let end: &str = rg.next().unwrap();
        let end_val: u64 = end.parse().unwrap();

        if start.len() % 2 == 1 && start.len() == end.len() {
            continue;
        }

        let mut div = 10_u64.pow(start.len() as u32 / 2);
        if start.len() % 2 == 1 {
            div *= 10;
        }
        let mut prefix = start_val / div;

        while prefix * div + prefix <= end_val {
            if prefix * div + prefix >= start_val {
                total += prefix * div + prefix;
            }

            prefix += 1;
            div = 10_u64.pow(prefix.to_string().len() as u32);
        }
    }

    total
}

fn get_invalid_ids2(data: &String) -> u64 {
    let mut ids: HashSet<u64> = HashSet::new();

    for rg in data.split(",") {
        let mut rg = rg.split("-");
        let start: &str = rg.next().unwrap();
        let start_val: u64 = start.parse().unwrap();
        let end: &str = rg.next().unwrap();
        let end_val: u64 = end.parse().unwrap();

        for prefix_length in 1..end.len() / 2 + 1 {
            if start.len() % prefix_length == 0 {
                let rep: usize = start.len() / prefix_length;

                let mut prefix: u64 = start[0..prefix_length].parse().unwrap();

                let mut num: u64 = tile_num(prefix, rep);
                while rep > 1 && num <= end_val {
                    if num >= start_val {
                        ids.insert(num);
                    }

                    prefix += 1;
                    num = tile_num(prefix, rep);
                }
            }

            if end.len() > start.len() && end.len() % prefix_length == 0 {
                let rep: usize = end.len() / prefix_length;

                let mut prefix: u64 = end[0..prefix_length].parse().unwrap();

                let mut num = tile_num(prefix, rep);
                while rep > 1 && num <= end_val {
                    if num >= start_val {
                        ids.insert(num);
                    }

                    prefix += 1;
                    num = tile_num(prefix, rep);
                }
            }
        }
    }

    ids.iter().sum()
}

fn tile_num(num: u64, n: usize) -> u64 {
    let digits = num.to_string().len() as u32;
    let mult: u64 = 10_u64.pow(digits);

    let mut total: u64 = num;

    for _ in 1..n {
        total *= mult;
        total += num;
    }

    total
}
