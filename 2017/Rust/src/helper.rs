use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

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