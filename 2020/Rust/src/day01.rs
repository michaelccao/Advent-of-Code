use crate::helper::read_data;

pub fn main() {
    let data: String = read_data("../Data/Day01.txt");
    let nums: Vec<u32> = data.lines().map(|line| line.parse().unwrap()).collect();

    let p1: u32 = parse_expense_report(&nums);

    println!("{p1}");

    let p2: u32 = parse_expense_report2(&nums);

    println!("{p2}");
}

fn parse_expense_report(nums: &Vec<u32>) -> u32 {
    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            if nums[i] + nums[j] == 2020 {
                return nums[i] * nums[j];
            }
        }
    }

    0
}

fn parse_expense_report2(nums: &Vec<u32>) -> u32 {
    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            if nums[i] + nums[j] >= 2020 {
                continue;
            }
            for k in j + 1..nums.len() {
                if nums[i] + nums[j] + nums[k] == 2020 {
                    return nums[i] * nums[j] * nums[k];
                }
            }
        }
    }

    0
}
