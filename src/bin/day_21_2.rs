use std::collections::HashMap;

const VICTORY: u64 = 21;

type GameState = (u64, u64, u64, u64, bool);

// true -> p1's turn, false -> p2's turn
// (p1_wins, p2_wins)
fn outcome(
    cache: &mut HashMap<GameState, (u64, u64)>,
    p1_pos: u64,
    p1_score: u64,
    p2_pos: u64,
    p2_score: u64,
    p1_turn: bool,
    level: usize,
) -> (u64, u64) {
    let cache_key = (p1_pos, p1_score, p2_pos, p2_score, p1_turn);
    if cache.contains_key(&cache_key) {
        return *cache.get(&cache_key).unwrap();
    }

    let mut p1_ret = 0;
    let mut p2_ret = 0;

    if p1_turn {
        for i in 1..=3 {
            for j in 1..=3 {
                for k in 1..=3 {
                    let roll = i + j + k;
                    let p1_pos = 1 + (p1_pos + roll - 1) % 10;
                    let p1_score = p1_score + p1_pos;

                    if p1_score >= VICTORY {
                        p1_ret += 1;
                        continue;
                    }

                    let (p1, p2) =
                        outcome(cache, p1_pos, p1_score, p2_pos, p2_score, false, level + 1);
                    p1_ret += p1;
                    p2_ret += p2;
                }
            }
        }
    } else {
        for i in 1..=3 {
            for j in 1..=3 {
                for k in 1..=3 {
                    let roll = i + j + k;
                    let p2_pos = 1 + (p2_pos + roll - 1) % 10;
                    let p2_score = p2_score + p2_pos;

                    if p2_score >= VICTORY {
                        p2_ret += 1;
                        continue;
                    }

                    let (p1, p2) =
                        outcome(cache, p1_pos, p1_score, p2_pos, p2_score, true, level + 1);
                    p1_ret += p1;
                    p2_ret += p2;
                }
            }
        }
    }

    cache.insert(cache_key, (p1_ret, p2_ret));
    (p1_ret, p2_ret)
}

fn main() {
    // (p1_pos, p1_score, p2_pos, p2_score, p1_turn)
    let mut cache: HashMap<GameState, (u64, u64)> = HashMap::new();

    let result = outcome(&mut cache, 5, 0, 9, 0, true, 0);
    println!("{:?}", result);
}
