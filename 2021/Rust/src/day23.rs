use crate::helper::read_data;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::cmp::Ordering;

pub fn main() {
    let data: String = read_data("../Data/Day23.txt");

    let state: State = State::from_string(&data);

    let p1: usize = state.rearrange();

    println!("{p1}");

    let mut data2: Vec<&str> = data.lines().collect();

    data2.insert(3,"  #D#C#B#A#");
    data2.insert(4,"  #D#B#A#C#");

    let data2: String = data2.join("\n");

    let state2: State = State::from_string(&data2);

    let p2: usize = state2.rearrange();

    println!("{p2}");

}

#[derive(Eq, PartialEq, Clone, Debug)]
struct State {
    base_cost: usize,
    total_cost: usize,
    tiles: Vec<Vec<char>>,
    hallway: HashMap<usize, char>,
    rooms: [(Vec<char>, (usize, usize));4],
}

impl State {
    fn lowball_total_cost(&self) -> usize {
        let mut total: usize = 0;

        if self.is_hallway_blocked() {
            return usize::MAX
        }

        let mappings: HashMap<char, (usize, usize)> = HashMap::from([
            ('A', (3, 1)),
            ('B', (5, 10)),
            ('C', (7, 100)),
            ('D', (9, 1000)),
        ]);

        let mut counter: HashMap<char, usize> = HashMap::from([
            ('A', 0),
            ('B', 0),
            ('C', 0),
            ('D', 0),
        ]);

        for (&j, &c) in &self.hallway {
            let (jf, cost) = mappings[&c];

            total += (1 + jf.abs_diff(j))*cost;
        }

        for room in 0..self.rooms.len() {
            let (occupants, (i_top, j)) = &self.rooms[room];
            let bottom = i_top + occupants.len() - 1;
            for (k, c) in occupants.iter().enumerate() {
                let i = bottom - k;
                let (jf, cost) = mappings[c];

                total += (i + jf.abs_diff(*j) + counter[c])*cost;

                *counter.get_mut(c).unwrap() += 1;
            }
        }

        self.base_cost + total

    }

    fn from_string(data: &String) -> Self {
        let mut tiles: Vec<Vec<char>> = Vec::new();

        let mut hallway: HashMap<usize, char> = HashMap::new();

        for (i, line) in data.lines().enumerate() {
            let mut row: Vec<char> = Vec::new();
            for (j, c) in line.chars().enumerate() {
                row.push(c);
                if c.is_alphabetic() && i == 1 {
                    hallway.insert(j, c);
                }
            }
            tiles.push(row);
        }

        let mut rooms: [(Vec<char>, (usize, usize)); 4] = [
            (Vec::new(), (tiles.len()-1, 3)),
            (Vec::new(), (tiles.len()-1, 5)),
            (Vec::new(), (tiles.len()-1, 7)),
            (Vec::new(), (tiles.len()-1, 9)),
        ];

        let room_names: [char; 4] =  ['A', 'B', 'C', 'D'];

        for room in 0..rooms.len() {
            let mut i: usize = tiles.len() - 2;

            while i > 1 {
                let c: char = tiles[i][2*room+3];

                if c.is_alphabetic() {
                    rooms[room].1.0 -= 1;

                    if room_names[room] != c || rooms[room].0.len() > 0 {
                        rooms[room].0.push(c);
                    }
                }

                i -= 1;
            }
        }

        let mut state: State = State {
            total_cost: 0,
            base_cost: 0,
            tiles: tiles,
            hallway: hallway,
            rooms: rooms,
        };

        state.total_cost = state.lowball_total_cost();

        state
    }

    fn legal_moves(&self) -> Vec<State> {
        // Always put amphipod in assigned room if possible
        // There are 7 possible hallway spots
        // Some hallway configurations are unsolvable
        // E.g.
        // #############
        // #...D.A.....#
        // ###.#.#.#.###
        //   #.#.#.#.#
        //   #########
        // D and A are blocking each other
        // Since some rooms could still be solved,
        // This could lead to a lot of wasted explorations

        // Consequently, some room to hallway moves 
        // are pointless

        let mut moves: Vec<State> = Vec::new();

        for (&j, _) in &self.hallway {
            if let Some(to_room) = self.moves(1, j).0 {
                return vec![to_room]
            }
        }

        for room in 0..self.rooms.len() {
            let (occupants, (i, j)) = &self.rooms[room];

            if occupants.len() == 0 {
                continue;
            }

            let (to_room, hallway_moves) = self.moves(*i, *j);

            if to_room.is_some() {
                return vec![to_room.unwrap()];
            } else {
                for h_move in hallway_moves {
                    moves.push(h_move);
                }
            }

        }

        moves
    }

