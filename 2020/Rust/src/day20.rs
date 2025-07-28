use crate::helper::read_data;
use itertools::all;
use std::collections::{HashMap, HashSet};

pub fn main() {
    let data: String = read_data("../Data/Day20.txt");

    let tiles: HashMap<u64, Vec<Vec<bool>>> = get_tiles(&data);

    // Tiles are 10x10
    // A vertical flip + horizontal flip = 180 degrees
    // Therefore 180 degrees + vert = horizontal flip
    // 180 degrees + horiz = vertical flip
    // 8 Orientations:
    // 0: Original
    // 1: Vertical Flip
    // 2: Horizontal Flip
    // 3: Vertical + Horizontal Flip
    // 4: 90 Degrees CCW
    // 5: 90 Degrees + Vertical
    // 6: 90 Degrees + Horizontal
    // 7: 90 Degrees + Vertical + Horizontal

    // Another way to think of orientations is that
    // corners must be in ABCD order
    // A can be at 4 corners
    // and can go in 2 directions
    // So there must be 8 unique orientations

    // Edges:
    // 0: Top
    // 1: Left
    // 2: Bottom
    // 3: Right

    // We can start with any tile in default orientation
    // and build from there

    // Outermost edges will not align with any tiles

    let matches: HashMap<u64, [HashSet<(u64, usize)>; 4]> = find_matches(&tiles);

    // Turns out all each tile has a unique matching edge
    for (_tile_num, edge_matches) in &matches {
        for tile_matches in edge_matches {
            assert!(tile_matches.len() <= 1);
        }
    }

    let (_corners, p1) = find_corners(&matches);

    println!("{p1}");

    let map: Vec<Vec<bool>> = create_map(&tiles);

    let monster: Vec<Vec<bool>> = vec![
        vec![
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, true, false,
        ],
        vec![
            true, false, false, false, false, true, true, false, false, false, false, true, true,
            false, false, false, false, true, true, true,
        ],
        vec![
            false, true, false, false, true, false, false, true, false, false, true, false, false,
            true, false, false, true, false, false, false,
        ],
    ];

    let monster_coords: Vec<(usize, usize, Vec<(usize, usize)>)> = get_monster_coords(&monster);

    let p2: usize = find_monsters(&map, &monster_coords);

    //2486 too high
    println!("{p2}");
}

fn find_monsters(
    map: &Vec<Vec<bool>>,
    monster_coords: &Vec<(usize, usize, Vec<(usize, usize)>)>,
) -> usize {
    let mut safe: Vec<Vec<bool>> = map.clone();

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            for (h, w, coords) in monster_coords {
                if i + h - 1 >= map.len() || j + w - 1 >= map[i].len() {
                    continue;
                }

                if all(coords.iter(), |(di, dj)| map[i + di][j + dj]) {
                    for (di, dj) in coords {
                        safe[i + di][j + dj] = false;
                    }
                }
            }
        }
    }

    safe.iter()
        .map(|row| row.iter().filter(|&is_safe| *is_safe).count())
        .sum::<usize>()
}

fn get_monster_coords(monster: &Vec<Vec<bool>>) -> Vec<(usize, usize, Vec<(usize, usize)>)> {
    let mut coords: Vec<(usize, usize, Vec<(usize, usize)>)> = Vec::new();

    let orientations: [Vec<fn(&Vec<Vec<bool>>) -> Vec<Vec<bool>>>; 8] = [
        vec![],
        vec![vertical_flip],
        vec![horizontal_flip],
        vec![vertical_flip, horizontal_flip],
        vec![rotate_90_ccw],
        vec![rotate_90_ccw, vertical_flip],
        vec![rotate_90_ccw, horizontal_flip],
        vec![rotate_90_ccw, vertical_flip, horizontal_flip],
    ];

    for orient in &orientations {
        let mut monster2 = monster.clone();
        for op in orient {
            monster2 = op(&monster2);
        }

        let mut monster_coords: Vec<(usize, usize)> = Vec::new();
        for i in 0..monster2.len() {
            for j in 0..monster2[i].len() {
                if monster2[i][j] {
                    monster_coords.push((i, j));
                }
            }
        }

        coords.push((monster2.len(), monster2[0].len(), monster_coords));
    }

    coords
}

