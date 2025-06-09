use crate::helper::read_data;

pub fn main() {
    let data: String = read_data("../Data/Day02.txt");
    let passwords: Vec<&str> = data.lines().collect();

    let p1: usize = passwords.iter().filter(|&&pass| is_valid_password(pass)).count();

    println!("{p1}");

    let p2: usize = passwords.iter().filter(|&&pass| is_valid_password2(pass)).count();

    println!("{p2}");
}

fn is_valid_password(pass: &str) -> bool {

    let pass: Vec<&str> = pass.split_whitespace().collect();
    let mut count_range = pass[0].split("-");
    let lower: u32 = count_range.next().unwrap().parse().unwrap();
    let upper: u32 = count_range.next().unwrap().parse().unwrap();

    let letter: char = pass[1].chars().next().unwrap();

    let pass: &str = pass[2];

    let mut count: u32 = 0;

    for c in pass.chars() {
        if c == letter {
            count += 1;
        }
    }

    count >= lower && count <= upper
}

fn is_valid_password2(pass: &str) -> bool {

    let pass: Vec<&str> = pass.split_whitespace().collect();
    let mut count_range = pass[0].split("-");
    let lower: usize = count_range.next().unwrap().parse().unwrap();
    let upper: usize = count_range.next().unwrap().parse().unwrap();

    let letter: char = pass[1].chars().next().unwrap();

    let pass: Vec<char> = pass[2].chars().collect();

    (pass[lower-1] == letter) ^ (pass[upper-1] == letter)
}