use crate::helper::read_data;

pub fn main() {
    let data: String = read_data("../Data/Day01.txt");

    let depths: Vec<u32> = data.lines().map(|line| line.parse().unwrap()).collect();

    let p1: u32 = count_increases(&depths, 1);

    println!("{p1}");

    let p2: u32 = count_increases(&depths, 3);

    println!("{p2}");
}

fn count_increases(depths: &Vec<u32>, offset: usize) -> u32 {
    let mut increases = 0;
    let mut prev: u32 = depths[0];

    for i in offset..depths.len() {
        if depths[i] > prev {
            increases += 1;
        }
        prev = depths[i - offset + 1];
    }

    increases
}
