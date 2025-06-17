use crate::helper::read_data;
use std::collections::{HashMap, HashSet};

pub fn main() {
    let data: String = read_data("../Data/Day16.txt");

    let (rules, tickets) = get_rules_and_tickets(&data);

    let p1: u32 = get_invalid(&rules, &tickets);

    println!("{p1}");

    let p2: u64 = order_fields(&rules, &tickets);

    println!("{p2}");
}

fn get_rules_and_tickets(data: &String) -> (HashMap<u32, HashSet<String>>, Vec<Vec<u32>>) {
    let mut rules: HashMap<u32, HashSet<String>> = HashMap::new();
    let mut tickets: Vec<Vec<u32>> = Vec::new();

    for line in data.lines() {
        if line.contains("or") {
            let line: Vec<&str> = line.split(": ").collect();
            let field: String = line[0].to_string();
            let ranges: Vec<Vec<u32>> = line[1].split(" or ").map(|r| r.split("-").map(|num| num.parse().unwrap()).collect()).collect();

            for r in ranges {
                for num in r[0]..r[1]+1 {
                    if let Some(fields) = rules.get_mut(&num) {
                        fields.insert(field.clone());
                    } else {
                        rules.insert(num, HashSet::from([field.clone()]));
                    }
                }
            }            
        } else if line.contains(",") {
            let ticket: Vec<u32> = line.split(",").map(|num| num.parse().unwrap()).collect();
            tickets.push(ticket);
        }
    }

    (rules, tickets)
}

fn get_invalid(rules: &HashMap<u32, HashSet<String>>, tickets: &Vec<Vec<u32>>) -> u32 {
    let mut total_invalid: u32 = 0;

    for ticket in tickets[1..].iter() {
        for num in ticket {
            if !rules.contains_key(num) {
                total_invalid += num;
            }
        }
    }

    total_invalid
}

fn order_fields(rules: &HashMap<u32, HashSet<String>>, tickets: &Vec<Vec<u32>>) -> u64 {

    let mut candidates: Vec<HashSet<String>> = vec![HashSet::new();tickets[0].len()];
    
    for ticket in tickets[1..].iter() {
        let mut ticket_candidates: Vec<HashSet<String>> = Vec::new();

        for num in ticket {
            if let Some(fields) = rules.get(num) {
                ticket_candidates.push(fields.clone());
            } else {
                break;
            }
        }

        if ticket_candidates.len() == ticket.len() {

            for i in 0..candidates.len() {
                if candidates[i].len() == 0 {
                    candidates[i] = ticket_candidates[i].clone();
                } else {
                    candidates[i] = candidates[i].intersection(&ticket_candidates[i]).cloned().collect();
                }
            }

        }
    }

    let mut settled: usize = 0;

    let mut final_candidates: Vec<String> = vec![String::new();candidates.len()];

    while settled < candidates.len() {
        for i in 0..candidates.len() {
            if candidates[i].len() == 1 {
                let locked_field: String = candidates[i].iter().next().unwrap().clone();
                final_candidates[i] = locked_field.clone();
                for j in 0..candidates.len() {
                    candidates[j].remove(&locked_field);
                }
                settled += 1;
                break;
            }
        }
    }

    let mut total: u64 = 1;

    for (i, candidate) in final_candidates.iter().enumerate() {
        if candidate.starts_with("departure") {
            total *= tickets[0][i] as u64
        }
    }

    total
}