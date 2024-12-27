use crate::helper::read_data;
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day2.txt");
    let presents: Vec<(u32, u32, u32)> = get_presents(&data);

    let (p1, p2): (u32, u32) = calculate_wrap_and_ribbon(&presents);

    println!("{p1}\n{p2}");
}

fn get_presents(data: &String) -> Vec<(u32, u32, u32)> {

    let mut presents: Vec<(u32, u32, u32)> = Vec::new();

    for present in data.lines() {
        let dims = present.split('x').map(|d| d.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        let (l, w, h): (u32, u32, u32) = (dims[0], dims[1], dims[2]);

        presents.push((l, w, h));
    }

    presents


}

fn calculate_wrap_and_ribbon(presents: &Vec<(u32, u32, u32)>) -> (u32, u32) {
    let mut wrap: u32 = 0;
    let mut ribbon: u32 = 0;

    for (l, w, h) in presents {
        wrap += 2*l*w + 2*w*h + 2*l*h;
        wrap += vec![l*w, w*h, l*h].iter().min().unwrap();

        ribbon += l*w*h;
        ribbon += vec![2*(l+w), 2*(w+h), 2*(l+h)].iter().min().unwrap();
    }

    (wrap, ribbon)
}