use crate::helper::read_data;
use std::collections::HashMap;

pub fn main() {
    let data: String = read_data("../Data/Day08.txt");
    let (p1, p2) = get_image(&data, 6, 25);

    println!("{p1}");
    println!("{p2}");
}

fn get_image(data: &String, height: usize, width: usize) -> (usize, String) {
    let num_layers = data.len() / height / width;
    let mut image: Vec<Vec<Vec<u32>>> = vec![vec![vec![0; width]; height]; num_layers];

    let mut layer_count: HashMap<u32, usize> = HashMap::new();

    let mut fewest_zeros: usize = height * width;

    let mut answer: usize = 0;

    for (i, c) in data.chars().enumerate() {
        let layer_num: usize = i / height / width;
        let row_num: usize = i % (height * width) / width;
        let col_num: usize = i % (height * width) % width;

        let digit: u32 = c.to_digit(10).unwrap();

        image[layer_num][row_num][col_num] = digit;

        if let Some(count) = layer_count.get_mut(&digit) {
            *count += 1;
        } else {
            layer_count.insert(digit, 1);
        }

        if row_num == height - 1 && col_num == width - 1 {
            let layer_zeros: usize = *layer_count.get(&0).unwrap_or(&0);

            if layer_zeros < fewest_zeros {
                fewest_zeros = layer_zeros;
                let layer_ones: usize = *layer_count.get(&1).unwrap_or(&0);
                let layer_twos: usize = *layer_count.get(&2).unwrap_or(&0);

                answer = layer_ones * layer_twos;
            }

            layer_count = HashMap::new();
        }
    }

    let mut image_str: String = String::new();

    for i in 0..height {
        for j in 0..width {
            for k in 0..num_layers {
                if image[k][i][j] == 0 {
                    image_str.push(' ');
                    break;
                } else if image[k][i][j] == 1 {
                    image_str.push('#');
                    break;
                }
            }
            if j == width - 1 {
                image_str.push('\n');
            }
        }
    }

    (answer, image_str)
}
