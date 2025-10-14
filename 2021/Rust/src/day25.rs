use crate::helper::read_data;
use std::collections::HashSet;

pub fn main() {
    let data: String = read_data("../Data/Day25.txt");

    let cucumbers: Cucumbers = Cucumbers::from_string(&data);

    let p1: usize = cucumbers.migrate_to_stop();

    println!("{p1}");
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Cucumbers {
    east: HashSet<(usize, usize)>,
    south: HashSet<(usize, usize)>,
    dimensions: (usize, usize),
}

impl Cucumbers {
    fn from_string(s: &String) -> Self {
        let mut i_max: usize = 0;
        let mut j_max: usize = 0;

        let mut cucumbers: Cucumbers = Cucumbers {
            east: HashSet::new(),
            south: HashSet::new(),
            dimensions: (0, 0),
        };

        for (i, row) in s.lines().enumerate() {
            i_max = i_max.max(i);
            for (j, c) in row.chars().enumerate() {
                j_max = j_max.max(j);

                if c == '>' {
                    cucumbers.east.insert((i, j));
                } else if c == 'v' {
                    cucumbers.south.insert((i, j));
                }
            }
        }

        cucumbers.dimensions = (i_max + 1, j_max + 1);

        cucumbers
    }

    fn migrate(&self) -> Self {
        let mut c2: Cucumbers = self.clone();

        let (rows, cols) = self.dimensions;

        for &(i, j) in &self.east {
            let j2: usize = (j + 1) % cols;

            if !self.east.contains(&(i, j2)) && !self.south.contains(&(i, j2)) {
                c2.east.remove(&(i, j));
                c2.east.insert((i, j2));
            }
        }

        for &(i, j) in &self.south {
            let i2: usize = (i + 1) % rows;

            if !c2.east.contains(&(i2, j)) && !self.south.contains(&(i2, j)) {
                c2.south.remove(&(i, j));
                c2.south.insert((i2, j));
            }
        }

        c2
    }

    fn migrate_to_stop(&self) -> usize {
        let mut steps: usize = 1;

        let mut prev: Cucumbers = self.clone();
        let mut c2: Cucumbers = self.migrate();

        while c2 != prev {
            prev = c2.clone();
            c2 = c2.migrate();
            steps += 1;
        }

        steps
    }
}
