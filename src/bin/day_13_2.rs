// use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

use itertools::Itertools;

// Fold right to left
fn fold_x(paper: &mut HashSet<(u32, u32)>, line: u32, dimensions: (u32, u32)) -> (u32, u32) {
    let (max_x, max_y) = dimensions;

    for y in 0..=max_y {
        for x in line..=max_x {
            if paper.contains(&(x, y)) {
                paper.insert((line * 2 - x, y));
                paper.remove(&(x, y));
            }
        }
    }

    (line, max_y)
}

// Fold bottom to top
fn fold_y(paper: &mut HashSet<(u32, u32)>, line: u32, dimensions: (u32, u32)) -> (u32, u32) {
    let (max_x, max_y) = dimensions;

    for y in line..=max_y {
        for x in 0..=max_x {
            if paper.contains(&(x, y)) {
                paper.insert((x, line * 2 - y));
                paper.remove(&(x, y));
            }
        }
    }

    (max_x, line)
}

fn main() {
    let input = fs::read_to_string("inputs/day_13.txt").unwrap();
    let input_lines = input.trim().lines();

    let mut max_x = 0;
    let mut max_y = 0;
    let mut paper: HashSet<(u32, u32)> = HashSet::new();

    input_lines
        .clone()
        .take_while(|line| !line.is_empty())
        .for_each(|line| {
            let (x, y) = line
                .trim()
                .split(',')
                .map(|x| x.parse::<u32>().unwrap())
                .collect_tuple()
                .unwrap();
            if x > max_x {
                max_x = x
            };
            if y > max_y {
                max_y = y
            };
            paper.insert((x, y));
        });

    input_lines
        .skip_while(|line| !line.is_empty())
        .for_each(|line| {
            if line.is_empty() {
                return;
            }

            let (direction, line) = line
                .trim_start_matches("fold along ")
                .split('=')
                .collect_tuple()
                .unwrap();
            let line = line.parse::<u32>().unwrap();
            if direction == "x" {
                let new_dims = fold_x(&mut paper, line, (max_x, max_y));
                max_x = new_dims.0;
                max_y = new_dims.1;
            } else {
                let new_dims = fold_y(&mut paper, line, (max_x, max_y));
                max_x = new_dims.0;
                max_y = new_dims.1;
            }
        });

    for y in 0..=max_y {
        for x in 0..=max_x {
            if paper.contains(&(x, y)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }

    println!("{:?}", paper.len());
}
