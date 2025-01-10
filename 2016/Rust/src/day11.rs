use crate::helper::read_data;
use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};
use std::vec::Vec;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Layout {
    chips: Vec<i8>,
    gens: Vec<i8>,
    elevator: i8,
    floors: i8,
}

pub fn main() {
    let data: String = read_data("../Data/Day11.txt");

    let (chips, gens) = get_components(&data);

    let layout: Layout = Layout {
        chips,
        gens,
        elevator: 0,
        floors: data.lines().collect::<Vec<_>>().len() as i8,
    };

    let mut layout2: Layout = layout.clone();

    layout2.chips.append(&mut vec![0, 0]);
    layout2.gens.append(&mut vec![0, 0]);

    let p1: u32 = layout.move_all_top().unwrap();
    println!("{p1}");

    let p2: u32 = layout2.move_all_top().unwrap();
    println!("{p2}");
}

fn get_components(data: &String) -> (Vec<i8>, Vec<i8>) {
    let mut chips: HashMap<String, i8> = HashMap::new();
    let mut gens: HashMap<String, i8> = HashMap::new();

    let re: Regex = Regex::new(r"a (\S*?)-compatible (microchip)|a (\S*?) (generator)").unwrap();

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
        let chips: &Vec<i8> = &self.chips;
        let gens: &Vec<i8> = &self.gens;

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

    fn move_chips(&self, to_move: &HashSet<usize>, shift: i8) -> Option<Layout> {        
        let mut layout2: Layout = self.clone();      

        layout2.elevator += shift;

        for c in to_move {
            layout2.chips[*c] += shift;
        }

        if layout2.is_valid() {
            return Some(layout2);
        } else {
            return None;
        }
    }

    fn move_gens(&self, to_move: &HashSet<usize>, shift: i8) -> Option<Layout> {     
        let mut layout2: Layout = self.clone();

        layout2.elevator += shift;

        for c in to_move {
            layout2.gens[*c] += shift;
        }

        if layout2.is_valid() {
            return Some(layout2);
        } else {
            return None;
        }
    }

    fn move_pair(&self, to_move: usize, shift: i8) -> Option<Layout> {      
        let mut layout2: Layout = self.clone();

        layout2.elevator += shift;

        layout2.chips[to_move] += shift;
        layout2.gens[to_move] += shift;

        if layout2.is_valid() {
            return Some(layout2);
        } else {
            return None;
        }
    }

    fn components_above(&self) -> bool {
        let chips_above: u32 = self.chips.iter().map(|cf| if *cf > self.elevator {1} else {0}).sum();
        let gens_above: u32 = self.gens.iter().map(|gf| if *gf > self.elevator {1} else {0}).sum();

        chips_above + gens_above > 0
    }

    fn components_below(&self) -> bool {
        let chips_above: u32 = self.chips.iter().map(|cf| if *cf < self.elevator {1} else {0}).sum();
        let gens_above: u32 = self.gens.iter().map(|gf| if *gf < self.elevator {1} else {0}).sum();

        chips_above + gens_above > 0
    }

    fn next_moves(&self) -> Vec<Layout> {
        let mut layouts: Vec<Layout> = Vec::new();

        let mut shifts: Vec<i8> = Vec::new();
        if self.elevator > 0 { shifts.push(-1) };
        if self.elevator < self.floors - 1 { shifts.push(1) };

        let above: bool = self.components_above();
        let below: bool = self.components_below();

        for i in 0..self.chips.len() {

            // Chip moves and Chip-Gen Pair moves
            if self.chips[i] == self.elevator {

                // Chip-Gen Pair Move
                if self.gens[i] == self.elevator {
                    for shift in &shifts {

                        // Don't move down if nothing below
                        if *shift == -1 && !below {
                            continue;
                        }
                        
                        if let Some(layout2) = self.move_pair(i, *shift) {
                            layouts.push(layout2);
                        }
                    }
                }

                // Chip Moves
                for j in i..self.chips.len() {
                    if self.chips[j] != self.elevator {
                        continue;
                    }

                    let to_move: HashSet<usize> = HashSet::from([i, j]);

                    for shift in &shifts {
                        // Don't move chips down if nothing below
                        if *shift == -1 && !below {
                            continue;
                        } else if *shift == 1 && i == j && !above {
                            // Don't move only one chip up if nothing above
                            continue;
                        }
                        if let Some(layout2) = self.move_chips(&to_move, *shift) {
                            layouts.push(layout2);
                        }
                    }
                }
            }

            // Gen Moves
            if self.gens[i] == self.elevator {
                for j in i..self.gens.len() {
                    if self.gens[j] != self.elevator {
                        continue;
                    }
                    
                    let to_move: HashSet<usize> = HashSet::from([i, j]);

                    for shift in &shifts {
                        // Don't move gens down if nothing below
                        if *shift == -1 && !below {
                            continue;
                        } else if *shift == 1 && i == j && !above {
                            // Don't move only one gen up if nothing above
                            continue;
                        }

                        if let Some(layout2) = self.move_gens(&to_move, *shift) {
                            layouts.push(layout2);
                        }

                    }

                
                }
            }
        }

        layouts
    }

    fn all_top(&self) -> bool {
        for chip in &self.chips {
            if *chip != self.floors - 1 {
                return false;
            }
        }

        for gen in &self.gens {
            if *gen != self.floors - 1 {
                return false;
            }
        }

        true
    }

    fn move_all_top(&self) -> Option<u32> {
        let mut visited: HashMap<Layout, u32> = HashMap::new();

        let mut nodes: VecDeque<(Layout, u32)> = VecDeque::from([(self.clone(), 0)]);

        let mut shortest: Option<u32> = None;

        while nodes.len() > 0 {
            let (layout, moves) = nodes.pop_front().unwrap();

            if layout.all_top() {
                shortest = Some(moves);
                continue;
            }

            if shortest.is_some() && moves >= shortest.unwrap() {
                continue;
            }

            for layout2 in layout.next_moves() {
                if layout2
                    == (Layout {
                        chips: vec![1, 3, 2, 1, 3],
                        gens: vec![1, 1, 0, 1, 1],
                        elevator: 3,
                        floors: 4,
                    })
                {}
                if !visited.contains_key(&layout2) || visited[&layout2] > moves + 1 {
                    nodes.push_back((layout2.clone(), moves + 1));
                    visited.insert(layout2, moves + 1);
                }
            }
        }

        shortest
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
