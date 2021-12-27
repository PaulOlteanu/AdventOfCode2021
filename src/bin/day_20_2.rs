use std::fs;

use itertools::Itertools;

// This should be a lot better
fn get_surrounding_on(image: &[Vec<bool>], row: usize, col: usize, grid_val: bool) -> usize {
    let mut result = Vec::new();
    // -1, -1
    if row == 0 || col == 0 {
        result.push(grid_val);
    } else {
        result.push(image[row - 1][col - 1]);
    }
    // -1, 0
    if row == 0 {
        result.push(grid_val);
    } else {
        result.push(image[row - 1][col]);
    }
    // -1, +1
    if row == 0 || col == image[0].len() - 1 {
        result.push(grid_val);
    } else {
        result.push(image[row - 1][col + 1]);
    }

    // 0, -1
    if col == 0 {
        result.push(grid_val);
    } else {
        result.push(image[row][col - 1]);
    }
    // 0, 0
    result.push(image[row][col]);
    // 0, +1
    if col == image[0].len() - 1 {
        result.push(grid_val);
    } else {
        result.push(image[row][col + 1]);
    }

    // +1, -1
    if row == image.len() - 1 || col == 0 {
        result.push(grid_val);
    } else {
        result.push(image[row + 1][col - 1]);
    }
    // +1, 0
    if row == image.len() - 1 {
        result.push(grid_val);
    } else {
        result.push(image[row + 1][col]);
    }
    // +1, +1
    if row == image.len() - 1 || col == image[0].len() - 1 {
        result.push(grid_val);
    } else {
        result.push(image[row + 1][col + 1]);
    }

    let temp = result.iter().map(|x| if *x { '1' } else { '0' }).join("");
    usize::from_str_radix(&temp, 2).unwrap()
}

fn main() {
    let input = fs::read_to_string("inputs/day_20.txt").unwrap();
    let mut input = input.trim().lines();

    let algo = input
        .next()
        .unwrap()
        .trim()
        .chars()
        .map(|c| c == '#')
        .collect_vec();
    input.next();

    let mut image = input
        .map(|line| line.chars().map(|c| c == '#').collect_vec())
        .collect_vec();

    let mut prev_grid_val = false;
    let mut grid_val = false;

    for _ in 0..50 {
        let mut new_image = vec![vec![prev_grid_val; image[0].len() + 2]; image.len() + 2];

        let mut ref_image = vec![vec![grid_val; image[0].len() + 2]; image.len() + 2];
        for i in 0..image.len() {
            for j in 0..image[0].len() {
                ref_image[i + 1][j + 1] = image[i][j];
            }
        }

        for next_row in 0..new_image.len() {
            for next_col in 0..new_image[0].len() {
                let num_on = get_surrounding_on(&ref_image, next_row, next_col, grid_val);
                new_image[next_row][next_col] = algo[num_on];
            }
        }

        image = new_image;
        prev_grid_val = grid_val;
        if !grid_val {
            grid_val = algo[0];
        } else {
            grid_val = algo[511];
        }
    }

    let mut on_count = 0;
    for row in image {
        for col in row {
            if col {
                on_count += 1
            }
        }
    }

    println!("{}", on_count);
}
