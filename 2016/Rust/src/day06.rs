use crate::helper::read_data;
use std::collections::HashMap;
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day06.txt");

    let messages: Vec<Vec<char>> = get_messages(&data);

    let p1: String = decode(&messages, false);
    let p2: String = decode(&messages, true);

    println!("{p1}\n{p2}");
}

fn get_messages(data: &String) -> Vec<Vec<char>> {
    data.lines()
        .map(|line| line.trim().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

fn decode(messages: &Vec<Vec<char>>, part2: bool) -> String {
    let mut message: String = String::new();

    let mut counters: Vec<HashMap<char, u32>> = vec![HashMap::new(); messages[0].len()];

    for m in messages {
        for (i, c) in m.iter().enumerate() {
            if let Some(count) = counters[i].get_mut(c) {
                *count += 1;
            } else {
                counters[i].insert(*c, 1);
            }
        }
    }

    if part2 {
        counters
            .iter()
            .for_each(|counter| message.push(*counter.keys().min_by_key(|c| counter[*c]).unwrap()));
    } else {
        counters
            .iter()
            .for_each(|counter| message.push(*counter.keys().max_by_key(|c| counter[*c]).unwrap()));
    }

    message
}
