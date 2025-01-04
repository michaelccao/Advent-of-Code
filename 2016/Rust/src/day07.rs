use crate::helper::read_data;
use regex::Regex;
use std::collections::{HashSet, VecDeque};
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day07.txt");
    let ips: Vec<&str> = data.lines().map(|line| line.trim()).collect();

    let p1: usize = ips
        .iter()
        .filter(|ip| supports_tls(ip.to_owned()))
        .collect::<Vec<_>>()
        .len();
    let p2: usize = ips
        .iter()
        .filter(|ip| supports_ssl(ip.to_owned()))
        .collect::<Vec<_>>()
        .len();

    println!("{p1}\n{p2}");
}

fn supports_tls(ip: &str) -> bool {
    let re = Regex::new(r"[^\[\]]*").unwrap();

    let mut good_abba: bool = false;

    for m in re.find_iter(ip) {
        let abba: bool = has_abba(m.as_str());

        if m.start() > 0 && &ip[m.start() - 1..m.start()] == "[" {
            if abba {
                return false;
            }
        } else if abba {
            good_abba = true;
        }
    }
    good_abba
}

fn supports_ssl(ip: &str) -> bool {
    let re = Regex::new(r"[^\[\]]*").unwrap();

    let mut abas: HashSet<(char, char)> = HashSet::new();
    let mut babs: HashSet<(char, char)> = HashSet::new();

    for m in re.find_iter(ip) {
        let xyxs: Vec<(char, char)> = find_abas(m.as_str());

        if m.start() > 0 && &ip[m.start() - 1..m.start()] == "[" {
            xyxs.iter().for_each(|(a, b)| {
                babs.insert((*b, *a));
            });
        } else {
            xyxs.iter().for_each(|(a, b)| {
                abas.insert((*a, *b));
            })
        };
    }

    abas.intersection(&babs).next().is_some()
}

fn has_abba(s: &str) -> bool {
    let mut buffer: VecDeque<char> = VecDeque::new();

    for c in s.chars() {
        buffer.push_back(c);

        if buffer.len() > 4 {
            buffer.pop_front();
        }

        if buffer.len() == 4 {
            if buffer[0] == buffer[3] && buffer[1] == buffer[2] && buffer[0] != buffer[1] {
                return true;
            }
        }
    }

    false
}

fn find_abas(s: &str) -> Vec<(char, char)> {
    let mut abas: Vec<(char, char)> = Vec::new();

    let mut buffer: VecDeque<char> = VecDeque::new();

    for c in s.chars() {
        buffer.push_back(c);

        if buffer.len() > 3 {
            buffer.pop_front();
        }

        if buffer.len() == 3 {
            if buffer[0] == buffer[2] && buffer[0] != buffer[1] {
                abas.push((buffer[0], buffer[1]));
            }
        }
    }

    abas
}
