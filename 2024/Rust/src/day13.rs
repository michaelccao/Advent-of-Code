use crate::helper::read_data;
use std::collections::{HashSet, HashMap};
use std::vec::Vec;
use regex::Regex;

pub fn main() {
    
    let data = read_data("../Data/Day13.txt");
    
    let games = get_games(&data);

    let p1 = solve(&games, true);
    let p2 = solve(&games, false);

    println!("{p1}");
    println!("{p2}");

}

fn get_games(data: &String) -> Vec<(i64, i64, i64, i64, i64, i64)> {
    let re = Regex::new(r"X\+(\d+), Y\+(\d+)\r\nButton B: X\+(\d+), Y\+(\d+)\r\nPrize: X=(\d+), Y=(\d+)").unwrap();

    let games = re.captures_iter(data).map(|caps| {
        let (_, [x1, y1, x2, y2, xf, yf]) = caps.extract();
        let x1:i64 = x1.parse::<i64>().unwrap();
        let x2:i64 = x2.parse::<i64>().unwrap();
        
        let y1:i64 = y1.parse::<i64>().unwrap();
        let y2:i64 = y2.parse::<i64>().unwrap();
        
        let xf:i64 = xf.parse::<i64>().unwrap();
        let yf:i64 = yf.parse::<i64>().unwrap();

        (x1, y1, x2, y2, xf, yf)
    }).collect::<Vec<(i64, i64, i64, i64, i64, i64)>>();

    games

}

fn solve(games: &Vec<(i64, i64, i64, i64, i64, i64)>, part1: bool) -> i64 {
    let mut tokens:i64 = 0;

    for game in games {
        let (x1, y1, x2, y2, xf, yf) = game;

        let xf = if !part1 { xf + 10000000000000 } else { *xf };
        let yf = if !part1 { yf + 10000000000000 } else { *yf };

        let b = (yf*x1 - xf*y1) / (y2*x1 - x2*y1);
        let a = (xf - x2*b) / x1;

        if x1*a + x2*b == xf && y1*a + y2*b == yf {
            tokens += 3*a + b;
        }
    }

    tokens
}