fn rotate_90_ccw(tile: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut tile2 = vec![vec![false; tile.len()]; tile[0].len()];

    for i in 0..tile.len() {
        for j in 0..tile[i].len() {
            tile2[tile[i].len() - j - 1][i] = tile[i][j];
        }
    }

    tile2
}

fn vertical_flip(tile: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut tile2: Vec<Vec<bool>> = tile.clone();

    tile2.reverse();

    tile2
}

fn horizontal_flip(tile: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut tile2: Vec<Vec<bool>> = tile.clone();

    for i in 0..tile2.len() {
        tile2[i].reverse();
    }

    tile2
}

fn find_corners(matches: &HashMap<u64, [HashSet<(u64, usize)>; 4]>) -> (Vec<u64>, u64) {
    let mut corners: Vec<u64> = Vec::new();

    for (tile, sides) in matches {
        let mut side_matches: usize = 0;
        for matching_tiles in sides {
            side_matches += matching_tiles.len();
        }

        if side_matches == 2 {
            corners.push(*tile);
        }
    }

    (corners.clone(), corners.iter().fold(1, |acc, &e| acc * e))
}

fn get_tiles(data: &String) -> HashMap<u64, Vec<Vec<bool>>> {
    let mut tiles: HashMap<u64, Vec<Vec<bool>>> = HashMap::new();

    let mut tile: Vec<Vec<bool>> = Vec::new();
    let mut tile_num: u64 = 0;

    for line in data.lines() {
        if line.trim().len() == 0 {
            tiles.insert(tile_num, tile.clone());
            tile = Vec::new();
        } else if line.starts_with("Tile") {
            tile_num = line
                .trim()
                .trim_end_matches(':')
                .split_whitespace()
                .last()
                .unwrap()
                .parse()
                .unwrap();
        } else {
            let row: Vec<bool> = line.trim().chars().map(|c| c == '#').collect();
            tile.push(row);
        }
    }

    // Add last tile
    tiles.insert(tile_num, tile.clone());

    tiles
}

fn get_tile_edges(tile: &Vec<Vec<bool>>) -> [Vec<bool>; 4] {
    let mut top: Vec<bool> = Vec::new();
    let mut bottom: Vec<bool> = Vec::new();
    let mut left: Vec<bool> = Vec::new();
    let mut right: Vec<bool> = Vec::new();

    for i in 0..tile.len() {
        for j in 0..tile[0].len() {
            if i == 0 {
                top.push(tile[i][j]);
            }

            if i == tile.len() - 1 {
                bottom.push(tile[i][j]);
            }

            if j == 0 {
                left.push(tile[i][j]);
            }

            if j == tile[0].len() - 1 {
                right.push(tile[i][j]);
            }
        }
    }

    [top, left, bottom, right]
}

fn find_matches(tiles: &HashMap<u64, Vec<Vec<bool>>>) -> HashMap<u64, [HashSet<(u64, usize)>; 4]> {
    let mut matches: HashMap<u64, [HashSet<(u64, usize)>; 4]> = HashMap::new();

    let orientations: [Vec<fn(&Vec<Vec<bool>>) -> Vec<Vec<bool>>>; 8] = [
        vec![],
        vec![vertical_flip],
        vec![horizontal_flip],
        vec![vertical_flip, horizontal_flip],
        vec![rotate_90_ccw],
        vec![rotate_90_ccw, vertical_flip],
        vec![rotate_90_ccw, horizontal_flip],
        vec![rotate_90_ccw, vertical_flip, horizontal_flip],
    ];

    for (tile_num, tile) in tiles {
        matches.insert(
            *tile_num,
            [
                HashSet::new(),
                HashSet::new(),
                HashSet::new(),
                HashSet::new(),
            ],
        );

        let edges: [Vec<bool>; 4] = get_tile_edges(tile);

        for (tile_num2, tile2) in tiles {
            if tile_num == tile_num2 {
                continue;
            }

            for i in 0..orientations.len() {
                let mut tile2_2: Vec<Vec<bool>> = tile2.clone();
                for op in &orientations[i] {
                    tile2_2 = op(&tile2_2);
                }

                let edges2: [Vec<bool>; 4] = get_tile_edges(&tile2_2);

                for j in 0..edges.len() {
                    if edges[j] == edges2[(j + 2) % edges.len()] {
                        matches.get_mut(tile_num).unwrap()[j].insert((*tile_num2, i));
                    }
                }
            }
        }
    }

    matches
}

