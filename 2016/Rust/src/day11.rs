use crate::helper::read_data;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::vec::Vec;

#[derive(Debug, Clone)]
struct Layout {
    chips: Vec<i8>,
    gens: Vec<i8>,
    elevator: i8,
    floors: i8,
}

pub fn main() {
    let data: String = read_data("../Data/Day11.txt");

    let (floors, _components) = get_floors(&data);

    let mut floors2: Vec<Vec<(&str, &str)>> = floors.clone();

    let (chips, gens) = get_components(&data);

    let layout = Layout {
        chips,
        gens,
        elevator: 0,
        floors: data.lines().collect::<Vec<_>>().len() as i8
    };

    println!("{layout:?}");
    println!("{}", layout.is_valid());

    // floors2[1].push(("elerium", "generator"));
    // floors2[1].push(("elerium", "microchip"));
    // floors2[1].push(("dilithium", "generator"));
    // floors2[1].push(("dilithium", "microchip"));

    // floors2[1].sort();

    // let p1: u32 = shortest_moves(&floors);
    // println!("{p1}");

    // println!("{}", floor_is_valid(&vec![("curium", "generator"), ("curium", "microchip"), ("plutonium", "generator"), ("promethium", "microchip"), ("ruthenium", "generator")]));

    // let p2: u32 = shortest_moves(&floors2);

    // println!("{p2}");
}

fn get_components(data: &String) -> (Vec<i8>, Vec<i8>) {
    let mut chips: HashMap<String, i8> = HashMap::new();
    let mut gens: HashMap<String, i8> = HashMap::new();

    let re =  Regex::new(r"a (\S*?)-compatible (microchip)|a (\S*?) (generator)").unwrap();

    for (floor, line) in data.lines().enumerate() {
        let floor = floor as i8;

        for cap in re.captures_iter(line) {
            let (_, [element, component]) = cap.extract();

            if component == "microchip" {
                chips.insert(element.to_string(), floor);
            } else {
                gens.insert(element.to_string(), floor);
            }
        }
    }

    let mut elements: Vec<&String> = chips.keys().collect();
    elements.sort();

    let mut chips_vec: Vec<i8> = Vec::new();
    let mut gens_vec: Vec<i8> = Vec::new();

    for element in elements {
        chips_vec.push(chips[element]);
        gens_vec.push(gens[element]);
    }

    (chips_vec, gens_vec)
}

impl Layout {
    fn is_valid(&self) -> bool {
        let chips = &self.chips;
        let gens = &self.gens;

        for i in 0..chips.len() {
            if chips[i] == gens[i] {
                continue;
            }
    
            for j in 0..gens.len() {
                if chips[i] == gens[j] {
                    return false;
                }
            }
        }
    
        true
    }

    fn move_chips(&self, to_move: HashSet<usize>, shift: i8) -> Option<Layout> {
        let mut layout2: Layout = self.clone();

        for c in to_move {
            layout2.chips[c] += shift;
        }

        if layout2.is_valid() {
            return Some(layout2);
        } else {
            return None;
        }
    }

    fn move_gens(&self, to_move: HashSet<usize>, shift: i8) -> Option<Layout> {
        let mut layout2: Layout = self.clone();

        for c in to_move {
            layout2.gens[c] += shift;
        }

        if layout2.is_valid() {
            return Some(layout2);
        } else {
            return None;
        }
    }

    fn move_pair(&self, to_move: usize, shift: i8) -> Option<Layout> {
        let mut layout2: Layout = self.clone();

        layout2.chips[to_move] += shift;
        layout2.gens[to_move] += shift;
        
        if layout2.is_valid() {
            return Some(layout2);
        } else {
            return None;
        }
    }

    fn next_moves(&self) -> Vec<Layout> {
        let mut layouts: Vec<Layout> = Vec::new();

        for i in 0..self.chips.len() {
            if self.chips[i] == self.elevator {

                if self.gens[i] == self.elevator {
                    if self.elevator > 0 {
                        if let Some(layout2) = self.move_pair(i, -1) {
                            layouts.push(layout2);
                        }
                    }

                    if self.elevator < self.floors - 1 {
                        if let Some(layout2) = self.move_pair(i, 1) {
                            layouts.push(layout2);
                        }
                    }
                }

                for j in i..self.chips.len() {
                    if self.chips[j] != self.elevator {
                        continue;
                    }

                    let to_move: HashSet<usize> = HashSet::from([i, j]);

                    if self.elevator > 0 {
                        if let Some(layout2) = self.move_chips(to_move.clone(), -1) {
                            layouts.push(layout2);
                        }
                    }

                    if self.elevator < self.floors - 1 {
                        if let Some(layout2) = self.move_chips(to_move.clone(), 1) {
                            layouts.push(layout2);
                        }
                    }
                }

            }

            if self.gens[i] == self.elevator {
                for j in i..self.gens.len() {
                    let to_move: HashSet<usize> = HashSet::from([i, j]);

                    if self.elevator > 0 {
                        if let Some(layout2) = self.move_gens(to_move.clone(), -1) {
                            layouts.push(layout2);
                        }
                    }

                    if self.elevator < self.floors - 1 {
                        if let Some(layout2) = self.move_gens(to_move.clone(), 1) {
                            layouts.push(layout2);
                        }
                    }
                }
            }
        }

        layouts

    }
}

