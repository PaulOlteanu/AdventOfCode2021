use std::fs;

use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("inputs/day_25.txt")
        .unwrap()
        .trim()
        .lines()
        .map(|line| line.trim().chars().collect_vec())
        .collect_vec();

    let mut current = input;

    let mut done = false;
    let mut step = 0;
    while !done {
        done = true;
        step += 1;

        for i in current.iter() {
            for j in i {
                print!("{}", j);
            }
            println!();
        }

        println!("On step {}", step);

        let mut next = vec![vec!['.'; current[0].len()]; current.len()];
        for i in 0..current.len() {
            for j in 0..current[0].len() {
                match current[i][j] {
                    '>' => {
                        if j == current[0].len() - 1 && current[i][0] == '.' {
                            done = false;
                            next[i][0] = '>';
                        } else if j != current[0].len() - 1 && current[i][j + 1] == '.' {
                            done = false;
                            next[i][j + 1] = '>';
                        } else {
                            next[i][j] = '>';
                        }
                    }
                    'v' => next[i][j] = 'v',
                    _ => continue,
                }
            }
        }

        current = next;
        let mut next = vec![vec!['.'; current[0].len()]; current.len()];
        for i in 0..current.len() {
            for j in 0..current[0].len() {
                match current[i][j] {
                    '>' => next[i][j] = '>',
                    'v' => {
                        if i == current.len() - 1 && current[0][j] == '.' {
                            done = false;
                            next[0][j] = 'v';
                        } else if i != current.len() - 1 && current[i + 1][j] == '.' {
                            done = false;
                            next[i + 1][j] = 'v';
                        } else {
                            next[i][j] = 'v';
                        }
                    }
                    _ => continue,
                }
            }
        }

        current = next;
    }

    println!("Stopped after: {} steps", step);
}
