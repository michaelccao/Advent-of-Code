use crate::helper::read_data;
use std::collections::{HashMap, HashSet};

pub fn main() {
    let data: String = read_data("../Data/Day12.txt");

    let graph: HashMap<String, HashSet<String>> = get_graph(&data);

    let p1: usize = get_paths(&graph, false);

    println!("{p1}");

    let p2: usize = get_paths(&graph, true);

    println!("{p2}");
}

fn get_graph(data: &String) -> HashMap<String, HashSet<String>> {
    let mut graph: HashMap<String, HashSet<String>> = HashMap::new();

    for line in data.lines() {
        let mut line = line.split("-");
        let start: String = line.next().unwrap().to_string();
        let end: String = line.next().unwrap().to_string();

        if let Some(connections) = graph.get_mut(&start) {
            connections.insert(end.clone());
        } else {
            graph.insert(start.clone(), HashSet::from([end.clone()]));
        }

        if let Some(connections) = graph.get_mut(&end) {
            connections.insert(start.clone());
        } else {
            graph.insert(end.clone(), HashSet::from([start.clone()]));
        }
    }

    graph
}

fn get_paths(graph: &HashMap<String, HashSet<String>>, part2: bool) -> usize {
    let mut nodes: Vec<(String, HashSet<String>, bool)> = vec![(
        "start".to_string(),
        HashSet::from(["start".to_string()]),
        true,
    )];

    let mut paths: usize = 0;

    while nodes.len() > 0 {
        let (origin, visited, can_revisit) = nodes.pop().unwrap();

        if origin == "end" {
            paths += 1;
            continue;
        }

        for dest in &graph[&origin] {
            if *dest == dest.to_lowercase() {
                if !visited.contains(dest) {
                    let mut visited2: HashSet<String> = visited.clone();
                    visited2.insert(dest.clone());

                    nodes.push((dest.clone(), visited2, can_revisit));
                } else if visited.contains(dest) && can_revisit && part2 && dest != "start" {
                    nodes.push((dest.clone(), visited.clone(), false));
                }
            } else if *dest == dest.to_uppercase() {
                nodes.push((dest.clone(), visited.clone(), can_revisit));
            }
        }
    }

    paths
}
