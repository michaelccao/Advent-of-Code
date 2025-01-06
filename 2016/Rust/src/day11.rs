use crate::helper::read_data;
use std::collections::{HashMap, HashSet};
use std::vec::Vec;
use regex::Regex;

pub fn main() {
    let data: String = read_data("../Data/Day11.txt");

    let (floors, components) = get_floors(&data);

    println!("{floors:?}");

    println!("{components:?}");
}

fn get_floors(data: &String) -> (Vec<HashSet<(&str, &str)>>, HashMap<(&str, &str), usize>) {
    let mut floors: Vec<HashSet<(&str, &str)>> = Vec::new();
    
    let mut components: HashMap<(&str, &str), usize> = HashMap::new();

    let re = Regex::new(r"a (\S*?)-compatible (microchip)|a (\S*?) (generator)").unwrap();

    for (floor, line) in data.lines().enumerate() {
        let mut floor_components: HashSet<(&str, &str)> = HashSet::new();

        for cap in re.captures_iter(line) {
            let (_, [element, component]) = cap.extract();

            floor_components.insert((element, component));

            components.insert((element, component), floor);
        }

        floors.push(floor_components);

    }


    (floors, components)
}