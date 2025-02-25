use crate::helper::read_data;
use std::collections::{HashMap, VecDeque};
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day25.txt");

    let (start_state, steps, instructions) = get_instructions(&data);

    let p1: u64 = follow_instructions(start_state, steps, &instructions);

    println!("{p1}");
}

fn get_instructions(
    data: &String,
) -> (char, u64, HashMap<char, (u64, bool, char, u64, bool, char)>) {
    let mut instructions: HashMap<char, (u64, bool, char, u64, bool, char)> = HashMap::new();

    let lines: Vec<Vec<&str>> = data
        .lines()
        .map(|line| line.trim().split_whitespace().collect())
        .collect();

    let start_state: char = lines[0][3].chars().next().unwrap();

    let steps: u64 = lines[1][5].parse().unwrap();

    let num_blueprints: usize = (lines.len() - 2) / 10;

    for i in 0..num_blueprints {
        let base_line: usize = 3 + 10 * i;
        let state: char = lines[base_line][2].chars().next().unwrap();

        let zero_write: u64 = lines[base_line + 2][4]
            .trim_end_matches(".")
            .parse()
            .unwrap();
        let zero_move: bool = lines[base_line + 3][6] == "right.";
        let zero_state: char = lines[base_line + 4][4].chars().next().unwrap();

        let one_write: u64 = lines[base_line + 6][4]
            .trim_end_matches(".")
            .parse()
            .unwrap();
        let one_move: bool = lines[base_line + 7][6] == "right.";
        let one_state: char = lines[base_line + 8][4].chars().next().unwrap();

        instructions.insert(
            state,
            (
                zero_write, zero_move, zero_state, one_write, one_move, one_state,
            ),
        );
    }

    (start_state, steps, instructions)
}

fn follow_instructions(
    start_state: char,
    steps: u64,
    instructions: &HashMap<char, (u64, bool, char, u64, bool, char)>,
) -> u64 {
    let mut pointer: usize = 0;
    let mut tape: VecDeque<u64> = VecDeque::from([0]);

    let mut state: char = start_state;

    for _ in 0..steps {
        let (zero_write, zero_move, zero_state, one_write, one_move, one_state) =
            instructions[&state];

        if tape[pointer] == 0 {
            tape[pointer] = zero_write;

            if zero_move {
                pointer += 1;
                if pointer == tape.len() {
                    tape.push_back(0);
                }
            } else {
                if pointer == 0 {
                    tape.push_front(0);
                } else {
                    pointer -= 1;
                }
            }

            state = zero_state;
        } else {
            tape[pointer] = one_write;

            if one_move {
                pointer += 1;
                if pointer == tape.len() {
                    tape.push_back(0);
                }
            } else {
                if pointer == 0 {
                    tape.push_front(0);
                } else {
                    pointer -= 1;
                }
            }

            state = one_state;
        }
    }

    tape.iter().sum()
}
