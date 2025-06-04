use crate::helper::read_data;

pub fn main() {
    let data: String = read_data("../Data/Day22.txt");
    let shuffles: Vec<Vec<&str>> = data
        .lines()
        .map(|line| line.split_whitespace().collect())
        .collect();

    // With deck of size D:
    // Deal into new deck: index i shifts to D-i-1
    // Cut N: index i shifts to (i + D - N) % D
    // Increment N: index i shifts to i*N % D

    let d: i128 = 10007;

    let p1: i128 = execute_shuffles(&shuffles, 2019, d);

    println!("{p1}");

    let d2: i128 = 119315717514047;
    let num_shuffles: i128 = 101741582076661;

    let p2: i128 = big_unshuffle(&shuffles, 2020, d2, num_shuffles);

    println!("{p2}");
}

fn big_unshuffle(shuffles: &Vec<Vec<&str>>, card: i128, d2: i128, num_shuffles: i128) -> i128 {
    // Every shuffle is a linear operation
    // So the final result is still a linear operation a*i + b
    // This is true after ANY number of shuffles

    let mut unshuffle_params: Vec<(i128, i128)> = Vec::new();

    for i in 0..5 {
        if i == 0 {
            let offset: i128 = undo_shuffles(shuffles, 0, d2);
            let scale: i128 = (undo_shuffles(shuffles, 1, d2) - offset + d2) % d2;

            unshuffle_params.push((scale, offset));
            continue;
        }

        let (scale, offset) = unshuffle_params[i - 1];
        // Do this shuffle 1000 times to get scale and offset of batch shuffle
        // This new scale represents change after doing previous shuffle 1000 times
        // So unshuffle_params[0] is 1000^0 shuffles
        // unshuffle_params[1] is 1000^1 shuffles
        // unshuffle_params[2] is 1000^2 shuffles etc...

        let mut offset2: i128 = 0;
        let mut scale2: i128 = 1;

        for _ in 0..1000 {
            offset2 = (offset2 * scale + offset) % d2;
            scale2 = (scale2 * scale + offset) % d2;
        }

        scale2 = scale2 + d2 - offset2;
        scale2 %= d2;

        unshuffle_params.push((scale2, offset2))
    }

    let mut shuffles_remaining = num_shuffles;

    let mut card: i128 = card;

    for i in 0..unshuffle_params.len() {
        let shuffle_size: i128 = 1000_i128.pow(4 - i as u32);
        let (scale, offset) = unshuffle_params[4 - i];

        let batch_shuffles: i128 = shuffles_remaining / shuffle_size;

        for _ in 0..batch_shuffles {
            card = card * scale + offset;
            card %= d2;
        }

        shuffles_remaining %= shuffle_size;
    }

    card
}

fn execute_shuffles(shuffles: &Vec<Vec<&str>>, mut i: i128, d: i128) -> i128 {
    for shuffle in shuffles {
        if shuffle[0] == "deal" && shuffle[1] == "into" {
            i = d - i - 1;
        } else if shuffle[0] == "deal" && shuffle[1] == "with" {
            let inc: i128 = shuffle.last().unwrap().parse().unwrap();
            i = i * inc % d;
        } else if shuffle[0] == "cut" {
            let cut: i128 = shuffle.last().unwrap().parse().unwrap();
            i = (i + d - cut) % d;
        }
    }

    i
}

fn undo_shuffles(shuffles: &Vec<Vec<&str>>, mut i: i128, d: i128) -> i128 {
    for s in 0..shuffles.len() {
        let shuffle: &Vec<&str> = &shuffles[shuffles.len() - s - 1];

        if shuffle[0] == "deal" && shuffle[1] == "into" {
            i = d - i - 1;
        } else if shuffle[0] == "deal" && shuffle[1] == "with" {
            let inc: i128 = shuffle.last().unwrap().parse().unwrap();
            while i % inc != 0 {
                i += d;
            }
            i = i / inc;
        } else if shuffle[0] == "cut" {
            let cut: i128 = shuffle.last().unwrap().parse().unwrap();
            i = (i + d + cut) % d;
        }
    }

    i
}
