use crate::helper::read_data;
use std::collections::{HashMap, HashSet};
use std::vec::Vec;
use regex::Regex;

pub fn main() {
    let data: String = read_data("../Data/Day11.txt");

    let (floors, components) = get_floors(&data);

}

fn get_floors(data: &String) -> (Vec<Vec<(&str, &str)>>, HashMap<(&str, &str), usize>) {
    let mut floors: Vec<Vec<(&str, &str)>> = Vec::new();
    
    let mut components: HashMap<(&str, &str), usize> = HashMap::new();

    let re = Regex::new(r"a (\S*?)-compatible (microchip)|a (\S*?) (generator)").unwrap();

    for (floor, line) in data.lines().enumerate() {
        let mut floor_components: Vec<(&str, &str)> = Vec::new();

        for cap in re.captures_iter(line) {
            let (_, [element, component]) = cap.extract();

            floor_components.push((element, component));

            components.insert((element, component), floor);
        }

        floor_components.sort();

        floors.push(floor_components);

    }


    (floors, components)
}

fn is_valid(floors: &Vec<Vec<(&str, &str)>>) -> bool {

    for floor in floors {
        let mut chips: HashSet<&str> = HashSet::new();
        let mut generators: HashSet<&str> = HashSet::new();

        for component in floor {
            if component.1 == "microchip" {
                chips.insert(component.0);
            } else {
                generators.insert(component.0);
            }
        }

        for chip in chips {
            if generators.contains(chip) {
                continue;
            } else if generators.len() > 0 {
                return false;
            }
        }
    }

    true

}