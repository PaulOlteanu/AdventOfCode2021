use std::collections::HashSet;
use std::fs;

use itertools::Itertools;
use regex::Regex;

type Coordinate = (i64, i64, i64);

fn constrain(n: i64, min: i64, max: i64) -> i64 {
    if n < min {
        min
    } else if n > max {
        max
    } else {
        n
    }
}

fn get_range(tup: (i64, i64)) -> (i64, i64) {
    let (a, b) = tup;
    if a < b {
        (a, b)
    } else {
        (b, a)
    }
}

fn main() {
    let re = Regex::new(r"x=(-?\d+..-?\d+),y=(-?\d+..-?\d+),z=(-?\d+..-?\d+)").unwrap();

    let input = fs::read_to_string("inputs/day_22.txt")
        .unwrap()
        .trim()
        .lines()
        .map(|line| {
            let (change, ranges) = line.split(' ').collect_tuple().unwrap();
            let change = change == "on";
            let captures = re.captures(ranges).unwrap();
            let x: (i64, i64) = (&captures[1])
                .split("..")
                .map(|x| x.parse::<i64>().unwrap())
                .collect_tuple()
                .unwrap();
            let y: (i64, i64) = (&captures[2])
                .split("..")
                .map(|x| x.parse::<i64>().unwrap())
                .collect_tuple()
                .unwrap();
            let z: (i64, i64) = (&captures[3])
                .split("..")
                .map(|x| x.parse::<i64>().unwrap())
                .collect_tuple()
                .unwrap();

            let x = get_range(x);
            let y = get_range(y);
            let z = get_range(z);

            let x = (x.0, x.1 + 1);
            let y = (y.0, y.1 + 1);
            let z = (z.0, z.1 + 1);

            (change, (x, y, z))
        })
        .collect_vec();

    let mut cubes: HashSet<Coordinate> = HashSet::new();

    for (change, ranges) in input {
        println!("Processing: {}, {:?}", change, ranges);
        let (xr, yr, zr) = ranges;
        let x_min = constrain(xr.0, -50, 51);
        let x_max = constrain(xr.1, -50, 51);

        let y_min = constrain(yr.0, -50, 51);
        let y_max = constrain(yr.1, -50, 51);

        let z_min = constrain(zr.0, -50, 51);
        let z_max = constrain(zr.1, -50, 51);

        if change {
            for x in x_min..x_max {
                for y in y_min..y_max {
                    for z in z_min..z_max {
                        cubes.insert((x, y, z));
                    }
                }
            }
        } else {
            for x in x_min..x_max {
                for y in y_min..y_max {
                    for z in z_min..z_max {
                        cubes.remove(&(x, y, z));
                    }
                }
            }
        }
    }

    println!("{:?}", cubes.len());
    // for cube in cubes.iter().sorted() {
    //     println!("{:?}", cube);
    // }
}
