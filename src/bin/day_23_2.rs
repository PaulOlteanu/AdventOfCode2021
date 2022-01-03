#![allow(clippy::needless_range_loop)]

use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};

use priority_queue::PriorityQueue;

// ##########################
// ## 0 1 2 3 4 5 6 7 8 910##
// ######11##12##13##14######
//     ##15##16##17##18##
//     ##19##20##21##22##
//     ##23##24##25##26##
//     ##################

// (A, A, A, A, B, B, B, B, C, C, C, C, D, D, D, D)
// type State = (usize, usize, usize, usize, usize, usize, usize, usize);
type State = [usize; 16];

fn generate_state(map: &[char]) -> State {
    let mut as_ = Vec::new();
    let mut bs = Vec::new();
    let mut cs = Vec::new();
    let mut ds = Vec::new();

    for i in 0..27 {
        match map[i] {
            'A' => as_.push(i),
            'B' => bs.push(i),
            'C' => cs.push(i),
            'D' => ds.push(i),
            _ => {}
        }
    }

    as_.sort_unstable();
    bs.sort_unstable();
    cs.sort_unstable();
    ds.sort_unstable();

    [as_, bs, cs, ds].concat().try_into().unwrap()
}

fn hall_start(index: usize) -> usize {
    match index {
        11 => 2,
        15 => 2,
        19 => 2,
        23 => 2,

        12 => 4,
        16 => 4,
        20 => 4,
        24 => 4,

        13 => 6,
        17 => 6,
        21 => 6,
        25 => 6,

        14 => 8,
        18 => 8,
        22 => 8,
        26 => 8,
        _ => unreachable!(),
    }
}

fn room_start(index: usize) -> usize {
    match index {
        2 => 11,
        4 => 12,
        6 => 13,
        8 => 14,
        _ => unreachable!(),
    }
}

fn path_clear(map: &[char], from: usize, to: usize) -> bool {
    let (f, t) = if from > to { (to, from) } else { (from, to) };

    let hall_target = hall_start(t);

    let (f1, t1) = if f > hall_target {
        (hall_target, f)
    } else {
        (f, hall_target)
    };

    for i in f1..=t1 {
        if i == from {
            continue;
        }

        if map[i] != '.' {
            return false;
        }
    }

    let mut r = room_start(hall_target);
    if r != from && map[r] != '.' {
        return false;
    }

    while r != t {
        r += 4;
        if r != from && map[r] != '.' {
            return false;
        }
    }

    true
}

fn is_in_correct_room(map: &[char], index: usize) -> bool {
    match map[index] {
        'A' => {
            index == 23
                || map[23] == 'A' && index == 19
                || map[23] == 'A' && map[19] == 'A' && index == 15
                || map[23] == 'A' && map[19] == 'A' && map[15] == 'A' && index == 11
        }
        'B' => {
            index == 24
                || map[24] == 'B' && index == 20
                || map[24] == 'B' && map[20] == 'B' && index == 16
                || map[24] == 'B' && map[20] == 'B' && map[16] == 'B' && index == 12
        }
        'C' => {
            index == 25
                || map[25] == 'C' && index == 21
                || map[25] == 'C' && map[21] == 'C' && index == 17
                || map[25] == 'C' && map[21] == 'C' && map[17] == 'C' && index == 13
        }
        'D' => {
            index == 26
                || map[26] == 'D' && index == 22
                || map[26] == 'D' && map[22] == 'D' && index == 18
                || map[26] == 'D' && map[22] == 'D' && map[18] == 'D' && index == 14
        }
        _ => unreachable!(),
    }
}

fn generate_moves(map: &[char]) -> Vec<(usize, usize)> {
    let mut moves = Vec::new();

    for i in 0..map.len() {
        if map[i] == '.' {
            continue;
        }

        if i <= 10 {
            // In the hallway

            match map[i] {
                'A' => {
                    if path_clear(map, i, 23) {
                        moves.push((i, 23));
                    } else if map[23] == 'A' && path_clear(map, i, 19) {
                        moves.push((i, 19));
                    } else if map[23] == 'A' && map[19] == 'A' && path_clear(map, i, 15) {
                        moves.push((i, 15));
                    } else if map[23] == 'A'
                        && map[19] == 'A'
                        && map[15] == 'A'
                        && path_clear(map, i, 11)
                    {
                        moves.push((i, 11));
                    }
                }
                'B' => {
                    if path_clear(map, i, 24) {
                        moves.push((i, 24));
                    } else if map[24] == 'B' && path_clear(map, i, 20) {
                        moves.push((i, 20));
                    } else if map[24] == 'B' && map[20] == 'B' && path_clear(map, i, 16) {
                        moves.push((i, 16));
                    } else if map[24] == 'B'
                        && map[20] == 'B'
                        && map[16] == 'B'
                        && path_clear(map, i, 12)
                    {
                        moves.push((i, 12));
                    }
                }
                'C' => {
                    if path_clear(map, i, 25) {
                        moves.push((i, 25));
                    } else if map[25] == 'C' && path_clear(map, i, 21) {
                        moves.push((i, 21));
                    } else if map[25] == 'C' && map[21] == 'C' && path_clear(map, i, 17) {
                        moves.push((i, 17));
                    } else if map[25] == 'C'
                        && map[21] == 'C'
                        && map[17] == 'C'
                        && path_clear(map, i, 13)
                    {
                        moves.push((i, 13));
                    }
                }
                'D' => {
                    if path_clear(map, i, 26) {
                        moves.push((i, 26));
                    } else if map[26] == 'D' && path_clear(map, i, 22) {
                        moves.push((i, 22));
                    } else if map[26] == 'D' && map[22] == 'D' && path_clear(map, i, 18) {
                        moves.push((i, 18));
                    } else if map[26] == 'D'
                        && map[22] == 'D'
                        && map[18] == 'D'
                        && path_clear(map, i, 14)
                    {
                        moves.push((i, 14));
                    }
                }
                _ => unreachable!(),
            }
        } else {
            // In a room

            // Done moving
            if is_in_correct_room(map, i) {
                continue;
            }

            // Blocked by above
            if i > 14 && map[i - 4] != '.' {
                continue;
            }

            for j in 0..=10 {
                if j == 2 || j == 4 || j == 6 || j == 8 {
                    continue;
                }

                if path_clear(map, i, j) {
                    moves.push((i, j))
                }
            }
        }
    }

    moves
}

