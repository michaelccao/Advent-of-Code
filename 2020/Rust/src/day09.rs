use crate::helper::read_data;

pub fn main() {
    let data: String = read_data("../Data/Day09.txt");
    let nums: Vec<i64> = data.lines().map(|line| line.parse().unwrap()).collect();

    let p1: i64 = *nums.iter().enumerate().find(|&(i, _)| !is_valid(&nums, i)).unwrap().1;

    println!("{p1}");

    let p2: i64 = find_set(&nums, p1);

    println!("{p2}");
}

fn binary_search(nums: &[i64], target: i64) -> Option<usize> {
    let mut i: usize = 0;
    let mut j: usize = nums.len();

    while i < j-1 {
        let mid: usize = (i+j)/2;

        if nums[mid] < target {
            i = mid;
        } else if nums[mid] > target {
            j = mid;
        } else if nums[mid] == target {
            return Some(mid);
        }
    }

    if nums[i] == target {
        return Some(i);
    } else {
        return None;
    }
}

fn is_valid(nums: &Vec<i64>, target: usize) -> bool {
    if target < 25 {
        return true;
    }

    let mut previous_nums: Vec<i64> = nums[target-25..target].iter().cloned().collect();

    previous_nums.sort_unstable();

    for i in 0..previous_nums.len() {
        let diff = nums[target]-previous_nums[i];
        if diff < previous_nums[i] {
            continue;
        } else {
            if binary_search(&previous_nums[i+1..], diff).is_some() {
                return true;
            }
        }
    }

    false

}

fn find_set(nums: &Vec<i64>, target: i64) -> i64 {

    for i in 0..nums.len() {
        let mut running_sum: i64 = nums[i];
        let mut min: i64 = nums[i];
        let mut max: i64 = nums[i];

        for j in i+1..nums.len() {
            running_sum += nums[j];
            min = min.min(nums[j]);
            max = max.max(nums[j]);

            if running_sum == target {
                return min + max;    
            } else if running_sum > target {
                break;
            }
        }
    }

    -1
}