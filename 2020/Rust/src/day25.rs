use crate::helper::read_data;

pub fn main() {
    let data: String = read_data("../Data/Day25.txt");

    let keys: Vec<u64> = data.lines().map(|line| line.parse().unwrap()).collect();

    let card_key: u64 = keys[0];
    let door_key: u64 = keys[1];

    let card_count: u32 = find_count(7, card_key);
    let door_count: u32 = find_count(7, door_key);

    let encryption: u64 = transform(card_key, door_count);
    let encryption2: u64 = transform(door_key, card_count);

    println!("{encryption}, {encryption2}");
}

fn transform(s: u64, n: u32) -> u64 {
    let mut val: u64 = 1;

    for _ in 0..n {
        val *= s;
        val %= 20201227
    }

    val
}

fn find_count(s: u64, target: u64) -> u32 {
    let mut val: u64 = 1;
    let mut n: u32 = 0;

    while val != target {
        n += 1;
        val *= s;
        val %= 20201227;
    }

    n
}
