use crate::helper::read_data;

pub fn main() {
    let data: String = read_data("../Data/Day07.txt");
    let crabs: Vec<u32> = data.split(",").map(|n| n.parse().unwrap()).collect();

    // Best alignment is around median
    let p1: u32 = align_crabs(&crabs);

    println!("{p1}");

    // Best alignment is around mean
    // Handwave argument: minimizing (n)(n+1) is very similar
    // to minimizing variance which occurs at mean
    let p2: u32 = align_crabs2(&crabs);

    println!("{p2}");
}

fn align_crabs(crabs: &Vec<u32>) -> u32 {
    let mut crabs: Vec<u32> = crabs.clone();

    crabs.sort_unstable();

    if crabs.len() % 2 == 0 {
        let cand1: usize = crabs.len() / 2;
        let cand2: usize = cand1 - 1;

        let mut fuel: u32 = crabs.iter().map(|&c| c.abs_diff(crabs[cand1])).sum();

        fuel = fuel.min(crabs.iter().map(|&c| c.abs_diff(crabs[cand2])).sum());

        return fuel;
    } else {
        return crabs
            .iter()
            .map(|&c| c.abs_diff(crabs[crabs.len() / 2]))
            .sum();
    }
}

fn align_crabs2(crabs: &Vec<u32>) -> u32 {
    let mean: u32 = crabs.iter().sum::<u32>() / crabs.len() as u32;

    let fuel: u32 = crabs
        .iter()
        .map(|&c| c.abs_diff(mean) * (c.abs_diff(mean) + 1) / 2)
        .sum();

    fuel.min(
        crabs
            .iter()
            .map(|&c| c.abs_diff(mean + 1) * (c.abs_diff(mean + 1) + 1) / 2)
            .sum(),
    )
}
