use std::collections::HashMap;
use std::fs;

use itertools::Itertools;

#[derive(PartialEq, Eq, Hash, Debug)]
struct Coordinate {
    x: u32,
    y: u32,
}

fn main() {
    let mut points: HashMap<Coordinate, u32> = HashMap::new();

    let input: Vec<(Coordinate, Coordinate)> = fs::read_to_string("inputs/day_05.txt")
        .unwrap()
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|pair| {
                    let (x, y) = pair
                        .split(',')
                        .map(|x| x.parse::<u32>().unwrap())
                        .next_tuple()
                        .unwrap();
                    Coordinate { x, y }
                })
                .next_tuple()
                .unwrap()
        })
        .collect();

    input.iter().for_each(|(start, end)| {
        if start.x == end.x {
            // Loop over y
            let (start_y, end_y) = if start.y > end.y {
                (end.y, start.y)
            } else {
                (start.y, end.y)
            };
            for y in start_y..=end_y {
                *points.entry(Coordinate { x: start.x, y }).or_insert(0) += 1;
            }
        } else if start.y == end.y {
            // Loop over x
            let (start_x, end_x) = if start.x > end.x {
                (end.x, start.x)
            } else {
                (start.x, end.x)
            };
            for x in start_x..=end_x {
                *points.entry(Coordinate { x, y: start.y }).or_insert(0) += 1;
            }
        }
    });

    let danger_points = points.values().filter(|&x| *x >= 2).count();

    println!("{:#?}", danger_points);
}
