use crate::helper::read_data;
use std::collections::{HashSet, VecDeque};

pub fn main() {
    let data: String = read_data("../Data/Day22.txt");

    let decks: Vec<VecDeque<usize>> = get_decks(&data);

    let p1: usize = play_game(&decks);

    println!("{p1}");

    let p2: usize = play_recursive_game(&decks).1;

    println!("{p2}");
}

fn get_decks(data: &String) -> Vec<VecDeque<usize>> {
    let mut decks: Vec<VecDeque<usize>> = Vec::new();

    for line in data.lines() {
        if line.starts_with("Player") {
            decks.push(VecDeque::new());
        } else if line.len() > 0 {
            let card: usize = line.parse().unwrap();
            decks.last_mut().unwrap().push_back(card);
        }
    }

    decks
}

fn play_game(decks: &Vec<VecDeque<usize>>) -> usize {
    let mut deck1: VecDeque<usize> = decks[0].clone();
    let mut deck2: VecDeque<usize> = decks[1].clone();

    while deck1.len() > 0 && deck2.len() > 0 {
        let card1: usize = deck1.pop_front().unwrap();
        let card2: usize = deck2.pop_front().unwrap();

        if card1 > card2 {
            deck1.push_back(card1);
            deck1.push_back(card2);
        } else {
            deck2.push_back(card2);
            deck2.push_back(card1);
        }
    }

    if deck1.len() > 0 {
        score_deck(&deck1)
    } else {
        score_deck(&deck2)
    }
}

fn play_recursive_game(decks: &Vec<VecDeque<usize>>) -> (bool, usize) {
    let mut deck1: VecDeque<usize> = decks[0].clone();
    let mut deck2: VecDeque<usize> = decks[1].clone();

    let mut played: HashSet<(VecDeque<usize>, VecDeque<usize>)> = HashSet::new();

    while deck1.len() > 0 && deck2.len() > 0 {
        if played.contains(&(deck1.clone(), deck2.clone())) {
            return (true, score_deck(&deck1));
        } else {
            played.insert((deck1.clone(), deck2.clone()));
        }

        let card1: usize = deck1.pop_front().unwrap();
        let card2: usize = deck2.pop_front().unwrap();

        if deck1.len() >= card1 && deck2.len() >= card2 {
            let sub_deck1: VecDeque<usize> =
                deck1.make_contiguous()[0..card1].iter().cloned().collect();
            let sub_deck2: VecDeque<usize> =
                deck2.make_contiguous()[0..card2].iter().cloned().collect();

            let sub_decks: Vec<VecDeque<usize>> = vec![sub_deck1, sub_deck2];

            if play_recursive_game(&sub_decks).0 {
                deck1.push_back(card1);
                deck1.push_back(card2);
            } else {
                deck2.push_back(card2);
                deck2.push_back(card1);
            }
        } else if card1 > card2 {
            deck1.push_back(card1);
            deck1.push_back(card2);
        } else {
            deck2.push_back(card2);
            deck2.push_back(card1);
        }
    }

    if deck1.len() > 0 {
        (true, score_deck(&deck1))
    } else {
        (false, score_deck(&deck2))
    }
}

fn score_deck(deck: &VecDeque<usize>) -> usize {
    deck.iter()
        .enumerate()
        .map(|(i, &c)| (deck.len() - i) * c)
        .sum()
}
