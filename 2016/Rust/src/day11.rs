use crate::helper::read_data;
use std::collections::{HashMap, HashSet};
use std::vec::Vec;
use regex::Regex;

pub fn main() {
    let data: String = read_data("../Data/Day11.txt");

    let (floors, components) = get_floors(&data);

    let mut floors2: Vec<Vec<(&str, &str)>> = floors.clone();

    floors2[1].push(("elerium", "generator"));
    floors2[1].push(("elerium", "microchip"));
    floors2[1].push(("dilithium", "generator"));
    floors2[1].push(("dilithium", "microchip"));

    floors2[1].sort();


    let p1: u32 = shortest_moves(&floors);
    println!("{p1}");
    let p2: u32 = shortest_moves(&floors2);

    println!("{p2}");

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

fn valid_moves<'a>(floors: &Vec<Vec<(&'a str, &'a str)>>, elevator: usize) -> Vec<(Vec<Vec<(&'a str, &'a str)>>, usize)> {

    let mut moves: Vec<(Vec<Vec<(&str, &str)>>, usize)> = Vec::new();

    let num_comps: usize = floors[elevator].len();

    for i in 0..num_comps {
        for j in i..num_comps {
                        
            if elevator < floors.len() - 1 {
                let mut move_up: Vec<Vec<(&str, &str)>> = floors.clone();

                if j != i {
                    let comp: (&str, &str) = move_up[elevator].remove(j);
                    move_up[elevator+1].push(comp);
                }
                let comp: (&str, &str) = move_up[elevator].remove(i);
                move_up[elevator+1].push(comp);

                move_up[elevator+1].sort();

                if is_valid(&move_up) {
                    moves.push((move_up, elevator+1));
                }
            }

            if elevator > 0 {
                let mut move_down: Vec<Vec<(&str, &str)>> = floors.clone();

                if j != i {
                    let comp: (&str, &str) = move_down[elevator].remove(j);
                    move_down[elevator-1].push(comp);
                }
                let comp: (&str, &str) = move_down[elevator].remove(i);
                move_down[elevator-1].push(comp);

                move_down[elevator-1].sort();

                if is_valid(&move_down) {
                    moves.push((move_down, elevator-1));
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
    let mut shortest: u32 = (num_components*4*floors.len()) as u32;

    while nodes.len() > 0 {

        let (floors, elevator, moves) = nodes.pop().unwrap();

        visited.insert((floors.clone(), elevator), moves);


        if floors[floors.len()-1].len() == num_components {
            shortest = moves;
            continue;
        }

        if moves >= shortest {
            continue;
        }


        for (floors2, elevator2) in valid_moves(&floors, elevator) {

            if !visited.contains_key(&(floors2.clone(), elevator2)) || visited[&(floors2.clone(), elevator2)] > moves+1 {
                nodes.push((floors2, elevator2, moves+1))
            }
            

        }
    }

    shortest

}