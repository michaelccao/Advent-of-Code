use crate::helper::read_data;

pub fn main() {
    let data: String = read_data("../Data/Day04.txt");
    let pass_range: Vec<u32> = data.split("-").map(|num| num.parse().unwrap()).collect();

    let p1: usize = (pass_range[0]..pass_range[1] + 1)
        .filter(|&pass| is_acceptable_password(pass, false))
        .count();

    println!("{p1}");

    let p2: usize = (pass_range[0]..pass_range[1] + 1)
        .filter(|&pass| is_acceptable_password(pass, true))
        .count();

    println!("{p2}");
}

fn is_acceptable_password(pass: u32, part2: bool) -> bool {
    let mut repeat: bool = false;

    let mut pass: u32 = pass.clone();

    let mut buffer: (u32, u32) = (pass % 10, 1);
    pass /= 10;

    while pass > 0 {
        let digit: u32 = pass % 10;

        if digit == buffer.0 {
            buffer = (digit, buffer.1 + 1);
        } else if digit > buffer.0 {
            return false;
        } else {
            if buffer.1 == 2 {
                repeat = true;
            } else if buffer.1 > 2 && !part2 {
                repeat = true;
            }

            buffer = (digit, 1);
        }

        pass /= 10;
    }

    if buffer.1 == 2 {
        repeat = true;
    } else if buffer.1 > 2 && !part2 {
        repeat = true;
    }

    repeat
}
