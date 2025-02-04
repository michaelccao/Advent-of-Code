use crate::helper::read_data;
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day15.txt");
    let seeds: Vec<u64> = data
        .lines()
        .map(|line| {
            line.trim()
                .split_whitespace()
                .last()
                .unwrap()
                .parse()
                .unwrap()
        })
        .collect();

    let p1: u32 = count_matches(seeds[0], seeds[1]);
    let p2: u32 = count_matches2(seeds[0], seeds[1]);

    println!("{p1}\n{p2}");
}

fn count_matches(mut a: u64, mut b: u64) -> u32 {
    let mut matches: u32 = 0;

    let bits: u64 = 2_u64.pow(16);

    for _ in 0..40000000 {
        a *= 16807;
        a %= 2147483647;

        b *= 48271;
        b %= 2147483647;

        if (a % bits) == (b % bits) {
            matches += 1;
        }
    }

    matches
}

fn count_matches2(mut a: u64, mut b: u64) -> u32 {
    let mut matches: u32 = 0;

    let bits: u64 = 2_u64.pow(16);

    for _ in 0..5000000 {
        a *= 16807;
        a %= 2147483647;

        while a % 4 != 0 {
            a *= 16807;
            a %= 2147483647;
        }

        b *= 48271;
        b %= 2147483647;

        while b % 8 != 0 {
            b *= 48271;
            b %= 2147483647;
        }

        if (a % bits) == (b % bits) {
            matches += 1;
        }
    }

    matches
}
