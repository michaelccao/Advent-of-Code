use crate::helper::read_data;

pub fn main() {
    let data: String = read_data("../Data/Day02.txt");
    let codes: Vec<usize> = data
        .split(",")
        .map(|num| num.parse::<usize>().unwrap())
        .collect();

    let p1: usize = run_program(&codes, 12, 2);

    println!("{p1}");

    let p2: usize = find_noun_verb(&codes, 19690720);

    println!("{p2}");
}

fn run_program(codes: &Vec<usize>, noun: usize, verb: usize) -> usize {
    let mut codes: Vec<usize> = codes.clone();

    codes[1] = noun;
    codes[2] = verb;

    let mut pointer: usize = 0;

    while codes[pointer] != 99 {
        let a: usize = codes[codes[pointer + 1]];
        let b: usize = codes[codes[pointer + 2]];

        let store: usize = codes[pointer + 3];

        if codes[pointer] == 1 {
            codes[store] = a + b;
        } else if codes[pointer] == 2 {
            codes[store] = a * b;
        }

        pointer += 4;
    }

    codes[0]
}

fn find_noun_verb(codes: &Vec<usize>, target: usize) -> usize {
    for noun in 0..100 {
        for verb in 0..100 {
            if run_program(codes, noun, verb) == target {
                return 100 * noun + verb;
            }
        }
    }

    0
}