fn create_map(tiles: &HashMap<u64, Vec<Vec<bool>>>) -> Vec<Vec<bool>> {
    let mut nodes: Vec<(i32, i32)> = Vec::new();
    nodes.push((0, 0));

    let mut tile_map: HashMap<(i32, i32), (u64, Vec<Vec<bool>>)> = HashMap::new();

    let (first_tile_num, first_tile) = tiles.iter().next().unwrap().clone();

    tile_map.insert((0, 0), (*first_tile_num, first_tile.clone()));

    let [mut i0, mut i1, mut j0, mut j1]: [i32; 4] = [0; 4];

    let orientations: [Vec<fn(&Vec<Vec<bool>>) -> Vec<Vec<bool>>>; 8] = [
        vec![],
        vec![vertical_flip],
        vec![horizontal_flip],
        vec![vertical_flip, horizontal_flip],
        vec![rotate_90_ccw],
        vec![rotate_90_ccw, vertical_flip],
        vec![rotate_90_ccw, horizontal_flip],
        vec![rotate_90_ccw, vertical_flip, horizontal_flip],
    ];

    while nodes.len() > 0 {
        let (i, j) = nodes.pop().unwrap();

        let (tile_num, tile) = &tile_map[&(i, j)].clone();

        let [t, l, b, r] = get_tile_edges(tile);

        for (tile_num2, tile2) in tiles {
            if tile_num == tile_num2 {
                continue;
            }

            for orient in &orientations {
                let mut tile2_2 = tile2.clone();
                for op in orient {
                    tile2_2 = op(&tile2_2);
                }

                let [t2, l2, b2, r2] = get_tile_edges(&tile2_2);

                let mut i2 = i;
                let mut j2 = j;

                if t == b2 {
                    i2 -= 1;
                } else if l == r2 {
                    j2 -= 1;
                } else if b == t2 {
                    i2 += 1;
                } else if r == l2 {
                    j2 += 1;
                }

                if (i != i2 || j != j2) && !tile_map.contains_key(&(i2, j2)) {
                    tile_map.insert((i2, j2), (*tile_num2, tile2_2.clone()));
                    nodes.push((i2, j2));

                    i0 = i0.min(i2);
                    i1 = i1.max(i2);

                    j0 = j0.min(j2);
                    j1 = j1.max(j2);

                    break;
                }
            }
        }
    }

    let tile_h: usize = first_tile.len();
    let tile_w: usize = first_tile[0].len();

    let rows: usize = (i1 - i0 + 1) as usize;
    let cols: usize = (j1 - j0 + 1) as usize;

    let mut stitch: Vec<Vec<bool>> = vec![vec![false; cols * (tile_w - 2)]; rows * (tile_h - 2)];

    for i in 0..stitch.len() {
        for j in 0..stitch[i].len() {
            let tile_i: i32 = i0 + (i / (tile_h - 2)) as i32;
            let tile_j: i32 = j0 + (j / (tile_w - 2)) as i32;

            let k: usize = i % (tile_h - 2);
            let l: usize = j % (tile_w - 2);

            stitch[i][j] = tile_map[&(tile_i, tile_j)].1[1 + k][1 + l];
        }
    }

    stitch
}
