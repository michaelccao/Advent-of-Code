use crate::helper::read_data;
use std::collections::HashMap;
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day7.txt");

    let gates: HashMap<&str, Vec<&str>> = get_gates(&data);

    let gate_values: HashMap<&str, u32> = HashMap::new();

    let p1: u32 = get_output("a", &gates, gate_values).0;

    let gate_values2: HashMap<&str, u32> = HashMap::from([("b", p1)]);

    let p2: u32 = get_output("a", &gates, gate_values2).0;

    println!("{p1}\n{p2}");
}

fn get_gates(data: &String) -> HashMap<&str, Vec<&str>> {
    let mut gates: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in data.lines() {
        let config: Vec<&str> = line.split(" -> ").collect();
        let gate: &str = config[1];
        let gate_in: Vec<&str> = config[0].split(" ").collect();

        gates.insert(gate, gate_in);
    }

    gates
}

fn get_output<'a>(
    gate: &'a str,
    gates: &'a HashMap<&str, Vec<&str>>,
    mut gate_values: HashMap<&'a str, u32>,
) -> (u32, HashMap<&'a str, u32>) {
    if gate_values.contains_key(gate) {
        return (gate_values[gate], gate_values);
    }

    let gate_parse: Result<u32, std::num::ParseIntError> = gate.parse::<u32>();

    if gate_parse.is_ok() {
        gate_values.insert(gate, gate_parse.unwrap());
        return (gate_values[gate], gate_values);
    }

    let gate_in: &Vec<&str> = gates.get(gate).unwrap();

    match gate_in.len() {
        1 => {
            let res: (u32, HashMap<&'a str, u32>) = get_output(gate_in[0], gates, gate_values);
            gate_values = res.1;
            let val: u32 = res.0;

            gate_values.insert(gate, val);
        }
        2 => {
            let res: (u32, HashMap<&'a str, u32>) = get_output(gate_in[1], gates, gate_values);
            gate_values = res.1;
            let val: u32 = !res.0;

            gate_values.insert(gate, val);
        }
        3 => {
            let res1: (u32, HashMap<&'a str, u32>) = get_output(gate_in[0], gates, gate_values);
            gate_values = res1.1;
            let val1: u32 = res1.0;

            let res2: (u32, HashMap<&'a str, u32>) = get_output(gate_in[2], gates, gate_values);
            gate_values = res2.1;
            let val2: u32 = res2.0;

            match gate_in[1] {
                "AND" => {
                    gate_values.insert(gate, val1 & val2);
                }
                "OR" => {
                    gate_values.insert(gate, val1 | val2);
                }
                "LSHIFT" => {
                    gate_values.insert(gate, val1 << val2);
                }
                "RSHIFT" => {
                    gate_values.insert(gate, val1 >> val2);
                }
                _ => {}
            };
        }
        _ => {}
    };

    (gate_values[gate], gate_values)
}
