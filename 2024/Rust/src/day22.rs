use crate::helper::read_data;
use std::collections::{HashMap, VecDeque};
use std::hash::Hash;
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day22.txt");

    let secrets: Vec<i64> = data
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect();

    let p1: i64 = secrets
        .iter()
        .map(|secret| generate_secret(*secret, 2000))
        .sum();
    let p2: i64 = best_buy(&secrets);

    println!("{p1}\n{p2}");
}

fn generate_secret(secret: i64, n: usize) -> i64 {
    if n == 0 {
        return secret;
    }

    let mut secret: i64 = secret;

    secret ^= secret << 6;
    secret %= 16777216;

    secret ^= secret >> 5;
    secret %= 16777216;

    secret ^= secret << 11;
    secret %= 16777216;

    if n == 1 {
        return secret;
    } else {
        return generate_secret(secret, n - 1);
    }
}

fn best_buy(secrets: &Vec<i64>) -> i64 {
    let mut buys: HashMap<(i64, i64, i64, i64), HashMap<usize, i64>> = HashMap::new();

    for (i, secret) in secrets.iter().enumerate() {
        let mut secret = *secret;
        let mut moves: VecDeque<i64> = VecDeque::new();

        for j in 0..2000_usize {
            let secret2: i64 = generate_secret(secret, 1);

            moves.push_back((secret2 % 10) - (secret % 10));

            secret = secret2;

            if moves.len() > 4 {
                moves.pop_front();
            }

            if moves.len() == 4 {
                let key: (i64, i64, i64, i64) = (moves[0], moves[1], moves[2], moves[3]);

                if buys.contains_key(&key) {
                    if !buys[&key].contains_key(&i) {
                        buys.get_mut(&key).unwrap().insert(i, secret2 % 10);
                    }
                } else {
                    buys.insert(key, HashMap::from([(i, secret2 % 10)]));
                }
            }
        }
    }

    buys.iter()
        .map(|(moves, prices)| prices.iter().map(|(ind, price)| price).sum::<i64>())
        .max()
        .unwrap()
}
