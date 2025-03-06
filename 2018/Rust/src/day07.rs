use crate::helper::read_data;
use std::collections::{HashMap, HashSet, VecDeque};
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day07.txt");

    let (steps, requirements) = get_requirements(&data);

    let p1: String = order_steps(&steps, &requirements);

    let p2: u32 = multi_step(&steps, &requirements);

    println!("{p1}\n{p2}");
}

fn get_requirements(data: &String) -> (Vec<String>, HashMap<String, HashSet<String>>) {
    let mut requirements: HashMap<String, HashSet<String>> = HashMap::new();
    let mut steps: HashSet<String> = HashSet::new();

    for line in data.lines() {
        let line: Vec<&str> = line.trim().split_whitespace().collect();
        let step1: String = line[1].to_string();
        let step2: String = line[7].to_string();

        steps.insert(step1.clone());
        steps.insert(step2.clone());

        if let Some(reqs) = requirements.get_mut(&step2) {
            reqs.insert(step1);
        } else {
            requirements.insert(step2, HashSet::from([step1]));
        }
    }

    let mut steps: Vec<String> = steps.iter().cloned().collect();
    steps.sort();

    (steps, requirements)
}

fn order_steps(steps: &Vec<String>, requirements: &HashMap<String, HashSet<String>>) -> String {
    let mut steps_done: HashSet<String> = HashSet::new();
    let mut output: String = String::new();

    while steps_done.len() < steps.len() {
        for step in steps {
            if steps_done.contains(step) {
                continue;
            }

            let reqs = requirements.get(step);

            if reqs.is_none() || reqs.unwrap().is_subset(&steps_done) {
                steps_done.insert(step.clone());
                output = format!("{}{}", output, step);
                break;
            }
        }
    }

    output
}

fn multi_step(steps: &Vec<String>, requirements: &HashMap<String, HashSet<String>>) -> u32 {
    let mut steps_done: HashSet<String> = HashSet::new();
    let mut steps_queued: HashSet<String> = HashSet::new();

    let mut t: u32 = 0;

    let mut costs: HashMap<String, i32> = HashMap::new();

    for i in 0..steps.len() {
        costs.insert(steps[i].clone(), 60 + i as i32);
    }

    let mut workers: [(Option<String>, i32); 5] =
        [(None, 0), (None, 0), (None, 0), (None, 0), (None, 0)];

    let mut steps_avail: VecDeque<String> =
        get_available_steps(steps, requirements, &steps_done, &steps_queued);

    while steps_done.len() < steps.len() {
        for i in 0..workers.len() {
            if workers[i].1 == 0 {
                if let Some(job) = steps_avail.pop_front() {
                    steps_queued.insert(job.clone());
                    workers[i].0 = Some(job.clone());
                    workers[i].1 = costs[&job];
                }
            } else if workers[i].1 == 1 {
                steps_done.insert(workers[i].0.clone().unwrap());
                workers[i].1 = 0;
            } else {
                workers[i].1 -= 1;
            }
        }

        steps_avail = get_available_steps(steps, requirements, &steps_done, &steps_queued);

        t += 1;
    }

    t
}

fn get_available_steps(
    steps: &Vec<String>,
    requirements: &HashMap<String, HashSet<String>>,
    steps_done: &HashSet<String>,
    steps_queued: &HashSet<String>,
) -> VecDeque<String> {
    let mut avail: VecDeque<String> = VecDeque::new();

    for step in steps {
        if steps_queued.contains(step) {
            continue;
        }

        let reqs = requirements.get(step);

        if reqs.is_none() || reqs.unwrap().is_subset(steps_done) {
            avail.push_back(step.clone());
        }
    }

    avail
}
