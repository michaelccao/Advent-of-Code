use crate::helper::read_data;

pub fn main() {
    let data: String = read_data("../Data/Day1.txt");

    let mut floor: i32 = 0;
    let mut p2: usize = 0;

    for (i, c) in data.chars().enumerate() {
        if c == '(' {
            floor += 1;
        } else {
            floor -= 1;
        }

        if floor == -1 && p2 == 0 {
            p2 = i + 1;
        }
    }

    println!("{floor}\n{p2}");
}