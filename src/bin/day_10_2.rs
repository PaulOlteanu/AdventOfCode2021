use std::fs;

use itertools::Itertools;

fn get_score(c: char) -> u64 {
    match c {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
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
    let mut input: Vec<Vec<char>> = fs::read_to_string("inputs/day_10.txt")
        .unwrap()
        .trim()
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    input.retain(|line| {
        let mut stack: Vec<char> = Vec::new();

        for c in line {
            if ['(', '[', '{', '<'].contains(c) {
                stack.push(*c);
            } else {
                let opener = stack.pop();
                if let Some(opener) = opener {
                    if !opener_matches(*c, opener) {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        }

        true
    });

    let scores = input
        .iter()
        .map(|line| {
            let mut stack: Vec<char> = Vec::new();

            for c in line {
                if ['(', '[', '{', '<'].contains(c) {
                    stack.push(*c);
                } else {
                    stack.pop();
                }
            }

            let mut score = 0;
            for c in stack.iter().rev() {
                score *= 5;
                score += get_score(*c);
            }

            score
        })
        .sorted()
        .collect_vec();

    let winner = scores[scores.len() / 2];

    println!("{:?}", winner);
}
