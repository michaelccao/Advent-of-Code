use crate::helper::read_data;
use std::collections::HashMap;
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day09.txt");

    let (players, last_score) = get_game_settings(&data);

    let p1: u32 = run_game(players, last_score);
    let p2: u32 = run_game(players, last_score * 100);

    println!("{p1}\n{p2}");
}

fn get_game_settings(data: &String) -> (usize, u32) {
    let game: Vec<&str> = data.split_whitespace().collect();

    let players: usize = game[0].parse().unwrap();
    let last_score: u32 = game[6].parse().unwrap();

    (players, last_score)
}

fn run_game(players: usize, last_marble: u32) -> u32 {
    let mut scores: Vec<u32> = vec![0; players];
    let mut marbles: HashMap<u32, (u32, u32)> = HashMap::from([(0, (1, 1)), (1, (0, 0))]);

    let mut marble: u32 = 2;

    let mut current: u32 = 0;

    while marble <= last_marble {
        if marble % 23 != 0 {
            let (_, marble1) = marbles[&current];
            let (_, marble2) = marbles[&marble1];
            let (_, marble3) = marbles[&marble2];

            marbles.insert(marble, (marble1, marble2));
            marbles.insert(marble1, (current, marble));
            marbles.insert(marble2, (marble, marble3));

            current = marble;
        } else {
            let mut to_remove: u32 = current;

            for _ in 0..7 {
                let (prev_marble, _) = marbles[&to_remove];
                to_remove = prev_marble;
            }

            let (prev_marble, next_marble) = marbles[&to_remove];

            if let Some(neighbors) = marbles.get_mut(&prev_marble) {
                neighbors.1 = next_marble;
            }

            if let Some(neighbors) = marbles.get_mut(&next_marble) {
                neighbors.0 = prev_marble;
            }

            scores[(marble as usize - 1) % players] += marble + to_remove;

            current = next_marble;
        }

        marble += 1;
    }

    *scores.iter().max().unwrap()
}
