use std::fs;

fn main() {
    let input =
        fs::read_to_string("inputs/day_1.1.txt")
        .unwrap();

    let measurements = input.lines();

    let result=
        measurements
        .map(|s| s.trim().parse::<u32>().unwrap())
        .collect::<Vec<_>>()
        .windows(3)
        .map(|x| {
            if let [a, b, c] = x {
                a + b + c
            } else {
                0
            }
        })
        .collect::<Vec<_>>()
        .windows(2)
        .filter(|x| {
            if let [a, b] = x {
                b > a
            } else {
                false
            }
        })
        .count();

    println!("{:?}", result);
}
