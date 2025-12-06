use crate::helper::read_data;

pub fn main() {
    let data: String = read_data("../Data/Day06.txt");

    let p1: u64 = calculate(&data);

    println!("{p1}");

    let p2: u64 = calculate2(&data);

    println!("{p2}");
}

fn calculate(data: &String) -> u64 {
    let mut total: u64 = 0;

    let mut nums: Vec<Vec<u64>> = Vec::new();
    let mut ops: Vec<&str> = Vec::new();

    for line in data.lines() {
        let line: Vec<&str> = line.split_whitespace().collect();

        if line[0] == "+" || line[0] == "*" {
            ops = line;
        } else {
            nums.push(line.iter().map(|s| s.parse().unwrap()).collect());
        }
    }

    for (i, &op) in ops.iter().enumerate() {
        let mut subtotal = nums[0][i];
        for j in 1..nums.len() {
            if op == "*" {
                subtotal *= nums[j][i];
            } else if op == "+" {
                subtotal += nums[j][i];
            }
        }
        total += subtotal;
    }

    total
}

fn calculate2(data: &String) -> u64 {
    let mut total: u64 = 0;

    let mut data: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();

    let ops: Vec<char> = data.pop().unwrap();

    let mut current_op: char = '+';
    let mut subtotal: u64 = 0;

    for (i, &c) in ops.iter().enumerate() {
        if c == '+' {
            total += subtotal;

            current_op = '+';
            subtotal = 0;
        } else if c == '*' {
            total += subtotal;

            current_op = '*';
            subtotal = 1;
        }

        let mut num: u64 = 0;
        for j in 0..data.len() {
            let c: char = data[j][i];
            if c == ' ' {
                continue;
            }
            let digit: u64 = c.to_digit(10).unwrap() as u64;
            num *= 10;
            num += digit;
        }

        if num == 0 {
            continue;
        }

        if current_op == '+' {
            subtotal += num;
        } else if current_op == '*' {
            subtotal *= num;
        }
    }

    total + subtotal
}
