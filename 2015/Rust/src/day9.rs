use crate::helper::read_data;
use std::collections::{HashMap, HashSet};
use std::vec::Vec;
use itertools::Itertools;

pub fn main() {
    let data: String = read_data("../Data/Day9.txt");
    let (cities, connections): (HashSet<&str>, HashMap<(&str, &str), u32>) = get_connections(&data);

    let (p1, p2): (u32, u32) = shortest_and_longest_trip(&cities, &connections);

    println!("{p1}\n{p2}");

}

fn get_connections(data: &String) -> (HashSet<&str>, HashMap<(&str, &str), u32>) {
    let mut connections: HashMap<(&str, &str), u32> = HashMap::new();
    let mut cities: HashSet<&str> = HashSet::new();

    for line in data.lines() {
        let con: Vec<&str> = line.split(" = ").collect();
        let dist: u32 = con[1].parse().unwrap();
        let cts: Vec<&str> = con[0].split(" to ").collect();

        let (c1, c2): (&str, &str) = (cts[0], cts[1]);

        connections.insert((c1, c2), dist);
        connections.insert((c2, c1), dist);

        cities.insert(c1);
        cities.insert(c2);
    }

    (cities, connections)
}

fn trip_distance(trip: &Vec<&str>, connections: &HashMap<(&str, &str), u32>) -> u32 {
    let mut distance:u32 = 0;

    for i in 0..trip.len() - 1 {
        let c1: &str = trip[i];
        let c2: &str = trip[i+1];

        distance += connections[&(c1, c2)];
    }

    distance
}

fn shortest_and_longest_trip(cities: &HashSet<&str>, connections: &HashMap<(&str, &str), u32>) -> (u32, u32) {
    let routes: Vec<Vec<&str>> = cities.iter().cloned().permutations(cities.len()).collect::<Vec<_>>();

    let route_dists = routes.iter().map(|trip| trip_distance(trip, connections));

    (route_dists.clone().min().unwrap(), route_dists.max().unwrap())

}