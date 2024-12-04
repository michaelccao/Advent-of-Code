use crate::helper::read_data;
use regex::Regex;

pub fn main() {

    let data = read_data("../Data/Day3.txt");

    println!("{}", p1(&data));

    println!("{}", p2(&data));

}

pub fn p1(data: &String) -> i32 {
    let mut total:i32 = 0;

    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let ops = re.captures_iter(data);

    for op in ops {
        total += op[1].parse::<i32>().unwrap() * op[2].parse::<i32>().unwrap();
    }

    total
}

pub fn p2(data: &String) -> i32 {
    let mut total:i32 = 0;
    let mut do_flag:bool = true;

    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();

    let ops = re.captures_iter(data);

    for op in ops {
        match &op[0] {
            "don't()" => do_flag = false,
            "do()" => do_flag = true,
            _ => {
                if do_flag {
                    total += op[1].parse::<i32>().unwrap() * op[2].parse::<i32>().unwrap();
                }
            }
        }
    }

    total
}