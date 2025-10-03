use crate::helper::read_data;
use std::collections::HashMap;

pub fn main() {
    let data: String = read_data("../Data/Day21.txt");
    let players: Vec<usize> = data
        .lines()
        .map(|line| line.split("position: ").last().unwrap().parse().unwrap())
        .collect();

    let p1: usize = play_game(&players);

    println!("{p1}");

    let (p1_wins, p2_wins, _) = dirac_game((players[0], 0), (players[1], 0), true, HashMap::new());

    let p2: usize = p1_wins.max(p2_wins);

    println!("{p2}");
}

fn play_game(players: &Vec<usize>) -> usize {
    let mut rolls: usize = 0;

    let mut p1: (usize, usize) = (players[0], 0);
    let mut p2: (usize, usize) = (players[1], 0);

    'game: loop {
        // Player 1
        for _ in 0..3 {
            rolls += 1;
            let mut dice = rolls % 100;
            if dice == 0 {
                dice += 100
            }
            p1.0 += dice;
        }
        p1.0 %= 10;
        if p1.0 == 0 {
            p1.0 += 10;
        }
        p1.1 += p1.0;

        if p1.1 >= 1000 {
            break 'game;
        }

        // Player 2
        for _ in 0..3 {
            rolls += 1;
            let mut dice = rolls % 100;
            if dice == 0 {
                dice = 100
            }
            p2.0 += dice;
        }
        p2.0 %= 10;
        if p2.0 == 0 {
            p2.0 = 10;
        }
        p2.1 += p2.0;

        if p2.1 >= 1000 {
            break 'game;
        }
    }

    if p1.1 >= 1000 {
        p2.1 * rolls
    } else {
        p1.1 * rolls
    }
}

fn dirac_game(
    p1: (usize, usize),
    p2: (usize, usize),
    p1_turn: bool,
    mut cache: HashMap<((usize, usize), (usize, usize), bool), (usize, usize)>,
) -> (
    usize,
    usize,
    HashMap<((usize, usize), (usize, usize), bool), (usize, usize)>,
) {
    // 27 Total dice roll possibilities
    // 3 - (1, 1, 1) 1 possibility
    // 4 - (1, 1, 2) 3 possibilities
    // 5 - (1, 1, 3), (1, 2, 2) 6 possibilities
    // 6 - (2, 2, 2) or (1, 2, 3) 7 possibilities
    // 7 - (1, 3, 3), (2, 2, 3) 6 possibilities
    // 8 - (2, 3, 3) 3 possibilities
    // 9 - (3, 3, 3) 1 possibility

    let rolls: [(usize, usize); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

    if p1_turn {
        if p1.1 == 20 {
            cache.insert((p1, p2, p1_turn), (27, 0));
            return (27, 0, cache);
        }

        let mut p1_wins: usize = 0;
        let mut p2_wins: usize = 0;

        for (roll, outcomes) in rolls {
            let mut p1: (usize, usize) = p1.clone();

            p1.0 += roll;
            p1.0 %= 10;
            if p1.0 == 0 {
                p1.0 = 10
            }
            p1.1 += p1.0;

            if p1.1 >= 21 {
                p1_wins += outcomes;
            } else {
                let result = dirac_game(p1, p2, false, cache);
                p1_wins += outcomes * result.0;
                p2_wins += outcomes * result.1;
                cache = result.2;
            }
        }

        cache.insert((p1, p2, true), (p1_wins, p2_wins));

        return (p1_wins, p2_wins, cache);
    } else {
        if p2.1 == 20 {
            cache.insert((p1, p2, p1_turn), (0, 27));
            return (0, 27, cache);
        }

        let mut p1_wins: usize = 0;
        let mut p2_wins: usize = 0;

        for (roll, outcomes) in rolls {
            let mut p2: (usize, usize) = p2.clone();

            p2.0 += roll;
            p2.0 %= 10;
            if p2.0 == 0 {
                p2.0 = 10
            }
            p2.1 += p2.0;

            if p2.1 >= 21 {
                p2_wins += outcomes;
            } else {
                let result = dirac_game(p1, p2, true, cache);
                p1_wins += outcomes * result.0;
                p2_wins += outcomes * result.1;
                cache = result.2;
            }
        }

        cache.insert((p1, p2, true), (p1_wins, p2_wins));

        return (p1_wins, p2_wins, cache);
    }
}
