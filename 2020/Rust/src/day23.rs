use crate::helper::read_data;
use std::collections::{HashMap, HashSet};

pub fn main() {
    let data: String = read_data("../Data/Day23.txt");

    let cups: Vec<u32> = data.chars().map(|c| c.to_digit(10).unwrap()).collect();

    let p1: String = move_cups(&cups, 100);

    println!("{p1}");

    let p2: u64 = move_cups2(&cups, 1_000_000, 10_000_000);

    println!("{p2}");
}

fn move_cups(cups: &Vec<u32>, moves: u32) -> String {
    let mut cups: Vec<u32> = cups.clone();

    let mut current: usize = 0;

    for _ in 0..moves {
        let mut target: u32 = cups[current] - 1;
        if target == 0 {
            target = 9;
        }


        let mut remove: Vec<u32> = Vec::new();

        for __ in 0..3 {
            if current + 1 < cups.len() {
                remove.push(cups.remove(current+1));
            } else {
                remove.push(cups.remove(0));
                current -= 1;
            }
            
        }

        let remove_set: HashSet<&u32> = remove.iter().collect::<HashSet<_>>();

        while remove_set.contains(&target) {
            target -= 1;

            if target == 0 {
                target = 9;
            }
        }

        let mut target_ind: usize = 0;

        while cups[target_ind] != target {
            target_ind += 1;
        }

        for __ in 0..3 {
            cups.insert(target_ind+1,remove.pop().unwrap());
        }

        if target_ind < current {
            current += 3;
        }

        current = (current + 1) % cups.len();
    }

    let mut cup_string: String = String::new();

    let mut i: usize = 0;
    let mut add: bool = false;

    while cup_string.len() < cups.len()-1 {
        if add {
            cup_string.push(char::from_digit(cups[i], 10).unwrap());
        }
        if cups[i] == 1 {
            add = true;
        }

        i += 1;
        i %= cups.len();
    }

    cup_string
}

fn move_cups2(cups: &Vec<u32>, num_cups: usize, moves: u32) -> u64 {
    let mut cups2: HashMap<u32, u32> = HashMap::new();

    for i in 0..num_cups {
        let cup: u32 = if i < cups.len() {
            cups[i]
        } else {
            i as u32 + 1
        };

        let next: u32 = if i + 1 < cups.len() {
            cups[i+1]
        } else if i + 2 <= num_cups {
            i as u32 + 2
        } else {
            cups[0]
        };

        cups2.insert(cup, next);
    }

    let mut current_cup = cups[0];

    for _ in 0..moves {
        let mut target_cup = current_cup - 1;
        if target_cup <= 0 {
            target_cup += cups2.len() as u32;
        }

        let mut removed: Vec<u32> = Vec::new();
        let mut removed_set: HashSet<u32> = HashSet::new();

        let mut next: u32 = cups2[&current_cup];

        for _remove in 0..3 {
            removed.push(next);
            removed_set.insert(next);

            next = cups2[&next];
        }

        cups2.insert(current_cup, next);

        while removed_set.contains(&target_cup) {
            target_cup -= 1;

            if target_cup <= 0 {
                target_cup += cups2.len() as u32;
            }
        }

        next = cups2[&target_cup];

        while removed.len() > 0 {
            let add = removed.pop().unwrap();
            cups2.insert(add, next);
            next = add;
        }
        cups2.insert(target_cup, next);

        current_cup = cups2[&current_cup];
    }

    let cup1: u32 = cups2[&1];
    let cup2: u32 = cups2[&cup1];

    cup1 as u64 * cup2 as u64
}