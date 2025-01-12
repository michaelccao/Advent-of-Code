use crate::helper::read_data;
use std::collections::{HashMap, HashSet};
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day19.txt");

    let num_elves: u32 = data.parse().unwrap();

    println!("{}", white_elephant(num_elves, false));

    // // Found pattern doing slow way
    // for i in 2..1000_u32 {
    //     println!("{}, {}",i, white_elephant(i, true));
    //     }

    println!("{}", white_elephant2(num_elves));

    }

    

fn white_elephant(num_elves: u32, part2: bool) -> u32 {

    let mut elves: HashMap<u32, (u32, u32)> = HashMap::new();

    for i in 1..num_elves+1 {
        elves.insert(i, (if i != num_elves {i + 1} else {1}, 1));
    }

    let mut current_elf: u32 = 1;

    loop {
        let (next_elf, gifts) = elves[&current_elf];

        let mut pre_target= current_elf;
        let mut target_elf = next_elf;
        let mut post_target = elves[&target_elf].0;

        if part2 {
            for _ in 1..elves.len()/2 {
                pre_target = target_elf;
                target_elf = post_target;
                post_target = elves[&post_target].0;
            }
        }

        let stolen_gifts = elves[&target_elf].1;

        if gifts + stolen_gifts == num_elves{
            return current_elf
        }

        elves.insert(current_elf, (elves[&current_elf].0, gifts + stolen_gifts));
        elves.remove(&target_elf);

        elves.insert(pre_target, (post_target, elves[&pre_target].1));

        current_elf = elves[&current_elf].0;


    }

}

fn white_elephant2(num_elves: u32) -> u32 {

    let mut i: u32 = 1;

    while i < num_elves {
        i *= 3;
    }

    if i == num_elves {
        return i
    } else {
        i /= 3;
    }

    if num_elves <= 2*i {
        return num_elves - i
    } else {
        3*i - (3*i - num_elves)*2
    }

}