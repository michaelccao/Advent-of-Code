use crate::helper::read_data;
use std::collections::HashMap;
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day04.txt");

    let rooms: Vec<(String, u32, String)> = get_rooms(&data);

    let p1: u32 = rooms
        .iter()
        .map(|room| if is_real_room(room) { room.1 } else { 0 })
        .sum();

    let real_rooms: Vec<String> = data.lines().map(|line| real_name(line.trim())).collect();

    let mut p2: u32 = 0;

    for i in 0..real_rooms.len() {
        if real_rooms[i] == "northpole object storage" {
            p2 = rooms[i].1;
            break;
        }
    }

    println!("{p1}\n{p2}");
}

fn get_rooms(data: &String) -> Vec<(String, u32, String)> {
    let mut rooms: Vec<(String, u32, String)> = Vec::new();

    for line in data.lines() {
        let line: Vec<&str> = line.trim().split('-').collect();

        let name: String = line[..line.len() - 1].join("");
        let mut id_hash = line[line.len() - 1].split('[');
        let id: u32 = id_hash.next().unwrap().parse().unwrap();
        let hash = String::from(id_hash.next().unwrap().trim_matches(']'));

        rooms.push((name, id, hash));
    }

    rooms
}

fn is_real_room(room: &(String, u32, String)) -> bool {
    let mut counter: HashMap<char, i32> = HashMap::new();

    let (name, id, hash) = room;

    for c in name.chars() {
        if let Some(count) = counter.get_mut(&c) {
            *count += 1;
        } else {
            counter.insert(c, 1);
        }
    }

    let mut letters: Vec<char> = counter.keys().cloned().collect();
    letters.sort();

    letters.sort_by_key(|c| (-counter[c], *c));

    let calc_hash: String = letters[0..5].iter().collect();

    calc_hash == *hash
}

fn real_name(room: &str) -> String {
    let mut name: String = String::new();
    let (room, id) = room.rsplit_once('-').unwrap();
    let id: u32 = id.split('[').next().unwrap().parse().unwrap();

    const A: u32 = 'a' as u32;

    for c in room.chars() {
        if c == '-' {
            name.push(' ');
        } else {
            name.push(char::from_u32(A + (c as u32 - A + id) % 26).unwrap());
        }
    }

    name
}
