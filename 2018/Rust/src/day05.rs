use crate::helper::read_data;

pub fn main() {
    let data: String = read_data("../Data/Day05.txt");

    let p1: usize = simplify(&data, None);

    let p2: usize = super_simplify(&data);

    println!("{p1}\n{p2}");
}

fn simplify(s: &String, skip: Option<char>) -> usize {
    let mut s2: String = String::new();

    for c in s.chars() {
        if let Some(skip_c) = skip {
            if c.to_ascii_lowercase() == skip_c {
                continue;
            }
        }

        if s2.len() > 0 {
            let last_char: char = s2.pop().unwrap();

            if last_char.to_ascii_lowercase() != c.to_ascii_lowercase()
                || last_char.is_lowercase() == c.is_lowercase()
            {
                s2.push(last_char);
                s2.push(c);
            }
        } else {
            s2.push(c);
        }
    }

    s2.len()
}

fn super_simplify(s: &String) -> usize {
    let mut shortest: usize = s.len();
    let a: u8 = 'a' as u8;

    for i in 0..26 {
        let c: char = (a + i) as char;

        let l: usize = simplify(s, Some(c));

        if l < shortest {
            shortest = l;
        }
    }

    shortest
}
