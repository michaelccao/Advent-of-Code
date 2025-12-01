use crate::helper::read_data;

pub fn main() {
    let data: String = read_data("../Data/Day01.txt");

    let p1: u32 = get_password(&data, false);
    println!("{p1}");

    let p2: u32 = get_password(&data, true);
    println!("{p2}");
}

fn get_password(data: &String, part2: bool) -> u32 {
    let mut dial: u32 = 50;

    let mut password: u32 = 0;

    for line in data.lines() {
        let mut spin: u32 = line[1..].parse().unwrap();

        if part2 {
            password += spin / 100
        }

        spin %= 100;

        if line.starts_with('R') {
            if spin + dial > 100 && part2 {
                password += 1;
            }
        } else if line.starts_with('L') {
            if spin > dial && dial != 0 && part2 {
                password += 1;
            }

            spin = 100 - spin;
        }

        dial = (dial + spin) % 100;

        if dial == 0 {
            password += 1;
        }
    }

    password
}
