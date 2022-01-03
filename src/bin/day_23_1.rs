#![allow(clippy::needless_range_loop)]

use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};

use priority_queue::PriorityQueue;

// ##########################
// ## 0 1 2 3 4 5 6 7 8 910##
// ######11##12##13##14######
//     ##15##16##17##18##
//     ##################

// (A, A, B, B, C, C, D, D)
type State = (usize, usize, usize, usize, usize, usize, usize, usize);

fn generate_state(map: &[char]) -> State {
    let mut as_ = Vec::new();
    let mut bs = Vec::new();
    let mut cs = Vec::new();
    let mut ds = Vec::new();

    for i in 0..19 {
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

    (as_[0], as_[1], bs[0], bs[1], cs[0], cs[1], ds[0], ds[1])
}

fn hall_start(index: usize) -> usize {
    match index {
        11 => 2,
        15 => 2,
        12 => 4,
        16 => 4,
        13 => 6,
        17 => 6,
        14 => 8,
        18 => 8,
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
        'A' => index == 15 || map[15] == 'A' && index == 11,
        'B' => index == 16 || map[16] == 'B' && index == 12,
        'C' => index == 17 || map[17] == 'C' && index == 13,
        'D' => index == 18 || map[18] == 'D' && index == 14,
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
                    if path_clear(map, i, 15) {
                        moves.push((i, 15));
                    } else if map[15] == 'A' && path_clear(map, i, 11) {
                        moves.push((i, 11));
                    }
                }
                'B' => {
                    if path_clear(map, i, 16) {
                        moves.push((i, 16));
                    } else if map[16] == 'B' && path_clear(map, i, 12) {
                        moves.push((i, 12));
                    }
                }
                'C' => {
                    if path_clear(map, i, 17) {
                        moves.push((i, 17));
                    } else if map[17] == 'C' && path_clear(map, i, 13) {
                        moves.push((i, 13));
                    }
                }
                'D' => {
                    if map[18] == '.' && path_clear(map, i, 18) {
                        moves.push((i, 18));
                    } else if map[18] == 'D' && path_clear(map, i, 14) {
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
        12 => (from as i32 - 4i32).abs() as u32 + 1,
        16 => (from as i32 - 4i32).abs() as u32 + 2,
        13 => (from as i32 - 6i32).abs() as u32 + 1,
        17 => (from as i32 - 6i32).abs() as u32 + 2,
        14 => (from as i32 - 8i32).abs() as u32 + 1,
        18 => (from as i32 - 8i32).abs() as u32 + 2,
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
    let mut map = vec!['.'; 19];
    map[state.0] = 'A';
    map[state.1] = 'A';
    map[state.2] = 'B';
    map[state.3] = 'B';
    map[state.4] = 'C';
    map[state.5] = 'C';
    map[state.6] = 'D';
    map[state.7] = 'D';
    map
}

fn solve(map: &[char]) -> u32 {
    let mut visited: HashSet<State> = HashSet::new();
    let mut costs: HashMap<State, u32> = HashMap::new();

    let state = generate_state(map);
    costs.insert(state, 0);

    let mut current = state;

    let mut queue = PriorityQueue::new();
    queue.push(current, Reverse(0));

    loop {
        visited.insert(current);
        if current == (11, 15, 12, 16, 13, 17, 14, 18) {
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

    *costs.get(&(11, 15, 12, 16, 13, 17, 14, 18)).unwrap()
}

fn main() {
    #[rustfmt::skip]
    let map = vec![
        '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                  'B',      'B',      'D',      'D',
                  'C',      'C',      'A',      'A',
    ];

    let cost = solve(&map);

    println!("{:?}", cost);
}
