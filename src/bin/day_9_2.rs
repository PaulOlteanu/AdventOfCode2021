use std::fs;

use itertools::Itertools;

fn find_basin_size(points: &[Vec<u32>], visited: &mut Vec<Vec<bool>>, x: usize, y: usize) -> u32 {
    if points[y][x] == 9 {
        return 0;
    }
    if visited[y][x] {
        return 0;
    }

    visited[y][x] = true;

    1 + if x == 0 {
        0
    } else {
        find_basin_size(points, visited, x - 1, y)
    } + if x == points[0].len() - 1 {
        0
    } else {
        find_basin_size(points, visited, x + 1, y)
    } + if y == 0 {
        0
    } else {
        find_basin_size(points, visited, x, y - 1)
    } + if y == points.len() - 1 {
        0
    } else {
        find_basin_size(points, visited, x, y + 1)
    }
}

fn main() {
    let input: Vec<Vec<u32>> = fs::read_to_string("inputs/day_9.txt")
        .unwrap()
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|digit| digit.to_digit(10).unwrap())
                .collect_vec()
        })
        .collect_vec();

    let mut visited: Vec<Vec<bool>>;
    visited = (0..input.len())
        .map(|_| vec![false; input[0].len()])
        .collect_vec();

    let mut basin_sizes: Vec<u32> = Vec::new();

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
            basin_sizes.push(find_basin_size(&input, &mut visited, j, i))
        }
    }

    basin_sizes.sort_unstable();
    let result: u32 = basin_sizes.iter().rev().take(3).product();

    println!("{:?}", result);
}
