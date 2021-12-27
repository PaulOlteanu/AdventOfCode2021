use std::collections::HashSet;
use std::fs;

use itertools::Itertools;

fn main() {
    let input: Vec<(Vec<String>, Vec<String>)> = fs::read_to_string("inputs/day_08.txt")
        .unwrap()
        .trim()
        .lines()
        .map(|line| {
            line.split('|')
                .map(|input| {
                    input
                        .trim()
                        .split(' ')
                        .map(|x| x.trim().to_owned())
                        .collect_vec()
                })
                .collect_tuple()
                .unwrap()
        })
        .collect_vec();

    let answer: i32 = input
        .iter()
        .map(|(input, output)| {
            let mut patterns: [HashSet<char>; 10] = Default::default();
            let output_patterns = output
                .iter()
                .map(|digit| digit.chars().collect::<HashSet<char>>())
                .collect_vec();

            while patterns.iter().any(|x| x.is_empty()) {
                for pattern in input {
                    let p: HashSet<char> = pattern.chars().collect();

                    if pattern.len() == 2 {
                        patterns[1] = pattern.chars().collect();
                    } else if pattern.len() == 3 {
                        patterns[7] = pattern.chars().collect();
                    } else if pattern.len() == 4 {
                        patterns[4] = pattern.chars().collect();
                    } else if pattern.len() == 5 {
                        let diff: HashSet<_> = patterns[4].difference(&patterns[1]).collect();
                        let diff: HashSet<char> = diff.iter().map(|&c| *c).collect();

                        if !patterns[1].is_empty() && p.is_superset(&patterns[1]) {
                            patterns[3] = pattern.chars().collect();
                        } else if !patterns[1].is_empty()
                            && !patterns[4].is_empty()
                            && p.is_superset(&diff)
                        {
                            patterns[5] = pattern.chars().collect();
                        } else if !patterns[1].is_empty() && !patterns[4].is_empty() {
                            patterns[2] = pattern.chars().collect();
                        }
                    } else if pattern.len() == 6 {
                        if !patterns[4].is_empty() && p.is_superset(&patterns[4]) {
                            patterns[9] = pattern.chars().collect();
                        } else if !patterns[5].is_empty() && p.is_superset(&patterns[5]) {
                            patterns[6] = pattern.chars().collect();
                        } else if !patterns[4].is_empty() && !patterns[5].is_empty() {
                            patterns[0] = pattern.chars().collect();
                        }
                    } else if pattern.len() == 7 {
                        patterns[8] = pattern.chars().collect();
                    }
                }
            }

            patterns
                .iter()
                .position(|x| *x == output_patterns[0])
                .unwrap() as i32
                * 1000
                + patterns
                    .iter()
                    .position(|x| *x == output_patterns[1])
                    .unwrap() as i32
                    * 100
                + patterns
                    .iter()
                    .position(|x| *x == output_patterns[2])
                    .unwrap() as i32
                    * 10
                + patterns
                    .iter()
                    .position(|x| *x == output_patterns[3])
                    .unwrap() as i32
        })
        .sum();

    println!("{:#?}", answer);
}
