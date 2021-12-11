use std::fs;

use itertools::Itertools;

fn get_score(c: char) -> u32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

fn opener_matches(c: char, opener: char) -> bool {
    (opener == '(' && c == ')')
        || (opener == '[' && c == ']')
        || (opener == '{' && c == '}')
        || (opener == '<' && c == '>')
}

fn main() {
    let input: Vec<Vec<char>> = fs::read_to_string("inputs/day_10.txt")
        .unwrap()
        .trim()
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let result: u32 = input
        .iter()
        .map(|line| {
            let mut stack: Vec<char> = Vec::new();
            let mut score = 0;

            for c in line {
                if ['(', '[', '{', '<'].contains(c) {
                    stack.push(*c);
                } else {
                    let opener = stack.pop();
                    if let Some(opener) = opener {
                        if !opener_matches(*c, opener) {
                            score = get_score(*c);
                            break;
                        }
                    } else {
                        score = get_score(*c);
                        break;
                    }
                }
            }

            score
        })
        .sum();

    println!("{}", result);
}
