use crate::helper::read_data;

pub fn main() {
    let data: String = read_data("../Data/Day09.txt");

    let (p1, p2): (u32, u32) = score(&data);

    println!("{p1}\n{p2}");
}

fn score(data: &String) -> (u32, u32) {
    let mut level: u32 = 0;

    let mut garbage: bool = false;
    let mut ignore: bool = false;

    let mut score: u32 = 0;

    let mut garbage_collected: u32 = 0;

    for c in data.chars() {
        if ignore {
            ignore = false;
            continue;
        }

        match c {
            '{' => {
                if !garbage {
                    level += 1
                } else {
                    garbage_collected += 1
                }
            }
            '}' => {
                if !garbage {
                    score += level;
                    level -= 1
                } else {
                    garbage_collected += 1
                }
            }
            '<' => {
                if !garbage {
                    garbage = true
                } else {
                    garbage_collected += 1
                }
            }
            '>' => garbage = false,
            '!' => ignore = true,
            _ => {
                if garbage {
                    garbage_collected += 1
                }
            }
        };
    }

    (score, garbage_collected)
}
