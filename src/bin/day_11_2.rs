use std::fs;

use itertools::Itertools;

fn main() {
    let mut octopi: Vec<Vec<i32>> = fs::read_to_string("inputs/day_11.txt")
        .unwrap()
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|x| x.to_digit(10).unwrap() as i32)
                .collect_vec()
        })
        .collect_vec();

    let mut step = 0;

    loop {
        step += 1;
        let mut full_done = false;
        let mut flashed = 0;

        for row in octopi.iter_mut() {
            for col in row.iter_mut() {
                *col += 1;
            }
        }

        loop {
            for row in 0..10 {
                for col in 0..10 {
                    if octopi[row][col] > 9 {
                        flashed += 1;

                        octopi[row][col] = -1;
                        if row != 0 && col != 0 && octopi[row - 1][col - 1] != -1 {
                            octopi[row - 1][col - 1] += 1
                        }
                        if row != 0 && octopi[row - 1][col] != -1 {
                            octopi[row - 1][col] += 1
                        }
                        if row != 0 && col != 9 && octopi[row - 1][col + 1] != -1 {
                            octopi[row - 1][col + 1] += 1
                        }

                        if col != 0 && octopi[row][col - 1] != -1 {
                            octopi[row][col - 1] += 1
                        }
                        if col != 9 && octopi[row][col + 1] != -1 {
                            octopi[row][col + 1] += 1
                        }

                        if row != 9 && col != 0 && octopi[row + 1][col - 1] != -1 {
                            octopi[row + 1][col - 1] += 1
                        }
                        if row != 9 && octopi[row + 1][col] != -1 {
                            octopi[row + 1][col] += 1
                        }
                        if row != 9 && col != 9 && octopi[row + 1][col + 1] != -1 {
                            octopi[row + 1][col + 1] += 1
                        }
                    }
                }
            }

            let mut step_done = true;
            for row in octopi.iter() {
                for col in row.iter() {
                    if *col > 9 {
                        step_done = false;
                        break;
                    }
                }
            }

            if step_done {
                for row in octopi.iter_mut() {
                    for col in row.iter_mut() {
                        if *col == -1 {
                            *col = 0;
                        }
                    }
                }

                if flashed == 100 {
                    full_done = true;
                }

                break;
            }
        }

        if full_done {
            break;
        }
    }

    println!("{}", step);
}
