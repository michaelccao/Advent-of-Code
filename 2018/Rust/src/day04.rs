use crate::helper::read_data;
use regex::Regex;
use std::collections::HashMap;
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day04.txt");

    let shifts: Vec<(u32, u32, u32, u32, bool)> = get_shifts(&data);

    let (p1, p2) = sleepiest_guard(&shifts);

    println!("{p1}\n{p2}");
}

fn get_shifts(data: &String) -> Vec<(u32, u32, u32, u32, bool)> {
    let mut shifts: Vec<(u32, u32, u32, u32, bool)> = Vec::new();
    let mut active_guard: u32 = 0;

    let mut lines: Vec<&str> = data.lines().collect();
    lines.sort();

    let re = Regex::new(r"\[\d+-(\d+)-(\d+) (\d+):(\d+)\] (.*)").unwrap();

    for line in lines {
        let Some((_, [month, day, hour, minute, msg])) =
            re.captures(line.trim()).map(|cap| cap.extract())
        else {
            continue;
        };

        let month: u32 = month.parse().unwrap();
        let day: u32 = day.parse().unwrap();
        let hour: u32 = hour.parse().unwrap();
        let mut minute: u32 = minute.parse().unwrap();

        let msg: Vec<&str> = msg.split_whitespace().collect();

        if msg[0] == "Guard" {
            active_guard = msg[1][1..].parse().unwrap();
        }

        let awake: bool = msg[0] != "falls";

        if hour != 0 {
            minute = 0;
        }

        shifts.push((active_guard, month, day, minute, awake));
    }

    shifts
}

fn sleepiest_guard(shifts: &Vec<(u32, u32, u32, u32, bool)>) -> (u32, u32) {
    let mut guard_naps: HashMap<u32, [u32; 60]> = HashMap::new();

    let (mut prev_guard, _, mut prev_day, mut prev_minute, mut prev_awake) = shifts[0];

    for &(guard, _, day, minute, awake) in shifts {
        if awake && !prev_awake {
            let end: u32;

            if day == prev_day {
                end = minute;
            } else {
                end = 60;
            }

            let mut naps: [u32; 60] = if let Some(naps) = guard_naps.get_mut(&prev_guard) {
                *naps
            } else {
                [0; 60]
            };

            for i in prev_minute..end {
                naps[i as usize] += 1;
            }

            guard_naps.insert(prev_guard, naps);
        }

        prev_guard = guard;
        prev_day = day;
        prev_minute = minute;
        prev_awake = awake;
    }

    let (&sleepy_guard, &best_nap) = guard_naps
        .iter()
        .max_by_key(|&(_, n)| n.iter().sum::<u32>())
        .unwrap();

    let sleepy_minute: u32 = best_nap
        .iter()
        .enumerate()
        .max_by_key(|(_, minute)| **minute)
        .map(|(i, _)| i)
        .unwrap() as u32;

    let (&sleepy_guard2, &best_nap2) = guard_naps
        .iter()
        .max_by_key(|&(_, n)| n.iter().max().unwrap())
        .unwrap();

    let sleepy_minute2: u32 = best_nap2
        .iter()
        .enumerate()
        .max_by_key(|(_, minute)| **minute)
        .map(|(i, _)| i)
        .unwrap() as u32;

    (sleepy_guard * sleepy_minute, sleepy_guard2 * sleepy_minute2)
}
