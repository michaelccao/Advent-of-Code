use crate::helper::read_data;
use regex::Regex;

pub fn main() {
    
    let data = read_data("../Data/Day7.txt");

    let eqns = get_equations(&data);

    println!("{:?}", solve(&eqns, false));

    println!("{:?}", solve(&eqns, true));
    
}

fn get_equations(data: &String) -> Vec<Vec<i64>> {

    let mut eqns: Vec<Vec<i64>> = Vec::new();

    let re = Regex::new(r"(\d+)").unwrap();

    for line in data.split("\n") {
        let mut eqn: Vec<i64> = Vec::new();

        let nums = re.captures_iter(line);
        
        for num in nums {
            eqn.push(num[1].parse::<i64>().unwrap());
        }

        eqns.push(eqn);
    }

    eqns

}

fn is_valid_eq(eq: &Vec<i64>, part2: bool) -> bool {
    let total = eq[0];

    let calc = eq[1];

    if calc > total { return false };

    let plus = calc + eq[2];
    let mult = calc * eq[2];
    let stradd = (calc.to_string() + &eq[2].to_string()).parse::<i64>().unwrap();

    if eq.len() == 3 {
        return total == plus || total == mult || (part2 && total == stradd )
    } else {
        for calc2 in [plus, mult, stradd] {
            if calc2 == stradd && !part2 {
                continue;
            }
            let mut eq2 = eq.clone();
            eq2.remove(2);
            eq2[1] = calc2;

            if is_valid_eq(&eq2, part2) {
                return true
            }
        }
    }

    false
}

fn solve(eqns: &Vec<Vec<i64>>, part2: bool) -> i64 {
    let mut total:i64 = 0;

    for eq in eqns {
        if is_valid_eq(&eq, part2) {
            total += eq[0];
        }
    }

    total
}