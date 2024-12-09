use crate::helper::read_data;
use std::collections::HashMap;

pub fn main() {
    
    let data = read_data("../Data/Day9.txt");

    let p1_ans = p1(&data);
    println!("{:?}", p1_ans);

    let p2_ans = p2(&data);
    println!("{:?}", p2_ans);
    
}

fn p1(data: &String) -> i64 {
    let mut files: Vec<i64> = Vec::new();
    let mut is_file: bool = true;
    let mut file_num: i64 = 0;

    for c in data.chars() {
        if is_file {
            files.append(&mut vec![file_num; c.to_digit(10).unwrap() as usize]);
            file_num += 1;
        } else {
            files.append(&mut vec![-1; c.to_digit(10).unwrap() as usize]);
        }
        is_file = !is_file;
    }

    let mut i:usize = 0;
    let mut j:usize = files.len() - 1;

    while i < j {
        if files[i] != -1 {
            i += 1;
        }

        if files[j] == -1 {
            j -= 1;
        }

        if files[i] == -1 && files[j] != -1 {
            files[i] = files[j];
            files[j] = -1;
            i += 1;
            j -= 1;
        }
    }


    files.iter().enumerate().map(|(i, file)| if *file > 0 {i as i64 * *file} else {0}).sum::<i64>()
}

fn p2(data: &String) -> i64 {
    let mut files: HashMap<i64, (usize, usize)> = HashMap::new();
    let mut spaces: Vec<(usize, usize)> = Vec::new();
    let mut file_num: i64 = 0;
    let mut is_file: bool = true;
    let mut pointer: usize = 0;

    for c in data.chars() {
        let space = c.to_digit(10).unwrap() as usize;

        if is_file {
            files.insert(file_num, (pointer, space));
            file_num += 1;
        } else {
            spaces.push((pointer, space));
        }

        pointer += space;
        is_file = !is_file;
    }

    for file in (0..file_num).rev() {
        let (fp, fsize) = files.get_mut(&file).unwrap();
        let fp = *fp;
        let fsize = *fsize;

        for i in 0..spaces.len() {
            let (sp, space) = spaces[i];

            if sp > fp {
                break;
            }

            if fsize == space {
                files.insert(file, (sp, fsize));
                spaces.remove(i);
                break

            } else if fsize < space {
                files.insert(file, (sp, fsize));
                spaces[i] = (sp + fsize, space - fsize);
                break
            }
        }
    }

    files.iter().map(|(file, (pointer, space))| *file*(((2*pointer + space - 1)*space/2) as i64)).sum::<i64>()
}