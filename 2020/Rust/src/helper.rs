#![allow(dead_code)]
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn read_data(path_str: &str) -> String {
    let mut file = match File::open(Path::new(&path_str)) {
        Err(_) => panic!("File not found"),
        Ok(file) => file,
    };

    let mut data = String::new();

    match file.read_to_string(&mut data) {
        Err(_) => panic!("Could not read"),
        Ok(_) => (),
    };

    data
}

pub fn neighbors(i: usize, j: usize, rows: usize, cols: usize, diags: bool) -> Vec<(usize, usize)> {
    let mut nbs: Vec<(usize, usize)> = Vec::new();

    if i > 0 {
        nbs.push((i - 1, j));
    }

    if j > 0 {
        nbs.push((i, j - 1));
    }

    if i + 1 < rows {
        nbs.push((i + 1, j));
    }

    if j + 1 < cols {
        nbs.push((i, j + 1));
    }

    if diags {
        if i > 0 && j > 0 {
            nbs.push((i - 1, j - 1));
        }

        if i > 0 && j + 1 < cols {
            nbs.push((i - 1, j + 1));
        }

        if i + 1 < rows && j > 0 {
            nbs.push((i + 1, j - 1));
        }

        if i + 1 < rows && j + 1 < cols {
            nbs.push((i + 1, j + 1));
        }
    }

    nbs
}
