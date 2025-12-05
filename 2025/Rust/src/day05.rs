use crate::helper::read_data;

pub fn main() {
    let data: String = read_data("../Data/Day05.txt");

    let (p1, p2) = get_fresh(&data);

    println!("{p1}\n{p2}");
}

fn get_fresh(data: &String) -> (usize, u64) {
    let mut total: usize = 0;

    let mut fresh: Vec<(u64, u64)> = Vec::new();

    for line in data.lines() {
        if line.len() == 0 {
            continue;
        }

        if line.contains("-") {
            let fresh_range: Vec<u64> = line.split("-").map(|s| s.parse().unwrap()).collect();

            fresh.push((fresh_range[0], fresh_range[1]));
        } else {
            let ingredient: u64 = line.parse().unwrap();

            for &(lower, upper) in &fresh {
                if ingredient >= lower && ingredient <= upper {
                    total += 1;
                    break;
                }
            }
        }
    }

    for i in 0..fresh.len() {
        let (x0, x1) = fresh[i];
        for j in i + 1..fresh.len() {
            let (y0, y1) = fresh[j];

            if x0 <= y0 && y0 <= x1 {
                if y1 > x1 {
                    fresh[j] = (x1 + 1, y1);
                } else {
                    fresh[j] = (1, 0)
                }
            } else if x0 <= y1 && y1 <= x1 {
                if y0 < x0 {
                    fresh[j] = (y0, x0 - 1);
                } else {
                    fresh[j] = (1, 0)
                }
            }
        }
    }

    (
        total,
        fresh
            .iter()
            .cloned()
            .fold(0, |acc, (lower, upper)| acc + (upper - lower + 1)),
    )
}
