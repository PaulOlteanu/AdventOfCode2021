use std::collections::HashMap;
use std::fs;
use std::iter;

use itertools::Itertools;

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
struct Coordinate {
    x: u32,
    y: u32,
}

impl Coordinate {
    fn generate_line(start: &Coordinate, end: &Coordinate) -> Vec<Coordinate> {
        let (start, end) = if start.x < end.x {
            (start, end)
        } else {
            (end, start)
        };
        let (xs, ys) = if start.x == end.x {
            let ys: Vec<u32> = if start.y < end.y {
                (start.y..=end.y).collect()
            } else {
                (end.y..=start.y).rev().collect()
            };
            let xs: Vec<u32> = (iter::repeat(start.x).take(ys.len())).collect();
            (xs, ys)
        } else if start.y == end.y {
            let xs: Vec<u32> = (start.x..=end.x).collect();
            let ys: Vec<u32> = (iter::repeat(start.y).take(xs.len())).collect();
            (xs, ys)
        } else if ((end.y as i32 - start.y as i32) / (end.x as i32 - start.x as i32)).abs() == 1 {
            let xs: Vec<u32> = (start.x..=end.x).collect();
            let ys: Vec<u32> = if start.y < end.y {
                (start.y..=end.y).collect()
            } else {
                (end.y..=start.y).rev().collect()
            };

            (xs, ys)
        } else {
            (Vec::<u32>::new(), Vec::<u32>::new())
        };

        xs.iter()
            .zip(ys.iter())
            .map(|(&x, &y)| Coordinate { x, y })
            .collect()
    }
}

fn main() {
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

    let mut points: HashMap<Coordinate, u32> = HashMap::new();
    input.iter().for_each(|(start, end)| {
        let line_points = Coordinate::generate_line(start, end);
        for point in line_points {
            *points.entry(point).or_insert(0) += 1;
        }
    });

    let danger_points = points.values().filter(|&x| *x >= 2).count();

    println!("{:#?}", danger_points);
}
