use crate::helper::read_data;

pub fn main() {
    let data: String = read_data("../Data/Day18.txt");
    let expressions: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();

    let p1: u64 = expressions.iter().map(|exp| eval(exp)).sum();

    println!("{p1}");

    let p2: u64 = expressions.iter().map(|exp| adv_eval(exp)).sum();

    println!("{p2}");
}

fn eval(expression: &Vec<char>) -> u64 {
    let mut total: u64 = 0;

    let mut op: char = '+';

    let mut pointer: usize = 0;

    while pointer < expression.len() {
        let c: char = expression[pointer];
        if c == '+' || c == '*' {
            op = c;
            pointer += 2;
        } else if c.is_numeric() {
            let val: u64 = c.to_digit(10).unwrap() as u64;
            if op == '+' {
                total += val;
            } else if op == '*' {
                total *= val;
            }
            pointer += 2;
        } else if c == '(' {
            let mut level: i32 = -1;
            pointer += 1;
            let start: usize = pointer;

            while level != 0 {
                let c2: char = expression[pointer];
                if c2 == '(' {
                    level -= 1;
                } else if c2 == ')' {
                    level += 1;
                }
                pointer += 1;
            }

            let val: u64 = eval(&expression[start..pointer - 1].iter().cloned().collect());

            if op == '+' {
                total += val;
            } else if op == '*' {
                total *= val;
            }

            pointer += 1;
        }
    }

    total
}

fn adv_eval(expression: &Vec<char>) -> u64 {
    let mut adv_expression: Vec<char> = expression.clone();

    let mut pointer: usize = 0;

    while pointer < adv_expression.len() {
        let c: char = adv_expression[pointer];

        if c == '+' {
            if adv_expression[pointer - 2].is_numeric() {
                adv_expression.insert(pointer - 2, '(');
                pointer += 1;
            } else if adv_expression[pointer - 2] == ')' {
                let mut level: i32 = 1;
                let mut pointer2: usize = pointer - 3;
                while level != 0 {
                    if adv_expression[pointer2] == '(' {
                        level -= 1;
                    } else if adv_expression[pointer2] == ')' {
                        level += 1;
                    }
                    pointer2 -= 1;
                }
                adv_expression.insert(pointer2 + 1, '(');
                pointer += 1;
            }

            if adv_expression[pointer + 2].is_numeric() {
                adv_expression.insert(pointer + 3, ')');
                pointer += 3;
            } else if adv_expression[pointer + 2] == '(' {
                let mut level: i32 = -1;
                let mut pointer2: usize = pointer + 3;
                while level != 0 {
                    if adv_expression[pointer2] == '(' {
                        level -= 1;
                    } else if adv_expression[pointer2] == ')' {
                        level += 1;
                    }
                    pointer2 += 1;
                }
                adv_expression.insert(pointer2, ')');
                pointer += 1;
            }
        } else {
            pointer += 1;
        }
    }

    eval(&adv_expression)
}
