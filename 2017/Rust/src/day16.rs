use crate::helper::read_data;
use std::collections::HashMap;
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day16.txt");
    let dance_moves: Vec<&str> = data.split(',').collect();

    let p1: String = p1(&dance_moves);
    let p2: String = p2(&dance_moves);

    println!("{p1}\n{p2}");
}

fn p1(dance_moves: &Vec<&str>) -> String {
    let mut dancers: Vec<u8> = Vec::from_iter(0..16);

    dancers = perform_dance(dancers, dance_moves);

    dancers_to_str(dancers)
}

fn p2(dance_moves: &Vec<&str>) -> String {
    let mut cache: HashMap<Vec<u8>, u32> = HashMap::new();
    let mut vec_cache: Vec<Vec<u8>> = Vec::new();

    let mut dancers: Vec<u8> = Vec::from_iter(0..16);

    cache.insert(dancers.clone(), 0);
    vec_cache.push(dancers.clone());

    for i in 1..1000000001 {
        dancers = perform_dance(dancers, dance_moves);
        vec_cache.push(dancers.clone());

        
        if let Some(prev_i) = cache.insert(dancers.clone(), i) {
            
            let cycle_length = i - prev_i;
            let target_i = (1000000000 - prev_i) % cycle_length + prev_i;

            return dancers_to_str(vec_cache[target_i as usize].clone())
        }
    }

    "".to_string()
}

fn perform_dance(mut dancers: Vec<u8>, dance_moves: &Vec<&str>) -> Vec<u8> {
    let a: u8 = 'a' as u8;

    for &dm in dance_moves {
        let command: &str = &dm[0..1];
        let params: &str = &dm[1..];

        match command {
            "s" => {
                let shift: usize = params.parse().unwrap();
                dancers.rotate_right(shift);
            },
            "x" => {
                let mut positions = params.split('/');
                let p1: usize = positions.next().unwrap().parse().unwrap();
                let p2: usize = positions.next().unwrap().parse().unwrap();

                dancers.swap(p1, p2);
            },
            "p" => {
                let mut partners = params.split('/');
                let d1: u8 = partners.next().unwrap().chars().next().unwrap() as u8 - a;
                let d2: u8 = partners.next().unwrap().chars().next().unwrap() as u8 - a;

                let mut positions: Vec<usize> = Vec::new();

                for p in 0..dancers.len() {
                    if dancers[p] == d1 || dancers[p] == d2 {
                        positions.push(p);
                        if positions.len() == 2 { break }
                    }
                }

                let (p1, p2) = (positions[0], positions[1]);

                dancers.swap(p1, p2);
            },
            _ => {}
        }
    }

    dancers

    

}

fn dancers_to_str(dancers: Vec<u8>) -> String {
    let a: u8 = 'a' as u8;
    let mut dance_str: String = String::new();

    for d in dancers {
        dance_str.push((a+d) as char);
    }

    dance_str
}

