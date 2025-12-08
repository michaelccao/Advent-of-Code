use crate::helper::read_data;
use std::collections::{HashMap, HashSet};

pub fn main() {
    let data: String = read_data("../Data/Day08.txt");

    let boxes: Vec<Vec<u64>> = data
        .lines()
        .map(|line| line.split(",").map(|s| s.parse().unwrap()).collect())
        .collect();

    let p1 = connect_boxes(&boxes, false);

    println!("{p1}");

    let p2 = connect_boxes(&boxes, true);

    println!("{p2}");
}

fn connect_boxes(boxes: &Vec<Vec<u64>>, part2: bool) -> usize {
    let mut distances: Vec<(u64, usize, usize)> = Vec::new();

    for i in 0..boxes.len() {
        let b1 = &boxes[i];
        for j in i + 1..boxes.len() {
            let b2 = &boxes[j];

            let distance = (b1[0] - b2[0]).pow(2) + (b1[1] - b2[1]).pow(2) + (b1[2] - b2[2]).pow(2);

            distances.push((distance, i, j));
        }
    }

    distances.sort_unstable();

    let mut graphs: Vec<HashSet<usize>> = Vec::new();
    let mut in_graph: HashMap<usize, usize> = HashMap::new();

    for (pair, &(_d, i, j)) in distances.iter().enumerate() {
        if let Some(&graph) = in_graph.get(&i) {
            if in_graph.get(&j).is_none() {
                graphs[graph].insert(j);
                in_graph.insert(j, graph);

                if graphs[graph].len() == boxes.len() {
                    return (boxes[i][0] * boxes[j][0]) as usize;
                }
            } else if let Some(&graph2) = in_graph.get(&j) {
                if graph2 != graph {
                    for b in graphs[graph2].clone() {
                        graphs[graph].insert(b);
                        in_graph.insert(b, graph);
                        graphs[graph2] = HashSet::new();
                    }

                    if graphs[graph].len() == boxes.len() {
                        return (boxes[i][0] * boxes[j][0]) as usize;
                    }
                }
            }
        } else if let Some(&graph) = in_graph.get(&j) {
            graphs[graph].insert(i);
            in_graph.insert(i, graph);

            if graphs[graph].len() == boxes.len() {
                return (boxes[i][0] * boxes[j][0]) as usize;
            }
        } else {
            let graph: usize = graphs.len();
            graphs.push(HashSet::from([i, j]));
            in_graph.insert(i, graph);
            in_graph.insert(j, graph);
        }

        if !part2 && pair + 1 == 1000 {
            break;
        }
    }

    graphs.sort_unstable_by_key(|e| e.len());
    graphs.reverse();

    graphs[0].len() * graphs[1].len() * graphs[2].len()
}