    fn moves(&self, i0: usize, j0: usize) -> (Option<State>, Vec<State>) {
        let c: char = self.tiles[i0][j0];
        let room: usize = match c {
            'A' => 0,
            'B' => 1,
            'C' => 2,
            'D' => 3,
            _ => 4,
        };

        let mut open: HashSet<usize> = HashSet::new();

        let mut hallway_moves: Vec<State> = Vec::new();

        let (i_final, j_final) = self.rooms[room].1;

        // If in hallway, can only move to designated room
        if i0 == 1 {
            if self.rooms[room].0.len() > 0 {
                return (None, hallway_moves)
            } else {
                let mut j: usize = j0;
                while j != j_final {
                    if j_final > j {
                        j += 1;
                    } else {
                        j -= 1;
                    }

                    if self.tiles[1][j] != '.' {
                        return (None, hallway_moves)
                    }
                }
                return (Some(self.move_amphipod(i0, j0, i_final-1, j_final)), hallway_moves)
            }
        }

        // Check hallway clearance
        let mut right: usize = j0 + 1;
        let mut left: usize = j0 - 1;

        while self.tiles[1][right] == '.' {
            open.insert(right);
            right += 1;
        }

        while self.tiles[1][left] == '.' {
            open.insert(left);
            left -= 1;
        }

        

        if open.contains(&j_final) && self.rooms[room].0.len() == 0 {
            return (Some(self.move_amphipod(i0, j0, i_final-1, j_final)), hallway_moves)
        }

        // Don't stop in front of room
        open.remove(&3);
        open.remove(&5);
        open.remove(&7);
        open.remove(&9);

        for j2 in open {
            hallway_moves.push(self.move_amphipod(i0, j0, 1, j2));
        }

        (None, hallway_moves)
    }

    fn move_amphipod(&self, i: usize, j: usize, i2: usize, j2: usize) -> Self {
        let mut state2: State = self.clone();

        let mut steps: usize = j2.abs_diff(j);
        if i2 == 1 {
            steps += i - i2;
        } else {
            steps += i + i2 - 2;
        }
        

        let c: char = self.tiles[i][j];

        let cost: usize = match c {
            'A' => 1,
            'B' => 10,
            'C' => 100,
            'D' => 1000,
            _ => 0,
        };

        state2.base_cost += steps*cost;

        state2.tiles[i][j] = '.';
        state2.tiles[i2][j2] = c;

        if i == 1 {
            state2.hallway.remove(&j);
        } else if i2 == 1 {
            state2.hallway.insert(j2, c);
        }

        if i > 1 {
            let room: usize = match j {
                3 => 0,
                5 => 1,
                7 => 2,
                9 => 3,
                _ => 4,
            };

            state2.rooms[room].0.pop();
            state2.rooms[room].1.0 += 1;
        }

        if i2 > 1 {
            let room: usize = match j2 {
                3 => 0,
                5 => 1,
                7 => 2,
                9 => 3,
                _ => 4,
            };

            state2.rooms[room].1.0 -= 1;
        }

        state2.total_cost = state2.lowball_total_cost();

        state2
    }

    fn is_hallway_blocked(&self) -> bool {

        let mut blockers: HashMap<usize, HashSet<usize>> = HashMap::new();

        let dests: HashMap<char, usize> = HashMap::from([
            ('A', 3),
            ('B', 5),
            ('C', 7),
            ('D', 9),
        ]);

        for (i, c) in self.tiles[1].iter().enumerate() {
            if c.is_alphabetic() {
                let jf: usize = dests[c];

                for j in i.min(jf)..jf.max(i)+1 {
                    if j == i {
                        continue;
                    }
                    if self.tiles[1][j].is_alphabetic() {
                        if let Some(blocks) = blockers.get_mut(&i) {
                            blocks.insert(j);
                        } else {
                            blockers.insert(i, HashSet::from([j]));
                        }

                        if let Some(blocks) = blockers.get_mut(&j) {
                            if blocks.contains(&i) {
                                return true
                            }
                        }
                    }
                }
            }
        }

        false
    }

    fn rearrange(&self) -> usize {
        let mut cheapest: usize = usize::MAX;

        let mut visited: HashMap<Vec<Vec<char>>, usize> = HashMap::new();

        let mut heap: BinaryHeap<State> = BinaryHeap::new();

        let start_state: State = self.clone();

        heap.push(start_state);

        while heap.len() > 0 {
            let state: State = heap.pop().unwrap();

            if state.total_cost == state.base_cost {
                cheapest = cheapest.min(state.total_cost);
                continue;
            } else if state.total_cost >= cheapest {
                continue;
            }

            let legal_moves: Vec<State> = state.legal_moves();

            for state2 in legal_moves {
                if state2.total_cost < *visited.get(&state2.tiles).unwrap_or(&usize::MAX) {
                    visited.insert(state2.tiles.clone(), state2.total_cost);
                    heap.push(state2);
                }

            }
        }

        cheapest
    }

    fn to_string(&self) -> String {
        let mut s: String = String::new();

        for row in &self.tiles {
            let row_string: String = row.iter().collect();
            s = format!("{s}{row_string}\n")
        }

        s
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        (other.total_cost, other.base_cost).cmp(&(self.total_cost, self.base_cost))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}