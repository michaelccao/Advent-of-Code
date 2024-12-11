use crate::helper::read_data;
use std::collections::HashMap;

pub fn main() {
    
    let data = read_data("../Data/Day11.txt");
    
    let mut cache: HashMap<(u64, u8), u64> = HashMap::new();

    let p1:u64 = data.trim().split(" ").map(|num| blink(num.parse::<u64>().unwrap(), 25, &mut cache)).sum();
    let p2:u64 = data.trim().split(" ").map(|num| blink(num.parse::<u64>().unwrap(), 75, &mut cache)).sum();

    println!("{p1}");
    println!("{p2}");
}

fn split_num(num: u64) -> (u64, u64) {
    let divisor: u64 = 10u64.pow((num.ilog10()+1)/2);

    return (num/divisor, num % divisor)
}

fn blink(num: u64, blinks: u8, cache: &mut HashMap<(u64, u8), u64>) -> u64 {
    if cache.contains_key(&(num, blinks)) {
        return cache[&(num, blinks)];
    } else if blinks == 1 {
        if num == 0 {
            cache.insert((0, 1), 1);
        } else if num.ilog10() % 2 == 1 {
            cache.insert((num, 1), 2);
        } else {
            cache.insert((num, 1), 1);
        }
    } else {
        let res: u64;
        if num == 0 {
            res = blink(1, blinks-1, cache);
        } else if num.ilog10() % 2 == 1 {
            let (num1, num2) = split_num(num);

            res = blink(num1, blinks-1, cache) + blink(num2, blinks-1, cache);
        } else {
            res = blink(num*2024, blinks-1, cache);
        }

        cache.insert((num, blinks), res);
    }

    return cache[&(num, blinks)]
}

