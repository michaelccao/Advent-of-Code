use crate::helper::read_data;

use std::vec::Vec;
use std::collections::HashMap;


pub fn main() {
    
    let data = read_data("../Data/Day5.txt");

    let (rules, updates) = get_rules_and_updates(&data);

    println!("{}", p1(&rules, &updates));

    println!("{}", p2(&rules, &updates));
    
}

fn get_rules_and_updates(data: &String) -> (HashMap<(i32, i32), bool>, Vec<Vec<i32>>) {

    let mut rules: HashMap<(i32,i32), bool> = HashMap::new();
    let mut updates: Vec<Vec<i32>> = Vec::new();

    for line in data.split("\n") {
        if line.contains("|") {
            let pages = line.split("|").collect::<Vec<&str>>();

            let p1 = pages[0].trim().parse::<i32>().unwrap();
            let p2 = pages[1].trim().parse::<i32>().unwrap();

            rules.insert((p1, p2), true);
            rules.insert((p2, p1), false);
        }

        if line.contains(",") {
            let pages = line.split(",").map(|p| p.trim().parse::<i32>().unwrap()).collect::<Vec<i32>>();
            
            updates.push(pages);
        }
    }

    return (rules, updates)

}

fn p1(rules: &HashMap<(i32, i32), bool>, updates: &Vec<Vec<i32>>) -> i32 {
    let mut total:i32 = 0;

    'check_update: for pages in updates {
        for i in 0..pages.len() {
            for j in i+1..pages.len() {
                let p1 = pages[i];
                let p2 = pages[j];

                match rules.get(&(p1, p2)) {
                    Some(order) => if !order { continue 'check_update},
                    None => (),
                };
            }
        }

        total += pages[pages.len() / 2];

    }

    total
}

fn p2(rules: &HashMap<(i32, i32), bool>, updates: &Vec<Vec<i32>>) -> i32 {

    let mut total:i32 = 0;

    for pages in updates {

        let mut corrected = false;
        let mut pages = pages.clone();

        for i in 0..pages.len() {
            for j in i+1..pages.len() {
                let p1 = pages[i];
                let p2 = pages[j];

                match rules.get(&(p1, p2)) {
                    Some(order) => if !order {
                        pages.remove(j);
                        pages.insert(i, p2);
                        corrected = true;
                    },
                    None => (),
                };
            }
        }
        
        if corrected {
            total += pages[pages.len() / 2];
        }
        

    }

    total
}