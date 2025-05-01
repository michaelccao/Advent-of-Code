use crate::helper::read_data;
use std::collections::{HashMap, HashSet};

pub fn main() {
    let data: String = read_data("../Data/Day14.txt");

    let recipes: HashMap<String, (u64, Vec<(u64, String)>)> = get_recipes(&data);

    let p1: u64 = ore_cost(&"FUEL".to_string(), 1, &recipes);

    println!("{p1}");

    let p2: u64 = ore_to_fuel(1000000000000, &recipes);

    println!("{p2}");
}

fn get_recipes(data: &String) -> HashMap<String, (u64, Vec<(u64, String)>)> {
    let mut recipes: HashMap<String, (u64, Vec<(u64, String)>)> = HashMap::new();

    for line in data.lines() {
        let mut line = line.split(" => ");

        let inputs: Vec<Vec<&str>> = line
            .next()
            .unwrap()
            .split(", ")
            .map(|ingred| ingred.split(" ").collect())
            .collect();
        let inputs: Vec<(u64, String)> = inputs
            .iter()
            .map(|ingred| (ingred[0].parse::<u64>().unwrap(), ingred[1].to_string()))
            .collect();

        let mut outputs = line.next().unwrap().split(" ");
        let output_amt: u64 = outputs.next().unwrap().parse().unwrap();
        let output_ingred: String = outputs.next().unwrap().to_string();

        recipes.insert(output_ingred, (output_amt, inputs));
    }

    recipes
}

fn which_chemicals_to_breakdown(
    chemicals: &HashSet<String>,
    recipes: &HashMap<String, (u64, Vec<(u64, String)>)>,
) -> HashSet<String> {
    let mut input_chems: HashSet<String> = HashSet::new();

    for chemical in chemicals {
        if chemical == "ORE" {
            continue;
        }
        for input_chem in get_inputs(chemical, recipes) {
            input_chems.insert(input_chem.clone());
        }
    }

    chemicals.difference(&input_chems).cloned().collect()
}

fn get_inputs(
    chemical: &String,
    recipes: &HashMap<String, (u64, Vec<(u64, String)>)>,
) -> HashSet<String> {
    if chemical == "ORE" {
        return HashSet::new();
    }

    let mut inputs: HashSet<String> = HashSet::new();

    for (_, input_chemical) in &recipes[chemical].1 {
        inputs.insert(input_chemical.clone());

        inputs = inputs
            .union(&get_inputs(input_chemical, recipes))
            .cloned()
            .collect();
    }

    inputs
}

fn ore_cost(
    ingredient: &String,
    amt: u64,
    recipes: &HashMap<String, (u64, Vec<(u64, String)>)>,
) -> u64 {
    let mut total: u64 = 0;

    let mut stockpile: HashMap<String, u64> = HashMap::from([(ingredient.clone(), amt)]);

    let mut changed: bool = true;

    while changed {
        changed = false;

        let mut stockpile2: HashMap<String, u64> = HashMap::new();

        let target_chemicals: HashSet<String> =
            which_chemicals_to_breakdown(&stockpile.keys().cloned().collect(), recipes);

        for (chem, amt) in stockpile {
            if chem == "ORE" {
                total += amt;
                continue;
            }

            let recipe_output: u64 = recipes[&chem].0;

            if amt >= recipe_output {
                changed = true;

                let recipe_scale: u64 = amt / recipe_output;

                for (input_amt, input_chem) in &recipes[&chem].1 {
                    if let Some(amt2) = stockpile2.get_mut(input_chem) {
                        *amt2 += input_amt * recipe_scale;
                    } else {
                        stockpile2.insert(input_chem.clone(), input_amt * recipe_scale);
                    }
                }

                let leftover: u64 = amt % recipe_output;

                if leftover > 0 {
                    if let Some(amt2) = stockpile2.get_mut(&chem) {
                        *amt2 += leftover;
                    } else {
                        stockpile2.insert(chem.clone(), leftover);
                    }
                }
            } else if target_chemicals.contains(&chem) {
                changed = true;

                for (input_amt, input_chem) in &recipes[&chem].1 {
                    if let Some(amt2) = stockpile2.get_mut(input_chem) {
                        *amt2 += input_amt;
                    } else {
                        stockpile2.insert(input_chem.clone(), *input_amt);
                    }
                }
            } else {
                if let Some(amt2) = stockpile2.get_mut(&chem) {
                    *amt2 += amt;
                } else {
                    stockpile2.insert(chem.clone(), amt);
                }
            }
        }

        stockpile = stockpile2;
    }

    total
}

fn ore_to_fuel(ore: u64, recipes: &HashMap<String, (u64, Vec<(u64, String)>)>) -> u64 {
    let mut lb: u64 = 1;
    let mut ub: u64 = 100000000;

    while lb < ub - 1 {
        let fuel: u64 = (lb + ub) / 2;

        let ore2: u64 = ore_cost(&"FUEL".to_string(), fuel, recipes);

        if ore2 <= ore {
            lb = fuel;
        } else {
            ub = fuel;
        }
    }

    lb
}
