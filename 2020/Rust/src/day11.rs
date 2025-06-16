use crate::helper::read_data;

pub fn main() {
    let data: String = read_data("../Data/Day11.txt");
    let seats: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();

    let p1: u32 = simulate_seating(&seats, false);

    println!("{p1}");

    let p2: u32 = simulate_seating(&seats, true);

    println!("{p2}");
}

fn change_seats(seats: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut seats2: Vec<Vec<char>> = seats.clone();

    for i in 0..seats.len() {
        for j in 0..seats[i].len() {
            if seats[i][j] == '.' {
                continue;
            }
            let mut occupied_neighbor: u32 = 0;

            if i > 0 && seats[i - 1][j] == '#' {
                occupied_neighbor += 1;
            }
            if i > 0 && j > 0 && seats[i - 1][j - 1] == '#' {
                occupied_neighbor += 1;
            }
            if i > 0 && j + 1 < seats[i].len() && seats[i - 1][j + 1] == '#' {
                occupied_neighbor += 1;
            }
            if i + 1 < seats.len() && seats[i + 1][j] == '#' {
                occupied_neighbor += 1;
            }
            if i + 1 < seats.len() && j > 0 && seats[i + 1][j - 1] == '#' {
                occupied_neighbor += 1;
            }
            if i + 1 < seats.len() && j + 1 < seats[i].len() && seats[i + 1][j + 1] == '#' {
                occupied_neighbor += 1;
            }
            if j > 0 && seats[i][j - 1] == '#' {
                occupied_neighbor += 1;
            }
            if j + 1 < seats[i].len() && seats[i][j + 1] == '#' {
                occupied_neighbor += 1;
            }

            if seats[i][j] == 'L' && occupied_neighbor == 0 {
                seats2[i][j] = '#';
            }
            if seats[i][j] == '#' && occupied_neighbor >= 4 {
                seats2[i][j] = 'L';
            }
        }
    }

    seats2
}

fn change_seats_los(seats: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut seats2: Vec<Vec<char>> = seats.clone();

    let directions: [(i32, i32); 8] = [
        (-1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
    ];

    for i in 0..seats.len() {
        for j in 0..seats[i].len() {
            if seats[i][j] == '.' {
                continue;
            }
            let mut occupied: u32 = 0;

            for (di, dj) in directions {
                let mut i2 = i as i32;
                let mut j2 = j as i32;

                i2 += di;
                j2 += dj;

                while i2 >= 0 && i2 < seats.len() as i32 && j2 >= 0 && j2 < seats[i].len() as i32 {
                    if seats[i2 as usize][j2 as usize] == '#' {
                        occupied += 1;
                        break;
                    } else if seats[i2 as usize][j2 as usize] == 'L' {
                        break;
                    }

                    i2 += di;
                    j2 += dj;
                }
            }

            if seats[i][j] == '#' && occupied >= 5 {
                seats2[i][j] = 'L';
            }
            if seats[i][j] == 'L' && occupied == 0 {
                seats2[i][j] = '#';
            }
        }
    }

    seats2
}

fn simulate_seating(seats: &Vec<Vec<char>>, part2: bool) -> u32 {
    let mut seats2: Vec<Vec<char>> = seats.clone();
    let mut prev_seats: Vec<Vec<char>> = seats2.clone();

    loop {
        if part2 {
            seats2 = change_seats_los(&prev_seats);
        } else {
            seats2 = change_seats(&prev_seats);
        }

        if seats2 == prev_seats {
            return seats2
                .iter()
                .map(|row| {
                    row.iter()
                        .map(|&c| if c == '#' { 1 } else { 0 })
                        .sum::<u32>()
                })
                .sum();
        }

        prev_seats = seats2;
    }
}
