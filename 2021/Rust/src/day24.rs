use crate::helper::read_data;

pub fn main() {
    let data: String = read_data("../Data/Day24.txt");
    let instructions: Vec<Vec<&str>> = data
        .lines()
        .map(|line| line.split_whitespace().collect())
        .collect();

    // Instructions have a pattern
    // where the only difference between patterns
    // are some numerical parameters a, b, c

    // final results given some initial w and z

    // if z % 26 + b == w
    // z2 = (z / a)
    // else
    // z2 = (z / a) * 26 + w + c

    // Given w and desired z2:

    // if z % 26 + b == w
    // z = [z2*a..z2*a + a] check condition

    // else
    // z2 - w - c % 26 must = 0
    // z = [(z2 - w - c) / 26 * a..(z2-w-c)/26*a + a] check condition

    let p1: i64 = get_valid_model_numbers(&instructions, false);

    println!("{p1}");

    let p2: i64 = get_valid_model_numbers(&instructions, true);

    println!("{p2}");
}

fn get_valid_model_numbers(instructions: &Vec<Vec<&str>>, part2: bool) -> i64 {
    let mut nodes: Vec<(Vec<i64>, i64)> = Vec::new();

    nodes.push((Vec::new(), 0));

    let mut w_order: Vec<i64> = (1..10).collect();

    if part2 {
        w_order.reverse();
    }

    while nodes.len() > 0 {
        let (mut inputs, z2) = nodes.pop().unwrap();

        if inputs.len() == 14 {
            // Our depth-first-search will always get the largest number or smallest number first
            // depending on how we iterate through w
            inputs.reverse();
            let model_num: i64 = inputs.iter().fold(0, |acc, x| acc * 10 + x);
            return model_num;
        }

        let start: usize = instructions.len() - 18 * inputs.len() - 18;

        let a: i64 = instructions[start + 4][2].parse().unwrap();
        let b: i64 = instructions[start + 5][2].parse().unwrap();
        let c: i64 = instructions[start + 15][2].parse().unwrap();

        for &w in &w_order {
            for z in z2 * a..z2 * a + a {
                if z % 26 + b == w {
                    let mut inputs2: Vec<i64> = inputs.clone();
                    inputs2.push(w);

                    nodes.push((inputs2, z));
                }
            }

            let z3: i64 = z2 - w - c;

            if z3 % 26 == 0 {
                for z in z3 / 26 * a..z3 / 26 * a + a {
                    if z % 26 + b != w {
                        let mut inputs2: Vec<i64> = inputs.clone();
                        inputs2.push(w);

                        nodes.push((inputs2, z));
                    }
                }
            }
        }
    }

    0
}
