use crate::helper::read_data;
use std::collections::{HashMap, VecDeque};
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day10.txt");

    let (bots, rules): (
        HashMap<String, VecDeque<u32>>,
        HashMap<String, (String, String)>,
    ) = get_instructions(&data);

    let (bots2, p1) = follow_instructions(&bots, &rules);

    let p2 = bots2[&"output0".to_string()][0]
        * bots2[&"output1".to_string()][0]
        * bots2[&"output2".to_string()][0];

    println!("{p1:?}\n{p2}");
}

fn get_instructions(
    data: &String,
) -> (
    HashMap<String, VecDeque<u32>>,
    HashMap<String, (String, String)>,
) {
    let mut bots: HashMap<String, VecDeque<u32>> = HashMap::new();
    let mut rules: HashMap<String, (String, String)> = HashMap::new();

    for line in data.lines() {
        let line: Vec<&str> = line.trim().split_whitespace().collect();

        if line[0] == "value" {
            let value: u32 = line[1].parse().unwrap();
            let bot: String = line[4..6].join("");

            if let Some(chips) = bots.get_mut(&bot) {
                if value > chips[0] {
                    chips.push_back(value);
                } else {
                    chips.push_front(value);
                }
            } else {
                bots.insert(bot, VecDeque::from([value]));
            }
        } else if line[0] == "bot" {
            let bot: String = line[0..2].join("");
            let low_out: String = line[5..7].join("");
            let high_out: String = line[10..12].join("");

            rules.insert(bot, (low_out, high_out));
        }
    }

    (bots, rules)
}

fn follow_instructions(
    bots: &HashMap<String, VecDeque<u32>>,
    rules: &HashMap<String, (String, String)>,
) -> (HashMap<String, VecDeque<u32>>, Option<String>) {
    let mut bots: HashMap<String, VecDeque<u32>> = bots.clone();

    let mut p1_bot: Option<String> = None;

    while bots.values().map(|chips| chips.len()).max().unwrap() == 2 {
        for (bot, chips) in bots.clone() {
            if chips.len() == 2 && chips[0] == 17 && chips[1] == 61 {
                p1_bot = Some(bot.clone());
            }

            if chips.len() == 2 {
                bots.insert(bot.clone(), VecDeque::new());

                let low = chips[0];
                let high = chips[1];

                let (low_out, high_out) = &rules[&bot];

                if let Some(chips2) = bots.get_mut(low_out) {
                    if chips2.len() > 0 && low > chips2[0] {
                        chips2.push_back(low);
                    } else {
                        chips2.push_front(low);
                    }
                } else {
                    bots.insert(low_out.clone(), VecDeque::from([low]));
                }

                if let Some(chips2) = bots.get_mut(high_out) {
                    if chips2.len() > 0 && high > chips2[0] {
                        chips2.push_back(high);
                    } else {
                        chips2.push_front(high);
                    }
                } else {
                    bots.insert(high_out.clone(), VecDeque::from([high]));
                }
            }
        }
    }

    (bots, p1_bot)
}
