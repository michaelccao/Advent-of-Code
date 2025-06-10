use crate::helper::read_data;
use std::collections::HashSet;

pub fn main() {
    let data: String = read_data("../Data/Day06.txt");
    let group_answers: Vec<Vec<&str>> = data
        .split("\r\n\r\n")
        .map(|answers| answers.lines().collect())
        .collect();

    let p1: usize = group_answers
        .iter()
        .map(|answers| count_unique_answers(answers))
        .sum();

    println!("{p1}");

    let p2: usize = group_answers
        .iter()
        .map(|answers| count_common_answers(answers))
        .sum();

    println!("{p2}");
}

fn count_unique_answers(answers: &Vec<&str>) -> usize {
    let mut unique_answers: HashSet<char> = HashSet::new();

    for &answer in answers {
        for q in answer.chars() {
            unique_answers.insert(q);
        }
    }

    unique_answers.len()
}

fn count_common_answers(answers: &Vec<&str>) -> usize {
    let mut common_answers: HashSet<char> = answers[0].chars().collect();

    for i in 1..answers.len() {
        let common_answers2: HashSet<char> = answers[i].chars().collect();

        common_answers = common_answers
            .intersection(&common_answers2)
            .cloned()
            .collect();
    }

    common_answers.len()
}
