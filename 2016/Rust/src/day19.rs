use crate::helper::read_data;
use std::collections::{HashMap, HashSet};
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day19.txt");

    let num_elves: u32 = data.parse().unwrap();

    println!("{}", white_elephant(num_elves, false));

    println!("{}", white_elephant(num_elves, true));
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