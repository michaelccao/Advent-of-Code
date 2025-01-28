use crate::helper::read_data;
use std::collections::{HashMap, HashSet};
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day12.txt");
    let connections: HashMap<u32, HashSet<u32>> = get_connections(&data);

    let p1: usize = get_subgraph_size(0, &connections);
    let p2: u32 = get_groups(&connections);

    println!("{p1}\n{p2}");
}

fn get_connections(data: &String) -> HashMap<u32, HashSet<u32>> {
    let mut graph: HashMap<u32, HashSet<u32>> = HashMap::new();

    for line in data.lines() {
        let mut line = line.trim().split(" <-> ");

        let node: u32 = line.next().unwrap().parse().unwrap();

        let connections: Vec<&str> = line.next().unwrap().split(", ").collect();

        for connection in connections {
            let connection: u32 = connection.parse().unwrap();
            if let Some(cons) = graph.get_mut(&node) {
                cons.insert(connection);
            } else {
                graph.insert(node, HashSet::from([connection]));
            }

            if let Some(cons) = graph.get_mut(&connection) {
                cons.insert(node);
            } else {
                graph.insert(connection, HashSet::from([node]));
            }
        }
    }

    graph
}

fn get_subgraph_size(node: u32, connections: &HashMap<u32, HashSet<u32>>) -> usize {
    let mut subgraph: HashSet<u32> = HashSet::new();
    subgraph.insert(node);

    let mut nodes: Vec<u32> = vec![node];

    while nodes.len() > 0 {
        let node: u32 = nodes.pop().unwrap();

        for &node2 in &connections[&node] {
            if !subgraph.contains(&node2) {
                subgraph.insert(node2);

                nodes.push(node2);
            }
        }
    }

    subgraph.len()
}

fn get_groups(connections: &HashMap<u32, HashSet<u32>>) -> u32 {
    let mut groups: u32 = 0;

    let mut visited: HashSet<u32> = HashSet::new();

    for &key in connections.keys() {
        if visited.contains(&key) {
            continue;
        }

        let mut nodes: Vec<u32> = vec![key];

        groups += 1;

        while nodes.len() > 0 {
            let node: u32 = nodes.pop().unwrap();

            for &node2 in &connections[&node] {
                if !visited.contains(&node2) {
                    visited.insert(node2);
                    nodes.push(node2);
                }
            }
        }
    }

    groups
}