fn get_steps(from: usize, to: usize) -> u32 {
    let (from, to) = if from > to { (to, from) } else { (from, to) };

    match to {
        11 => (from as i32 - 2i32).abs() as u32 + 1,
        15 => (from as i32 - 2i32).abs() as u32 + 2,
        19 => (from as i32 - 2i32).abs() as u32 + 3,
        23 => (from as i32 - 2i32).abs() as u32 + 4,

        12 => (from as i32 - 4i32).abs() as u32 + 1,
        16 => (from as i32 - 4i32).abs() as u32 + 2,
        20 => (from as i32 - 4i32).abs() as u32 + 3,
        24 => (from as i32 - 4i32).abs() as u32 + 4,

        13 => (from as i32 - 6i32).abs() as u32 + 1,
        17 => (from as i32 - 6i32).abs() as u32 + 2,
        21 => (from as i32 - 6i32).abs() as u32 + 3,
        25 => (from as i32 - 6i32).abs() as u32 + 4,

        14 => (from as i32 - 8i32).abs() as u32 + 1,
        18 => (from as i32 - 8i32).abs() as u32 + 2,
        22 => (from as i32 - 8i32).abs() as u32 + 3,
        26 => (from as i32 - 8i32).abs() as u32 + 4,
        _ => 0,
    }
}

fn get_cost_multiplier(fish_type: char) -> u32 {
    match fish_type {
        'A' => 1,
        'B' => 10,
        'C' => 100,
        'D' => 1000,
        _ => 0,
    }
}

fn do_move_get_state(map: &[char], from: usize, to: usize) -> State {
    let mut new_map = map.to_owned();
    new_map[to] = new_map[from];
    new_map[from] = '.';

    generate_state(&new_map)
}

fn state_to_map(state: State) -> Vec<char> {
    let mut map = vec!['.'; 27];
    for (i, c) in ('A'..='D').enumerate() {
        for j in 0..4 {
            map[state[i * 4 + j]] = c;
        }
    }

    map
}

const FIN: [usize; 16] = [
    11, 15, 19, 23, 12, 16, 20, 24, 13, 17, 21, 25, 14, 18, 22, 26,
];

fn solve(map: &[char]) -> u32 {
    let mut visited: HashSet<State> = HashSet::new();
    let mut costs: HashMap<State, u32> = HashMap::new();

    let state = generate_state(map);
    costs.insert(state, 0);

    let mut current = state;

    // let mut queue = HashMap::new();
    let mut queue = PriorityQueue::new();
    queue.push(current, Reverse(0));

    loop {
        visited.insert(current);
        if current == FIN {
            break;
        }

        let current_map = state_to_map(current);

        let mut moves = generate_moves(&current_map);

        for &(from, to) in moves.iter() {
            let move_cost = get_steps(from, to) * get_cost_multiplier(current_map[from]);

            let next_state = do_move_get_state(&current_map, from, to);

            let entry = costs.get(&next_state);
            let next_cost = match entry {
                Some(c) => *c,
                None => u32::MAX,
            };

            let current_cost = *costs.get(&current).unwrap();

            if current_cost + move_cost < next_cost {
                costs.insert(next_state, current_cost + move_cost);
            }
        }

        moves.retain(|&(from, to)| {
            let new_state = do_move_get_state(&current_map, from, to);
            !visited.contains(&new_state)
        });

        for (from, to) in moves {
            let next_state = do_move_get_state(&current_map, from, to);
            queue.push(next_state, Reverse(*costs.get(&next_state).unwrap()));
        }

        if let Some(next) = queue.pop() {
            current = next.0;
        } else {
            break;
        }
    }

    *costs.get(&FIN).unwrap()
}

fn main() {
    #[rustfmt::skip]
    let map = vec![
        '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  'B',      'B',      'D',      'D',
                  'D',      'C',      'B',      'A',
                  'D',      'B',      'A',      'C',
                  'C',      'C',      'A',      'A',
    ];

    let cost = solve(&map);

    println!("{:?}", cost);
}
