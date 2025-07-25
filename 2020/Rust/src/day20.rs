use crate::helper::read_data;
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

    let matches = find_matches(&tiles);

    // Turns out all each tile has a unique matching edge
    for (_tile_num, edge_matches) in &matches {
        for tile_matches in edge_matches {
            assert!(tile_matches.len() <= 1);
        }
    }

    let p1 = find_corners(&matches);

    println!("{p1}");

    println!("{:?}", matches.len());
}

fn rotate_90_ccw(tile: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut tile2 = tile.clone();

    for i in 0..tile.len() {
        for j in 0..tile[i].len() {
            tile2[tile[i].len() - j - 1][i] = tile[i][j];
        }
    }

    tile2
}

fn vertical_flip(tile: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut tile2 = tile.clone();

    tile2.reverse();

    tile2
}

fn horizontal_flip(tile: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut tile2 = tile.clone();

    for i in 0..tile2.len() {
        tile2[i].reverse();
    }

    tile2
}

fn find_corners(matches: &HashMap<u64, [HashSet<(u64, usize)>;4]>) -> u64 {

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

    println!("{corners:?}");

    corners.iter().fold(1,|acc, &e| acc*e)
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
            tile_num = line.trim().trim_end_matches(':').split_whitespace().last().unwrap().parse().unwrap();
        } else {
            let row: Vec<bool> = line.trim().chars().map(|c| c == '#').collect();
            tile.push(row);
        }
    }
    println!("{tile:?}");
    // Add last tile
    tiles.insert(tile_num, tile.clone());

    tiles
}

fn get_tile_edges(tile: &Vec<Vec<bool>>) -> [Vec<bool>;4] {

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

fn find_matches(tiles: &HashMap<u64, Vec<Vec<bool>>>) -> HashMap<u64, [HashSet<(u64, usize)>;4]> {

    let mut matches: HashMap<u64, [HashSet<(u64, usize)>;4]> = HashMap::new();

    let orientations: [Vec<fn(&Vec<Vec<bool>>) -> Vec<Vec<bool>>>; 8] = [
        vec![],
        vec![vertical_flip],
        vec![horizontal_flip],
        vec![vertical_flip, horizontal_flip],
        vec![rotate_90_ccw],
        vec![rotate_90_ccw, vertical_flip],
        vec![rotate_90_ccw, horizontal_flip],
        vec![rotate_90_ccw, vertical_flip, horizontal_flip]
    ];

    for (tile_num, tile) in tiles {
        matches.insert(*tile_num, [HashSet::new(),
        HashSet::new(),
        HashSet::new(),
        HashSet::new()]);

        let edges = get_tile_edges(tile);

        for (tile_num2, tile2) in tiles {
            if tile_num == tile_num2 {
                continue;
            }
            
            for i in 0..orientations.len() {
                let mut tile2_2 = tile2.clone();
                for op in &orientations[i] {
                    tile2_2 = op(&tile2_2);
                }

                let edges2 = get_tile_edges(&tile2_2);

                for j in 0..edges.len() {
                    if edges[j] == edges2[(j+2) % edges.len()] {
                        matches.get_mut(tile_num).unwrap()[j].insert((*tile_num2, i));
                    }
                }
            }
        }
    }

    matches

}

