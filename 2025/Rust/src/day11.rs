use crate::helper::read_data;
use std::collections::{HashMap, HashSet};

pub fn main() {
    let data: String = read_data("../Data/Day11.txt");

    let paths: HashMap<String, HashSet<String>> = get_paths(&data);

    let p1 = total_routes(&paths, "you", "out", "", HashMap::new()).0;

    println!("{p1}");

    let svr_fft_dac_out = total_routes(&paths, "svr", "fft", "dac", HashMap::new()).0
        * total_routes(&paths, "fft", "dac", "", HashMap::new()).0
        * total_routes(&paths, "dac", "out", "", HashMap::new()).0;

    let svr_dac_fft_out: u64 = total_routes(&paths, "svr", "dac", "fft", HashMap::new()).0
        * total_routes(&paths, "dac", "fft", "", HashMap::new()).0
        * total_routes(&paths, "fft", "out", "", HashMap::new()).0;

    let p2 = svr_fft_dac_out + svr_dac_fft_out;

    println!("{p2}");
}

fn get_paths(data: &String) -> HashMap<String, HashSet<String>> {
    let mut paths: HashMap<String, HashSet<String>> = HashMap::new();

    for line in data.lines() {
        let mut line = line.split(": ");
        let start = line.next().unwrap().to_string();

        let mut ends: HashSet<String> = HashSet::new();

        for end in line.next().unwrap().split_whitespace() {
            ends.insert(end.to_string());
        }

        paths.insert(start, ends);
    }

    paths
}

fn total_routes(
    paths: &HashMap<String, HashSet<String>>,
    start: &str,
    end: &str,
    forbidden: &str,
    mut cache: HashMap<String, u64>,
) -> (u64, HashMap<String, u64>) {
    if let Some(&routes) = cache.get(&start.to_string()) {
        return (routes, cache);
    }

    if start == end {
        return (1, cache);
    }

    if start == "out" {
        return (0, cache);
    }

    if let Some(nodes) = paths.get(&start.to_string()) {
        let mut routes: u64 = 0;
        for node in nodes {
            if node != forbidden {
                let res = total_routes(paths, node, end, forbidden, cache);
                routes += res.0;
                cache = res.1;
            }
        }

        cache.insert(start.to_string(), routes);
    }

    (cache[&start.to_string()], cache)
}
