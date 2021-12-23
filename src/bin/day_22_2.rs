use std::fs;

use itertools::Itertools;
use regex::Regex;

type RangePair = (i64, i64);
type Cuboid = (RangePair, RangePair, RangePair);

fn cuboids_overlap(c1: Cuboid, c2: Cuboid) -> bool {
    let (xr1, yr1, zr1) = c1;
    let (xr2, yr2, zr2) = c2;

    // Greatest is less than lowest
    if xr2.1 - 1 < xr1.0 {
        return false;
    }

    // Lowest is greater than greatest
    if xr2.0 > xr1.1 - 1 {
        return false;
    }

    // Greatest is less than lowest
    if yr2.1 - 1 < yr1.0 {
        return false;
    }

    // Lowest is greater than greatest
    if yr2.0 > yr1.1 - 1 {
        return false;
    }

    // Greatest is less than lowest
    if zr2.1 - 1 < zr1.0 {
        return false;
    }

    // Lowest is greater than greatest
    if zr2.0 > zr1.1 - 1 {
        return false;
    }

    true
}

// c1, the base
// c2, subtract from c1 any overlap
fn subtract_cuboid(c1: Cuboid, c2: Cuboid) -> Vec<Cuboid> {
    if !cuboids_overlap(c1, c2) {
        return Vec::new();
    }

    let (xr1, yr1, zr1) = c1;
    let (xr2, yr2, zr2) = c2;

    let mut xs = [xr1.0, xr1.1, xr2.0, xr2.1];
    xs.sort_unstable();
    let mut ys = [yr1.0, yr1.1, yr2.0, yr2.1];
    ys.sort_unstable();
    let mut zs = [zr1.0, zr1.1, zr2.0, zr2.1];
    zs.sort_unstable();

    let mut result = Vec::new();

    for (x_start, x_end) in xs.iter().tuple_windows() {
        for (y_start, y_end) in ys.iter().tuple_windows() {
            for (z_start, z_end) in zs.iter().tuple_windows() {
                let cuboid = ((*x_start, *x_end), (*y_start, *y_end), (*z_start, *z_end));

                if cuboids_overlap(c1, cuboid) && !cuboids_overlap(c2, cuboid) {
                    result.push(cuboid);
                }
            }
        }
    }

    result
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

    let input: Vec<(bool, Cuboid)> = fs::read_to_string("inputs/day_22.txt")
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

    let mut on_cuboids: Vec<Cuboid> = Vec::new();

    input.iter().for_each(|(change, cuboid)| {
        println!("Processing: {}, {:?}", change, cuboid);
        if *change {
            if on_cuboids.is_empty() {
                on_cuboids.push(*cuboid);
                return;
            }

            let mut next_cuboids: Vec<Cuboid> = Vec::new();

            for c in on_cuboids.iter() {
                if cuboids_overlap(*c, *cuboid) {
                    let diff = subtract_cuboid(*c, *cuboid);
                    next_cuboids.extend(diff);
                } else {
                    next_cuboids.push(*c);
                }
            }

            next_cuboids.push(*cuboid);
            on_cuboids = next_cuboids;
        } else {
            let mut next_cuboids: Vec<Cuboid> = Vec::new();

            for c in on_cuboids.iter() {
                if cuboids_overlap(*c, *cuboid) {
                    let diff = subtract_cuboid(*c, *cuboid);
                    next_cuboids.extend(diff);
                } else {
                    next_cuboids.push(*c);
                }
            }

            on_cuboids = next_cuboids;
        }
    });

    println!("Num cuboids: {}", on_cuboids.len());

    let mut count = 0;
    for (xr, yr, zr) in on_cuboids {
        let x_min = xr.0;
        let x_max = xr.1;

        let y_min = yr.0;
        let y_max = yr.1;

        let z_min = zr.0;
        let z_max = zr.1;

        count += (x_max - x_min).abs() * (y_max - y_min).abs() * (z_max - z_min).abs();
    }

    println!("Cubes on: {}", count);
}
