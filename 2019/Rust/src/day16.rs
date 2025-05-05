use crate::helper::read_data;
use std::collections::{HashMap, HashSet};

pub fn main() {
    let data: String = read_data("../Data/Day16.txt");
    let signal: Vec<i32> = data.chars().map(|c| c.to_digit(10).unwrap() as i32).collect();

    let p1 = &fftn(signal.clone(), 100)[0..8];

    println!("{p1:?}");

    println!("{:?}", calculate_digit(&signal, 8, 1, HashMap::new()).0);

    let long_signal: Vec<Vec<i32>> = vec![signal.clone(); 10_000];

    println!("{:?}", &long_fft(long_signal.clone())[0][0..8])
}

fn calculate_digit(signal: &Vec<i32>, digit: usize, layer: u32, mut cache: HashMap<(usize, u32), i32>) -> (i32, HashMap<(usize, u32), i32>) {

    if let Some(out) = cache.get(&(digit, layer)) {
        return (*out, cache)
    } else if layer == 0 {
        return (signal[digit % signal.len()], cache)
    } else {
        let place: usize = digit + 1;
        let mut count: usize = 0;
        let mut multiplier: i32 = 1;
        let mut pointer = digit;
        let mut total: i32 = 0;

        while pointer < signal.len()*10_000 {
            if count == place {
                count = 0;
                multiplier *= -1;
                pointer += place;
            } else {
                let res =  calculate_digit(signal, pointer, layer-1, cache);

                cache = res.1;
                let sub = res.0;

                total += sub*multiplier;

                count += 1;
                pointer += 1;
            }
        }

        total *= total.signum();
        total %= 10;

        cache.insert((digit, layer), total);

        return (total, cache)
    }


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

fn long_fft(mut long_signal: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let num_digits = long_signal.len() * long_signal[0].len();

    for i in 0..8 {
        let mut pointer: usize = i + 1;
        let mut count: usize = 1;
        let mut multiplier: i32 = 1;

        let row = i / long_signal[0].len();
        let col = i % long_signal[0].len();
        
        while pointer < num_digits {

            if count == i + 1 {
                count = 0;
                multiplier *= -1;
                pointer += i + 1;
            } else {
                let row2 = pointer / long_signal[0].len();
                let col2 = pointer % long_signal[0].len();

                long_signal[row][col] += long_signal[row2][col2]*multiplier;

                count += 1;
                pointer += 1;
            }


            
        }

        long_signal[row][col] %= 10;
        long_signal[row][col] *= long_signal[row][col].signum();

    }

    long_signal
}