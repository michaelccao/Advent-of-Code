use crate::helper::read_data;
use std::collections::{HashMap, HashSet};
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day06.txt");

    let locations = get_locations(&data);

    // A location has infinite area if it can infinitely expand in the 
    // strict North/South/East/West direction
    // Expansion in the North is stopped by a location that is more North
    // Than East/West
    // I.e. 90 degree cone centered on North direction
    // (Anything right on the edge creates ties which also stops infinite area)
    // Therefore a location has finite area if there are locations in 
    // all four quadrants
}

fn get_locations(data: &String) -> Vec<(i32, i32)> {
    let mut locations: Vec<(i32, i32)> = Vec::new();

    for line in data.lines() {
        let mut line = line.trim().split(", ");

        let x: i32 = line.next().unwrap().parse().unwrap();
        let y: i32 = line.next().unwrap().parse().unwrap();

        locations.push((x,y));
    }

    locations
}