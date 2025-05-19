use crate::helper::read_data;
use std::collections::{HashMap, HashSet};

pub fn main() {
    let data: String = read_data("../Data/Day22.txt");
    let shuffles: Vec<Vec<&str>> = data.lines().map(|line| line.split_whitespace().collect()).collect();

    // With deck of size D:
    // Deal into new deck: index i shifts to D-i-1
    // Cut N: index i shifts to (i + D - N) % D
    // Increment N: index i shifts to i*N % D

    let d: i64 = 10007;

    let p1 = execute_shuffles(&shuffles, 2019, d);

    println!("{p1}");

    

    // Every shuffle is a linear operation
    // So the final result is still a linear operation a*i + b

    let d2: i64 = 119_315_717_514_047;
    let num_shuffles: i64 = 101_741_582_076_661;

    let offset = undo_shuffles(&shuffles, 0, d2);
    let scale = (undo_shuffles(&shuffles, 1, d2) - offset + d2) % d2;

    for i in 0..100 {
        assert!(undo_shuffles(&shuffles, i, d2) == (scale*i + offset) % d2);
    }

    // Problem: scale*d2 easily overflows
    // Still don't have fast way to calculate all shuffles

    



}

fn execute_shuffles(shuffles: &Vec<Vec<&str>>, mut i: i64, d: i64) -> i64 {

    for shuffle in shuffles {
        if shuffle[0] == "deal" && shuffle[1] == "into" {
            i = d - i - 1;
        } else if shuffle[0] == "deal" && shuffle[1] == "with" {
            let inc: i64 = shuffle.last().unwrap().parse().unwrap();
            i = i * inc % d;
        } else if shuffle[0] == "cut" {
            let cut: i64 = shuffle.last().unwrap().parse().unwrap();
            i = (i + d - cut) % d;
        }
    }

    i
}

fn undo_shuffles(shuffles: &Vec<Vec<&str>>, mut i: i64, d: i64) -> i64 {
    for s in 0..shuffles.len() {
        let shuffle: &Vec<&str> = &shuffles[shuffles.len() - s - 1];

        if shuffle[0] == "deal" && shuffle[1] == "into" {
            i = d - i - 1;
        } else if shuffle[0] == "deal" && shuffle[1] == "with" {
            let inc: i64 = shuffle.last().unwrap().parse().unwrap();
            while i % inc != 0 {
                i += d;
            }
            i = i / inc;
        } else if shuffle[0] == "cut" {
            let cut: i64 = shuffle.last().unwrap().parse().unwrap();
            i = (i + d + cut) % d;
        }
    }

    i
}