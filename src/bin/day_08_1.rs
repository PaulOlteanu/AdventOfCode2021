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
        .map(|(_, output)| {
            output
                .iter()
                .map(|digit| {
                    if digit.len() == 2 || digit.len() == 4 || digit.len() == 3 || digit.len() == 7
                    {
                        1
                    } else {
                        0
                    }
                })
                .sum::<i32>()
        })
        .sum::<i32>();

    println!("{:#?}", answer);
}
