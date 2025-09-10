use crate::helper::read_data;
use std::collections::HashMap;

pub fn main() {
    let data: String = read_data("../Data/Day10.txt");
    let lines: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();

    let p1: u32 = lines.iter().map(|line| parse_line(line).0).sum();
    let mut autocomplete_scores: Vec<u64> = lines
        .iter()
        .map(|line| parse_line(line).1)
        .filter(|&score| score > 0)
        .collect();
    autocomplete_scores.sort_unstable();

    let p2: u64 = autocomplete_scores[autocomplete_scores.len() / 2];

    println!("{p1}\n{p2}");
}

fn parse_line(line: &Vec<char>) -> (u32, u64) {
    let closers: HashMap<char, char> =
        HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);

    let scores: HashMap<char, u32> = HashMap::from([
        (')', 3),
        (']', 57),
        ('}', 1197),
        ('>', 25137),
        ('(', 1),
        ('[', 2),
        ('{', 3),
        ('<', 4),
    ]);

    let mut buffer: Vec<char> = Vec::new();

    for c in line {
        if closers.contains_key(c) {
            buffer.push(*c);
        } else if buffer.len() > 0 {
            let last_open: char = buffer.pop().unwrap();
            if closers[&last_open] != *c {
                return (scores[c], 0);
            }
        } else {
            return (scores[c], 0);
        }
    }

    let mut score: u64 = 0;

    while buffer.len() > 0 {
        score *= 5;
        score += scores[&buffer.pop().unwrap()] as u64;
    }

    (0, score)
}
