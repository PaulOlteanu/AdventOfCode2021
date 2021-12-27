use std::fs;

use itertools::Itertools;

fn main() {
    let input: Vec<Vec<u32>> = fs::read_to_string("inputs/day_09.txt")
        .unwrap()
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|digit| digit.to_digit(10).unwrap())
                .collect_vec()
        })
        .collect_vec();

    let mut low_point_risks = 0;

    for i in 0..input.len() {
        for j in 0..input[i].len() {
            if j != 0 && input[i][j] >= input[i][j - 1] {
                continue;
            }
            if j != input[i].len() - 1 && input[i][j] >= input[i][j + 1] {
                continue;
            }
            if i != 0 && input[i][j] >= input[i - 1][j] {
                continue;
            }
            if i != input.len() - 1 && input[i][j] >= input[i + 1][j] {
                continue;
            }
            low_point_risks += input[i][j] + 1;
        }
    }

    println!("{:?}", low_point_risks);
}
