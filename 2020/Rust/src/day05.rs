use crate::helper::read_data;
use std::collections::HashSet;

pub fn main() {
    let data: String = read_data("../Data/Day05.txt");
    let seats: Vec<&str> = data.lines().collect();

    let p1: u32 = seats.iter().map(|&seat| find_seat(seat)).max().unwrap();

    println!("{p1}");

    let p2: u32 = find_missing_seat(&seats);

    println!("{p2}");
}

fn find_seat(seat: &str) -> u32 {
    let mut row: u32 = 0;
    let mut digit: u32 = 64;

    for c in seat[0..7].chars() {
        if c == 'B' {
            row += digit;
        }

        digit /= 2;
    }

    let mut col: u32 = 0;
    let mut digit: u32 = 4;

    for c in seat[7..].chars() {
        if c == 'R' {
            col += digit;
        }

        digit /= 2;
    }

    row * 8 + col
}

fn find_missing_seat(seats: &Vec<&str>) -> u32 {
    let mut unfilled_seats: HashSet<u32> = (8..8 * 127).collect();

    seats.iter().for_each(|&seat| {
        unfilled_seats.remove(&find_seat(seat));
    });

    for &seat in &unfilled_seats {
        if !unfilled_seats.contains(&(seat + 1)) && !unfilled_seats.contains(&(seat - 1)) {
            return seat;
        }
    }

    0
}
