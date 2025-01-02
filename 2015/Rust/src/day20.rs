use crate::helper::read_data;
use std::collections::HashMap;
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day20.txt");

    let target_sum: u32 = data.parse().unwrap();

    let mut prime_factors: HashMap<u32, HashMap<u32, u32>> = HashMap::new();

    let p1: u32;
    let p2: u32;

    (p1, prime_factors) = find_house(target_sum, prime_factors, false);
    (p2, _) = find_house(target_sum, prime_factors, true);

    println!("{p1}\n{p2}");
}

fn prime_factorization(
    n: u32,
    mut prime_factors: HashMap<u32, HashMap<u32, u32>>,
) -> HashMap<u32, HashMap<u32, u32>> {
    if prime_factors.contains_key(&n) {
        return prime_factors;
    }

    for i in 2..n {
        if n % i == 0 {
            prime_factors = prime_factorization(i, prime_factors);
            prime_factors = prime_factorization(n / i, prime_factors);

            prime_factors.insert(
                n,
                combine_prime_factors(&prime_factors[&i], &prime_factors[&(n / i)]),
            );

            return prime_factors;
        }
    }

    prime_factors.insert(n, HashMap::from([(n, 1)]));

    prime_factors
}

fn combine_prime_factors(
    factors1: &HashMap<u32, u32>,
    factors2: &HashMap<u32, u32>,
) -> HashMap<u32, u32> {
    let mut factors: HashMap<u32, u32> = HashMap::new();

    for (prime, pow) in factors1 {
        if let Some(pow2) = factors2.get(prime) {
            factors.insert(*prime, *pow + *pow2);
        } else {
            factors.insert(*prime, *pow);
        }
    }

    for (prime2, pow2) in factors2 {
        if !factors1.contains_key(prime2) {
            factors.insert(*prime2, *pow2);
        }
    }

    factors
}

fn get_divisors(
    n: u32,
    mut prime_factors: HashMap<u32, HashMap<u32, u32>>,
) -> (Vec<u32>, HashMap<u32, HashMap<u32, u32>>) {
    let mut divisors: Vec<u32> = Vec::new();

    prime_factors = prime_factorization(n, prime_factors);

    let factors = prime_factors[&n]
        .iter()
        .map(|(factor, power)| (*factor, *power))
        .collect::<Vec<_>>();

    let mut nodes: Vec<(u32, usize)> = vec![(1, 0)];

    while nodes.len() > 0 {
        let (div, pointer): (u32, usize) = nodes.pop().unwrap();

        if pointer == factors.len() {
            divisors.push(div);
            continue;
        }

        let (f, p): (u32, u32) = factors[pointer];

        for i in 0..p + 1 {
            nodes.push((div * f.pow(i), pointer + 1));
        }
    }

    divisors.sort();

    (divisors, prime_factors)
}

fn find_house(
    target_sum: u32,
    mut prime_factors: HashMap<u32, HashMap<u32, u32>>,
    part2: bool,
) -> (u32, HashMap<u32, HashMap<u32, u32>>) {
    let target_sum: u32 = target_sum / if part2 { 11 } else { 10 };

    for i in 2..target_sum + 1 {
        let res = get_divisors(i, prime_factors);
        prime_factors = res.1;

        if !part2 && res.0.iter().sum::<u32>() >= target_sum {
            return (i, prime_factors);
        } else if part2
            && res
                .0
                .iter()
                .map(|factor| if i / *factor <= 50 { *factor } else { 0 })
                .sum::<u32>()
                >= target_sum
        {
            return (i, prime_factors);
        }
    }

    return (0, prime_factors);
}
