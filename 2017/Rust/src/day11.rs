use crate::helper::read_data;
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day11.txt");
    let steps: Vec<&str> = data.split(',').collect();

    let (p1, p2): (i32, i32) = follow_steps(&steps);

    println!("{p1}\n{p2}");
}

fn follow_steps(steps: &Vec<&str>) -> (i32, i32) {
    let mut moves: Vec<i32> = vec![0; 3];

    let mut furthest: i32 = 0;

    for &step in steps {
        match step {
            "n" => moves[0] += 1,
            "nw" => moves[1] += 1,
            "ne" => moves[2] += 1,
            "s" => moves[0] -= 1,
            "se" => moves[1] -= 1,
            "sw" => moves[2] -= 1,
            _ => {}
        }

        let dist: i32 = distance(&moves);

        if dist > furthest {
            furthest = dist;
        }
    }

    (distance(&moves), furthest)
}

fn distance(moves: &Vec<i32>) -> i32 {
    let mut moves: Vec<i32> = moves.clone();

    if moves[1] * moves[2] > 0 {
        if moves[1].abs() < moves[2].abs() {
            moves[1] = 0;
            moves[2] -= moves[1];

            moves[0] += moves[1]
        } else {
            moves[2] = 0;
            moves[1] -= moves[2];

            moves[0] += moves[2];
        }
    }

    if moves[0] * moves[1] < 0 {
        if moves[0].abs() < moves[1].abs() {
            moves[0] = 0;
            moves[1] += moves[0];

            moves[2] += moves[0]
        } else {
            moves[1] = 0;
            moves[0] += moves[1];

            moves[2] -= moves[1];
        }
    } else if moves[0] * moves[2] < 0 {
        if moves[0].abs() < moves[2].abs() {
            moves[0] = 0;
            moves[2] += moves[0];

            moves[1] += moves[0];
        } else {
            moves[2] = 0;
            moves[0] += moves[2];

            moves[1] -= moves[2];
        }
    }

    moves.iter().map(|&m| m.abs()).sum::<i32>()
}
