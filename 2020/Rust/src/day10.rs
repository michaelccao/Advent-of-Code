use crate::helper::read_data;

pub fn main() {
    let data: String = read_data("../Data/Day10.txt");
    let jolts: Vec<u32> = data.lines().map(|line| line.parse().unwrap()).collect();

    let p1: u32 = count_differences(&jolts);

    println!("{p1}");

    let p2: u64 = count_combinations(&jolts);

    println!("{p2}");
}

fn count_differences(jolts: &Vec<u32>) -> u32 {
    let mut jolts = jolts.clone();

    jolts.sort_unstable();

    let mut one_diff: u32 = 0;
    let mut three_diff: u32 = 1;

    if jolts[0] == 1 {
        one_diff += 1;
    } else if jolts[0] == 3 {
        three_diff += 1;
    }

    for i in 0..jolts.len() - 1 {
        let diff: u32 = jolts[i + 1] - jolts[i];
        if diff == 1 {
            one_diff += 1;
        } else if diff == 3 {
            three_diff += 1;
        }
    }

    one_diff * three_diff
}

fn count_combinations(jolts: &Vec<u32>) -> u64 {
    let mut jolts: Vec<u32> = jolts.clone();

    jolts.sort_unstable();

    let mut dp: Vec<u64> = vec![0; jolts.len()];

    *dp.last_mut().unwrap() = 1;

    for i in 1..jolts.len() {
        let i2 = jolts.len() - i - 1;

        for j in i2 + 1..jolts.len() {
            let diff: u32 = jolts[j] - jolts[i2];

            if diff <= 3 {
                dp[i2] += dp[j];
            } else {
                break;
            }
        }
    }

    let mut combos: u64 = 0;

    for i in 0..jolts.len() {
        if jolts[i] <= 3 {
            combos += dp[i];
        } else {
            break;
        }
    }

    combos
}
