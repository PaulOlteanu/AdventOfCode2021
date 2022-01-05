use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

use itertools::Itertools;

use nalgebra::SMatrix;
use nalgebra::Vector3;

#[derive(Debug, Clone)]
struct Scanner {
    beacons: Vec<Vector3<i32>>,
}

impl Scanner {
    fn new() -> Self {
        Self {
            beacons: Vec::new(),
        }
    }
}

type Matrix3x3 = SMatrix<i32, 3, 3>;

fn rotation_matrices() -> Vec<Matrix3x3> {
    let opts = vec![
        Vector3::new(1, 0, 0),
        Vector3::new(-1, 0, 0),
        Vector3::new(0, 1, 0),
        Vector3::new(0, -1, 0),
        Vector3::new(0, 0, 1),
        Vector3::new(0, 0, -1),
    ];

    let mut result = Vec::new();
    for i in opts.iter() {
        for j in opts.iter() {
            for k in opts.iter() {
                if i.cross(j) == *k {
                    let temp = Matrix3x3::from_columns(&[*i, *j, *k]);

                    result.push(temp);
                }
            }
        }
    }

    result
}

fn compare_scanners(
    scanner1: &Scanner,
    scanner2: &Scanner,
    rotations: &[Matrix3x3],
) -> Option<(Matrix3x3, Vector3<i32>)> {
    for rotation in rotations {
        // Distance to count
        let mut similar: HashMap<(i32, i32, i32), u32> = HashMap::new();

        for beacon1 in scanner1.beacons.iter() {
            for beacon2 in scanner2.beacons.iter() {
                let rotated = rotation * beacon2;

                let diff = (
                    rotated.x - beacon1.x,
                    rotated.y - beacon1.y,
                    rotated.z - beacon1.z,
                );

                *similar.entry(diff).or_insert(0) += 1;
            }
        }

        let (offset, similar) = similar.iter().max_by_key(|(_, &v)| v).unwrap();

        if *similar >= 12 {
            let offset = Vector3::new(offset.0, offset.1, offset.2);
            return Some((*rotation, offset));
        }
    }

    None
}

fn main() {
    let input = fs::read_to_string("inputs/day_19.txt").unwrap();
    let mut input = input.trim().lines();

    let mut normalized_scanners: Vec<Scanner> = Vec::new();
    let mut unnormalized_scanners: Vec<Scanner> = Vec::new();

    let mut working_scanner: Scanner;

    while input.next().is_some() {
        working_scanner = Scanner::new();
        for line in input.by_ref() {
            let line = line.trim();
            if line.is_empty() {
                break;
            }

            let coord: (i32, i32, i32) = line
                .split(',')
                .map(|x| x.parse::<i32>().unwrap())
                .collect_tuple()
                .unwrap();

            let beacon = Vector3::new(coord.0, coord.1, coord.2);
            working_scanner.beacons.push(beacon);
        }

        if normalized_scanners.is_empty() {
            normalized_scanners.push(working_scanner)
        } else {
            unnormalized_scanners.push(working_scanner);
        }
    }

    let rotations = rotation_matrices();

    while !unnormalized_scanners.is_empty() {
        for i in 0..normalized_scanners.len() {
            let norm_scanner = normalized_scanners[i].clone();
            for j in 0..unnormalized_scanners.len() {
                let unnorm_scanner = &unnormalized_scanners[j];

                if let Some((rotation, offset)) =
                    compare_scanners(&norm_scanner, unnorm_scanner, &rotations)
                {
                    let mut new_scanner = Scanner::new();

                    for beacon in unnorm_scanner.beacons.iter() {
                        let rotated = rotation * beacon;
                        let normalized = rotated - offset;
                        new_scanner.beacons.push(normalized);
                    }

                    normalized_scanners.push(new_scanner);
                    unnormalized_scanners.remove(j);
                    break;
                }
            }
        }
    }

    let mut beacons: HashSet<(i32, i32, i32)> = HashSet::new();
    for scanner in normalized_scanners {
        for beacon in scanner.beacons {
            beacons.insert((beacon.x, beacon.y, beacon.z));
        }
    }

    println!("{}", beacons.len());
}
