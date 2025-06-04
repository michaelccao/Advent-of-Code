use crate::helper::read_data;

pub fn main() {
    let data: String = read_data("../Data/Day16.txt");
    let signal: Vec<i32> = data.chars().map(|c| c.to_digit(10).unwrap() as i32).collect();

    let p1 = &fftn(signal.clone(), 100)[0..8];

    println!("{p1:?}");

    let mut offset = 0;
    for d in &signal[0..7] {
        offset *= 10;
        offset += d;
    }

    // Digits greater than n/2 just sum all following digits
    // Offset is well past n/2 and we can just transform the tail end
    let mut tail: Vec<i32> = Vec::new();

    for i in offset as usize..signal.len()*10000 {
        tail.push(signal[i % signal.len()]);
    }

    println!("{:?}", &tail_fftn(&tail, 100)[0..8]);

    
}

fn tail_fft(signal: &Vec<i32>) -> Vec<i32> {
    // First digit is just sum of vector
    // Second digit is sum minus first
    // Third digit is sum minus first and second
    // etc...
    let mut signal2: Vec<i32> = Vec::new();

    let mut current_value: i32 = signal.iter().sum();

    signal2.push(current_value % 10);

    for i in 1..signal.len() {
        current_value -= signal[i-1];
        signal2.push(current_value % 10);
    }

    signal2
}

fn tail_fftn(signal: &Vec<i32>, n: u32) -> Vec<i32> {
    
    let mut signal2: Vec<i32> = signal.clone();

    for _ in 0..n {
        signal2 = tail_fft(&signal2);
    }

    signal2
}

fn fft(mut signal: Vec<i32>) -> Vec<i32> {

    for i in 0..signal.len() {
        let mut pointer: usize = i+1;
        let place: usize = i+1;
        let mut count: usize = 1;
        let mut multiplier: i32 = 1;

        while pointer < signal.len() {

            if count == place {
                count = 0;
                multiplier *= -1;
                pointer += place;
            } else {
                signal[i] += signal[pointer]*multiplier;
                

                count += 1;
                pointer += 1;
            }
        }

        signal[i] *= signal[i].signum();
        signal[i] %= 10;
    }

    signal
}

fn fftn(mut signal: Vec<i32>, n: u32) -> Vec<i32> {
    for _ in 0..n {
        signal = fft(signal);
    }

    signal
}