use crate::helper::read_data;
use std::collections::{HashMap, HashSet, VecDeque};

pub fn main() {
    let data: String = read_data("../Data/Day19.txt");

    let (rules, terminates, messages) = get_rules_and_messages(&data);

    let (valid_strings, cache) = decode_rule(&rules, &terminates, 0, HashMap::new());

    let p1: usize = messages
        .iter()
        .filter(|&msg| valid_strings.contains(msg))
        .count();

    println!("{p1}");

    // Only rule 0 uses rule 8 and rule 11 which translates to 42 42 31

    // Part 2 is all strings that are 42's then 31's and number of 31's should be less than 42's

    let rule_42: &HashSet<String> = cache.get(&42).unwrap();
    let rule_31: &HashSet<String> = cache.get(&31).unwrap();

    let p2 = messages
        .iter()
        .filter(|&msg| is_valid_string(msg, rule_42, rule_31))
        .count();

    println!("{p2}");
}

fn get_rules_and_messages(
    data: &String,
) -> (HashMap<u32, Vec<Vec<u32>>>, HashMap<u32, char>, Vec<String>) {
    let mut rules: HashMap<u32, Vec<Vec<u32>>> = HashMap::new();
    let mut messages: Vec<String> = Vec::new();
    let mut terminates: HashMap<u32, char> = HashMap::new();

    for line in data.lines() {
        if line.contains(":") {
            let line: Vec<&str> = line.split(": ").collect();
            let rule_num: u32 = line[0].parse().unwrap();

            if line[1].contains('"') {
                terminates.insert(rule_num, line[1].trim_matches('"').chars().next().unwrap());
            } else {
                let rule: Vec<Vec<u32>> = line[1]
                    .split(" | ")
                    .map(|branch| branch.split(" ").map(|num| num.parse().unwrap()).collect())
                    .collect();
                rules.insert(rule_num, rule);
            }
        } else if line.len() > 0 {
            messages.push(line.to_string());
        }
    }

    (rules, terminates, messages)
}

fn decode_rule(
    rules: &HashMap<u32, Vec<Vec<u32>>>,
    terminates: &HashMap<u32, char>,
    rule_num: u32,
    mut cache: HashMap<u32, HashSet<String>>,
) -> (HashSet<String>, HashMap<u32, HashSet<String>>) {
    if let Some(valid_strings) = cache.get(&rule_num) {
        return (valid_strings.clone(), cache);
    }

    let mut valid_strings: HashSet<String> = HashSet::new();

    for branch in &rules[&rule_num] {
        let mut branch_strings: HashSet<String> = HashSet::from([String::new()]);

        for rule_num in branch {
            let mut branch_strings2: HashSet<String> = HashSet::new();
            if let Some(&c) = terminates.get(&rule_num) {
                for s in &branch_strings {
                    let s2: String = format!("{s}{c}");
                    branch_strings2.insert(s2);
                }
            } else {
                let res = decode_rule(rules, terminates, *rule_num, cache);
                let suffixes: HashSet<String> = res.0;
                cache = res.1;

                for s in &branch_strings {
                    for s2 in &suffixes {
                        branch_strings2.insert(format!("{s}{s2}"));
                    }
                }
            }

            branch_strings = branch_strings2;
        }

        valid_strings = valid_strings.union(&branch_strings).cloned().collect();
    }

    cache.insert(rule_num, valid_strings.clone());

    return (valid_strings, cache);
}

fn is_valid_string(s: &String, prefixes: &HashSet<String>, suffixes: &HashSet<String>) -> bool {
    let mut nodes: VecDeque<(String, u32, u32)> = VecDeque::from([(s.clone(), 0, 0)]);

    while nodes.len() > 0 {
        let (s, pre, suff) = nodes.pop_back().unwrap();

        if s.len() == 0 && suff >= 1 && pre > suff {
            return true;
        }

        if pre < 2 {
            for prefix in prefixes {
                if s.starts_with(prefix) {
                    let s2: String = s.strip_prefix(prefix).unwrap().to_string();
                    nodes.push_back((s2, pre + 1, suff));
                }
            }
        } else if suff > 0 && suff + 1 < pre {
            for suffix in suffixes {
                if s.starts_with(suffix) {
                    let s2: String = s.strip_prefix(suffix).unwrap().to_string();
                    nodes.push_back((s2, pre, suff + 1));
                }
            }
        } else if pre >= 2 && suff == 0 {
            for prefix in prefixes {
                if s.starts_with(prefix) {
                    let s2: String = s.strip_prefix(prefix).unwrap().to_string();
                    nodes.push_back((s2, pre + 1, suff));
                }
            }

            for suffix in suffixes {
                if s.starts_with(suffix) {
                    let s2: String = s.strip_prefix(suffix).unwrap().to_string();
                    nodes.push_back((s2, pre, suff + 1));
                }
            }
        }
    }

    false
}
