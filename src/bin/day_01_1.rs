use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/day_01.txt").unwrap();

    let result = input
        .lines()
        .map(|s| s.trim().parse::<u32>().unwrap())
        .collect::<Vec<_>>()
        .windows(2)
        .flat_map(<[u32; 2]>::try_from)
        .filter(|[a, b]| b > a)
        .count();

    println!("{:?}", result);
}
