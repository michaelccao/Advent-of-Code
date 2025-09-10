use crate::helper::read_data;
use std::collections::HashSet;

pub fn main() {
    let data: String = read_data("../Data/Day09.txt");
    let heights: Vec<Vec<u32>> = data
        .lines()
        .map(|row| row.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let (p1, p2) = low_points(&heights);

    println!("{p1}\n{p2}");
}

fn low_points(heights: &Vec<Vec<u32>>) -> (u32, usize) {
    let mut risk: u32 = 0;
    let mut basins: Vec<usize> = Vec::new();

    for i in 0..heights.len() {
        for j in 0..heights[i].len() {
            if i > 0 && heights[i][j] >= heights[i - 1][j] {
                continue;
            } else if i + 1 < heights.len() && heights[i][j] >= heights[i + 1][j] {
                continue;
            } else if j > 0 && heights[i][j] >= heights[i][j - 1] {
                continue;
            } else if j + 1 < heights[i].len() && heights[i][j] >= heights[i][j + 1] {
                continue;
            } else {
                risk += heights[i][j] + 1;
                basins.push(find_basin(heights, i, j));
            }
        }
    }

    basins.sort_unstable();

    (
        risk,
        basins[basins.len() - 1] * basins[basins.len() - 2] * basins[basins.len() - 3],
    )
}

fn find_basin(heights: &Vec<Vec<u32>>, i: usize, j: usize) -> usize {
    let mut basin: HashSet<(usize, usize)> = HashSet::from([(i, j)]);

    let mut nodes: Vec<(usize, usize)> = vec![(i, j)];

    while nodes.len() > 0 {
        let (i, j) = nodes.pop().unwrap();

        if i > 0
            && heights[i][j] < heights[i - 1][j]
            && heights[i - 1][j] != 9
            && !basin.contains(&(i - 1, j))
        {
            basin.insert((i - 1, j));
            nodes.push((i - 1, j));
        }

        if j > 0
            && heights[i][j] < heights[i][j - 1]
            && heights[i][j - 1] != 9
            && !basin.contains(&(i, j - 1))
        {
            basin.insert((i, j - 1));
            nodes.push((i, j - 1));
        }

        if i + 1 < heights.len()
            && heights[i][j] < heights[i + 1][j]
            && heights[i + 1][j] != 9
            && !basin.contains(&(i + 1, j))
        {
            basin.insert((i + 1, j));
            nodes.push((i + 1, j));
        }

        if j + 1 < heights[i].len()
            && heights[i][j] < heights[i][j + 1]
            && heights[i][j + 1] != 9
            && !basin.contains(&(i, j + 1))
        {
            basin.insert((i, j + 1));
            nodes.push((i, j + 1));
        }
    }

    basin.len()
}
