use crate::helper::read_data;
use std::collections::HashMap;

pub fn main() {
    let data: String = read_data("../Data/Day04.txt");

    let (numbers, boards) = setup(&data);

    let p1: u32 = play_bingo(&numbers, &boards, false);

    println!("{p1}");

    let p2: u32 = play_bingo(&numbers, &boards, true);

    println!("{p2}");
}

fn setup(data: &String) -> (Vec<u32>, Vec<Board>) {
    let mut data = data.split("\r\n\r\n");

    let numbers: Vec<u32> = data
        .next()
        .unwrap()
        .split(",")
        .map(|num| num.parse().unwrap())
        .collect();

    let mut boards: Vec<Board> = Vec::new();

    for board in data {
        boards.push(Board::create_board_from_str(board))
    }

    (numbers, boards)
}

fn play_bingo(numbers: &Vec<u32>, boards: &Vec<Board>, part2: bool) -> u32 {
    let mut boards: Vec<Board> = boards.clone();
    let mut has_won: Vec<bool> = vec![false; boards.len()];
    let mut last_score: u32 = 0;

    for &num in numbers {
        for i in 0..boards.len() {
            if has_won[i] {
                continue;
            }
            let score: u32 = boards[i].activate_number(num);
            if score > 0 {
                has_won[i] = true;
                if !part2 {
                    return score;
                } else {
                    last_score = score;
                }
            }
        }
    }

    last_score
}

#[derive(Clone)]
struct Board {
    numbers: HashMap<u32, (usize, usize)>,
    activated: HashMap<(usize, usize), u32>,
}

impl Board {
    fn activate_number(&mut self, num: u32) -> u32 {
        if let Some(&(i, j)) = self.numbers.get(&num) {
            self.activated.insert((i, j), num);

            let mut row: bool = true;

            for j2 in 0..5 {
                if !self.activated.contains_key(&(i, j2)) {
                    row = false;
                    break;
                }
            }

            if row {
                return (self.numbers.keys().sum::<u32>() - self.activated.values().sum::<u32>())
                    * num;
            }

            let mut col: bool = true;

            for i2 in 0..5 {
                if !self.activated.contains_key(&(i2, j)) {
                    col = false;
                    break;
                }
            }

            if col {
                return (self.numbers.keys().sum::<u32>() - self.activated.values().sum::<u32>())
                    * num;
            }

            // Diagonals don't count (oops)

            // if i == j {
            //     score = 0;
            //     let mut diag: bool = true;
            //     for d in 0..5 {
            //         if let Some(num2) = self.activated.get(&(d, d)) {
            //             score += num2;
            //         } else {
            //             diag = false;
            //             break;
            //         }
            //     }

            //     if diag {
            //         return (self.numbers.keys().sum::<u32>() - self.activated.values().sum::<u32>())*num
            //     }
            // }

            // if i == 4-j {
            //     score = 0;
            //     let mut diag: bool = true;
            //     for d in 0..5 {
            //         if let Some(num2) = self.activated.get(&(d, 4-d)) {
            //             score += num2;
            //         } else {
            //             diag = false;
            //             break;
            //         }
            //     }

            //     if diag {
            //         return (self.numbers.keys().sum::<u32>() - self.activated.values().sum::<u32>())*num
            //     }
            // }

            return 0;
        } else {
            return 0;
        }
    }

    fn create_board_from_str(board_str: &str) -> Self {
        let mut numbers: HashMap<u32, (usize, usize)> = HashMap::new();

        for (i, row) in board_str.lines().enumerate() {
            for (j, num) in row.split_whitespace().enumerate() {
                numbers.insert(num.parse().unwrap(), (i, j));
            }
        }

        Board {
            numbers: numbers,
            activated: HashMap::new(),
        }
    }
}
