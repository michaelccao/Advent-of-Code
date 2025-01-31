use crate::helper::read_data;
use std::collections::HashMap;

pub fn main() {
    let data: String = read_data("../Data/Day13.txt");

    let gates: HashMap<u32, u32> = get_gates(&data);

    let p1: u32 = calculate_total_severity(&gates);
    let p2: u32 = find_timing(&gates);

    println!("{p1}\n{p2}");
}

fn get_gates(data: &String) -> HashMap<u32, u32> {
    let mut gates: HashMap<u32, u32> = HashMap::new();

    for line in data.lines() {
        let mut line = line.trim().split(": ");
        let layer: u32 = line.next().unwrap().parse().unwrap();
        let depth: u32 = line.next().unwrap().parse().unwrap();

        gates.insert(layer, depth);
    }

    gates
}

fn calculate_total_severity(gates: &HashMap<u32, u32>) -> u32 {
    let last_gate: &u32 = gates.keys().max().unwrap();
    let mut severity: u32 = 0;

    for t in 0..last_gate + 1 {
        if let Some(&depth) = gates.get(&t) {
            if t % (2 * depth - 2) == 0 {
                severity += t * depth;
            }
        }
    }

    severity
}

fn find_timing(gates: &HashMap<u32, u32>) -> u32 {
    let last_gate: &u32 = gates.keys().max().unwrap();

    let mut delay: u32 = 0;

    'outer: loop {
        delay += 1;

        for t in 0..last_gate + 1 {
            if let Some(&depth) = gates.get(&t) {
                if (t + delay) % (2 * depth - 2) == 0 {
                    continue 'outer;
                }
            }
        }

        return delay;
    }
}
