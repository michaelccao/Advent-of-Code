use crate::helper::read_data;
use std::collections::HashMap;

pub fn main() {
    let data: String = read_data("../Data/Day07.txt");

    let rules: HashMap<String, HashMap<String, u32>> = get_rules(&data);

    let p1: usize = rules
        .keys()
        .filter(|bag| contains_shiny_gold(bag, &rules, HashMap::new()).0)
        .count();

    println!("{p1}");

    // Answer wants bags inside shiny gold bag, so we don't count the gold bag itself
    let p2: u32 = total_bags(&"shiny gold".to_string(), &rules) - 1;

    println!("{p2}");
}

fn get_rules(data: &String) -> HashMap<String, HashMap<String, u32>> {
    let mut rules: HashMap<String, HashMap<String, u32>> = HashMap::new();

    for rule in data.lines() {
        let mut rule = rule.split(" contain ");
        let container: String = rule
            .next()
            .unwrap()
            .split(" bags")
            .next()
            .unwrap()
            .to_string();

        rules.insert(container.clone(), HashMap::new());

        let bags = rule.next().unwrap().split_whitespace();

        let mut amount: u32 = 0;
        let mut color: String = String::new();

        for word in bags {
            if word == "no" {
                break;
            }

            if amount == 0 {
                amount = word.parse().unwrap();
            } else if word.contains("bag") {
                color = color.trim().to_string();

                rules
                    .get_mut(&container)
                    .unwrap()
                    .insert(color.clone(), amount);
                color = String::new();
                amount = 0;
            } else {
                color = format!("{color}{word} ");
            }
        }
    }

    rules
}

fn contains_shiny_gold(
    bag: &String,
    rules: &HashMap<String, HashMap<String, u32>>,
    mut cache: HashMap<String, bool>,
) -> (bool, HashMap<String, bool>) {
    if let Some(&res) = cache.get(bag) {
        return (res, cache);
    }

    if bag == "shiny gold" {
        return (false, cache);
    }

    if let Some(bags) = rules.get(bag) {
        if bags.contains_key(&"shiny gold".to_string()) {
            cache.insert(bag.clone(), true);
            return (true, cache);
        } else {
            for bag in bags.keys() {
                let res = contains_shiny_gold(bag, rules, cache);
                cache = res.1;

                if res.0 {
                    cache.insert(bag.clone(), true);
                    return (true, cache);
                }
            }

            cache.insert(bag.clone(), false);
            return (false, cache);
        }
    } else {
        return (false, cache);
    }
}

fn total_bags(bag: &String, rules: &HashMap<String, HashMap<String, u32>>) -> u32 {
    if let Some(bags) = rules.get(bag) {
        let mut total: u32 = 1;

        for (bag, amt) in bags {
            total += amt * total_bags(bag, rules);
        }

        return total;
    } else {
        return 1;
    }
}
