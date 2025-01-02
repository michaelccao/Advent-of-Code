use crate::helper::read_data;
use std::vec::Vec;

pub fn main() {
    let data: String = read_data("../Data/Day22.txt");
    let boss: Vec<i32> = data
        .lines()
        .map(|line| {
            line.split(": ").collect::<Vec<&str>>()[1]
                .parse::<i32>()
                .unwrap()
        })
        .collect();

    let p1: i32 = fight(&boss, false);
    let p2: i32 = fight(&boss, true);

    println!("{p1}\n{p2}");
}

fn fight(boss: &Vec<i32>, part2: bool) -> i32 {
    let (boss_hp, boss_atk): (i32, i32) = (boss[0], boss[1]);

    let mut best_win: i32 = 999999;

    let mut nodes: Vec<(i32, i32, i32, i32, i32, i32, i32)> = vec![(50, 500, boss_hp, 0, 0, 0, 0)];

    let spells: [(i32, i32, i32, i32, i32, i32); 5] = [
        (53, 4, 0, 0, 0, 0),
        (73, 2, 2, 0, 0, 0),
        (113, 0, 0, 6, 0, 0),
        (173, 0, 0, 0, 6, 0),
        (229, 0, 0, 0, 0, 5),
    ];

    while nodes.len() > 0 {
        let (mut hp, mut mana, mut boss_hp, mana_spent, mut shield, mut poison, mut recharge): (
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
        ) = nodes.pop().unwrap();

        if part2 {
            hp -= 1;
        }

        if hp <= 0 {
            continue;
        }

        // Player Turn
        if mana_spent >= best_win {
            continue;
        }

        // Resolve effects
        (mana, _, boss_hp, shield, poison, recharge) =
            resolve_effects(mana, boss_hp, shield, poison, recharge);

        if boss_hp <= 0 && hp > 0 {
            if mana_spent < best_win {
                best_win = mana_spent;
            }
            continue;
        }

        // Resolve action
        if mana < spells[0].0 {
            continue;
        }

        for (cost, dmg, heal, add_shld, add_psn, add_rechg) in spells {
            // Don't cast if insufficient mana
            if cost > mana {
                continue;
            }

            // Don't cast effect if effect still in play
            if shield > 0 && add_shld > 0 {
                continue;
            }

            if poison > 0 && add_psn > 0 {
                continue;
            }

            if recharge > 0 && add_rechg > 0 {
                continue;
            }

            let mut hp2 = hp + heal;
            let mut mana2 = mana - cost;
            let mut boss_hp2 = boss_hp - dmg;
            let mana_spent2 = mana_spent + cost;
            let mut shield2 = shield + add_shld;
            let mut poison2 = poison + add_psn;
            let mut recharge2 = recharge + add_rechg;

            // End of Player Turn, check win
            if boss_hp2 <= 0 && hp2 > 0 {
                if mana_spent2 < best_win {
                    best_win = mana_spent2;
                }
                continue;
            }

            // Boss Turn

            // Resolve effects
            let armor: i32;
            (mana2, armor, boss_hp2, shield2, poison2, recharge2) =
                resolve_effects(mana2, boss_hp2, shield2, poison2, recharge2);

            // Check win
            if boss_hp2 <= 0 && hp2 > 0 {
                if mana_spent2 < best_win {
                    best_win = mana_spent2
                }
                continue;
            }

            // Boss attack
            if boss_atk > armor {
                hp2 -= boss_atk - armor;
            } else {
                hp2 -= 1;
            }

            // End of Boss Turn, check loss
            if hp2 <= 0 {
                continue;
            }

            nodes.push((
                hp2,
                mana2,
                boss_hp2,
                mana_spent2,
                shield2,
                poison2,
                recharge2,
            ));
        }
    }

    best_win
}

fn resolve_effects(
    mut mana: i32,
    mut boss_hp: i32,
    mut shield: i32,
    mut poison: i32,
    mut recharge: i32,
) -> (i32, i32, i32, i32, i32, i32) {
    let armor: i32;

    if shield > 0 {
        shield -= 1;
        armor = 7;
    } else {
        armor = 0;
    }

    if poison > 0 {
        poison -= 1;
        boss_hp -= 3;
    }

    if recharge > 0 {
        recharge -= 1;
        mana += 101;
    }

    (mana, armor, boss_hp, shield, poison, recharge)
}
