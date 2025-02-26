use crate::helper::read_data;
use regex::Regex;
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day03.txt");

    let claims: Vec<(usize, usize, usize, usize)> = get_claims(&data);

    let (p1, p2) = find_overlaps(&claims);

    println!("{p1}\n{p2}");
}

fn get_claims(data: &String) -> Vec<(usize, usize, usize, usize)> {
    let mut claims: Vec<(usize, usize, usize, usize)> = Vec::new();

    let re: Regex = Regex::new(r"#(?:\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();

    for line in data.lines() {
        let Some((_, [j, i, width, height])) = re.captures(line.trim()).map(|caps| caps.extract())
        else {
            continue;
        };

        let j: usize = j.parse().unwrap();
        let i: usize = i.parse().unwrap();

        let width: usize = width.parse().unwrap();
        let height: usize = height.parse().unwrap();

        claims.push((j, i, width, height));
    }

    claims
}

fn find_overlaps(claims: &Vec<(usize, usize, usize, usize)>) -> (u32, usize) {
    let mut tiles: Vec<Vec<u32>> = vec![vec![0; 1000]; 1000];

    let mut unique_claim: usize = 0;

    for &(j, i, width, height) in claims {
        for row in i..i + height {
            for col in j..j + width {
                tiles[row][col] += 1;
            }
        }
    }

    for (id, &(j, i, width, height)) in claims.iter().enumerate() {
        let mut overlap: bool = false;

        for row in i..i + height {
            for col in j..j + width {
                if tiles[row][col] > 1 {
                    overlap = true;
                }
                tiles[row][col] += 1;
            }
        }

        if !overlap {
            unique_claim = id + 1;
        }
    }

    (
        tiles
            .iter()
            .map(|row| {
                row.iter()
                    .map(|&tile| if tile >= 2 { 1 } else { 0 })
                    .sum::<u32>()
            })
            .sum(),
        unique_claim,
    )
}
