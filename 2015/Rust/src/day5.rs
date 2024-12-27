use crate::helper::read_data;
use std::collections::{HashMap, HashSet};
use std::io::repeat;
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day5.txt");
    let str_list: Vec<&str> = get_strings(&data);

    let p1: u32 = str_list.iter().map(|s| if is_nice(*s) {1} else {0} ).sum();
    let p2: u32 = str_list.iter().map(|s| if is_nice2(*s) {1} else {0} ).sum();

    println!("{p1}\n{p2}");

}

fn get_strings(data: &String) -> Vec<&str> {
    data.lines().collect::<Vec<&str>>()
}

fn is_nice(s: &str) -> bool {
    let vowels: HashSet<char> = HashSet::from([
        'a',
        'e',
        'i',
        'o',
        'u'
    ]);

    let forbidden: HashSet<&str> = HashSet::from([
        "ab",
        "cd",
        "pq",
        "xy"
    ]);

    let mut vowel_count: i32 = 0;
    let mut double_letter: i32 = 0;
    let mut prev_char: char = ' ';

    for c in s.chars() {
        let last_two: String = format!("{prev_char}{c}");

        if forbidden.contains(&last_two[0..]) {
            return false;
        }
        
        if vowels.contains(&c) {
            vowel_count += 1;
        }

        if c == prev_char {
            double_letter += 1;
        }

        prev_char = c;
    }

    if vowel_count >= 3 && double_letter >= 1 {
        return true
    }
    
    false
    
}

fn is_nice2(s: &str) -> bool {
    let mut xyx: bool = false;
    let mut repeat_pair: bool = false;

    let mut pairs: HashMap<String, usize> = HashMap::new();
    let s: Vec<char> = s.chars().collect();
    let num_letters:usize = s.len();

    for i in 0..num_letters {
        let c = s[i];

        if !xyx && i + 2 < num_letters && c == s[i+2] {
            xyx = true;
        }

        if !repeat_pair && i+1 < num_letters {
            let pair: String = format!("{}{}", c, s[i+1]);

            if pairs.contains_key(&pair) {
                let first_ind = *pairs.get(&pair).unwrap();

                if i - first_ind >= 2 {
                    repeat_pair = true;
                }

            } else {
                pairs.insert(pair, i);
            }
        }

        if repeat_pair && xyx {
            return true;
        }
    }

    false
}