fn get_floors(data: &String) -> (Vec<Vec<(&str, &str)>>, HashMap<(&str, &str), usize>) {
    let mut floors: Vec<Vec<(&str, &str)>> = Vec::new();

    let mut components: HashMap<(&str, &str), usize> = HashMap::new();

    let re: Regex = Regex::new(r"a (\S*?)-compatible (microchip)|a (\S*?) (generator)").unwrap();

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

fn floor_is_valid(floor: &Vec<(&str, &str)>) -> bool {

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

    true
}

fn valid_moves<'a>(
    floors: &Vec<Vec<(&'a str, &'a str)>>,
    elevator: usize,
) -> Vec<(Vec<Vec<(&'a str, &'a str)>>, usize)> {
    let mut moves: Vec<(Vec<Vec<(&str, &str)>>, usize)> = Vec::new();

    let num_comps: usize = floors[elevator].len();

    let comps_above: usize = if elevator == floors.len() - 1 {
        0
    } else {
        floors[elevator + 1..].iter().map(|floor| floor.len()).sum()
    };
    let comps_below: usize = floors[0..elevator].iter().map(|floor| floor.len()).sum();

    for i in 0..num_comps {
        let comp1 = floors[elevator][i];

        if comps_below > 0 {
            let mut move_one_down = floors.clone();
            move_one_down[elevator].remove(i);

            move_one_down[elevator - 1].push(comp1);
            move_one_down[elevator - 1].sort();

            if floor_is_valid(&move_one_down[elevator]) && floor_is_valid(&move_one_down[elevator-1]) {
                moves.push((move_one_down, elevator - 1));
            }

        }

        if comps_above > 0 {
            let mut move_one_up = floors.clone();
            move_one_up[elevator].remove(i);

            move_one_up[elevator + 1].push(comp1);
            move_one_up[elevator + 1].sort();

            if floor_is_valid(&move_one_up[elevator]) && floor_is_valid(&move_one_up[elevator+1]) {
                moves.push((move_one_up, elevator + 1));
            }

        }

        for j in i + 1..num_comps {

            let comp2 = floors[elevator][j];

            // Don't move if it's non-compatible chip-generator pair
            if comp1.1 != comp2.1 && comp1.0 != comp2.0 {
                continue;
            }

            if comps_below > 0 {
                let mut move_two_down = floors.clone();
                
                move_two_down[elevator].remove(j);
                move_two_down[elevator].remove(i);

                move_two_down[elevator-1].push(comp1);
                move_two_down[elevator-1].push(comp2);

                move_two_down[elevator-1].sort();

                if floor_is_valid(&move_two_down[elevator]) && floor_is_valid(&move_two_down[elevator-1]) {
                    moves.push((move_two_down, elevator-1));
                }
            }

            if elevator < floors.len() - 1 {
                let mut move_two_up = floors.clone();

                move_two_up[elevator].remove(j);
                move_two_up[elevator].remove(i);

                move_two_up[elevator+1].push(comp1);
                move_two_up[elevator+1].push(comp2);

                move_two_up[elevator+1].sort();

                if floor_is_valid(&move_two_up[elevator]) && floor_is_valid(&move_two_up[elevator+1]) {
                    moves.push((move_two_up, elevator+1));
                }
            }
            
        }
    }

    moves
}

fn shortest_moves(floors: &Vec<Vec<(&str, &str)>>) -> u32 {
    let num_components: usize = floors.iter().map(|floor| floor.len()).sum();

    let mut visited: HashMap<(Vec<Vec<(&str, &str)>>, usize), u32> = HashMap::new();

    let mut nodes: Vec<(Vec<Vec<(&str, &str)>>, usize, u32)> = vec![(floors.clone(), 0, 0)];

    // We brute-forcing through a lot of bad decisions
    // We can stop going down really bad decision by making a good estimate as to
    // upper limit of moves
    let mut shortest: u32 = (num_components * 4 * floors.len()) as u32;

    while nodes.len() > 0 {
        let (floors, elevator, moves) = nodes.pop().unwrap();

        visited.insert((floors.clone(), elevator), moves);

        if floors[floors.len() - 1].len() == num_components {
            shortest = moves;
            continue;
        }

        if moves >= shortest {
            continue;
        }

        for (floors2, elevator2) in valid_moves(&floors, elevator) {
            // println!("{:?}, {:?}", floors2.clone(), elevator2);

            if !visited.contains_key(&(floors2.clone(), elevator2))
                || visited[&(floors2.clone(), elevator2)] > moves + 1
            {
                nodes.push((floors2, elevator2, moves + 1))
            }
        }
    }

    shortest
}
