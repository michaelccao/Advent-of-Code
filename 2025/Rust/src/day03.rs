use crate::helper::read_data;

pub fn main() {
    let data: String = read_data("../Data/Day03.txt");

    let p1: u64 = total_joltage(&data, false);

    println!("{p1}");

    let p2: u64 = total_joltage(&data, true);

    println!("{p2}");
}

fn total_joltage(data: &String, part2: bool) -> u64 {
    let mut total: u64 = 0;

    for line in data.lines() {
        let line: Vec<u64> = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u64)
            .collect();

        let mut nums: Vec<u64> = vec![0; if part2 { 12 } else { 2 }];

        for (i, &num) in line.iter().enumerate() {
            for j in 0..nums.len() {
                if i >= j && line.len() - i >= nums.len() - j {
                    if num > nums[j] {
                        nums[j] = num;
                        for k in j + 1..nums.len() {
                            nums[k] = 0;
                        }
                        break;
                    }
                }
            }
        }

        total += nums.iter().cloned().reduce(|acc, e| 10 * acc + e).unwrap();
    }

    total
}
